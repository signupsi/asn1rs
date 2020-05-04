use asn1rs::prelude::*;

asn_to_rust!(
    r#"BasicSchema DEFINITIONS AUTOMATIC TAGS ::=
BEGIN

  Pizza ::= SEQUENCE {
    price INTEGER,
    size INTEGER(1..4),
    note Utf8String OPTIONAL
  }

  Topping ::= ENUMERATED {
    not-pineapple,
    even-less-pineapple,
    no-pineapple-at-all
  }

  Custom ::= Utf8String 

  WhatToEat ::= CHOICE {
    pizza Pizza,
    custom Custom
  }
  
END"#
);

/// This module contains the content which is generated by the macro call above
mod what_is_being_generated {
    use asn1rs::prelude::*;

    #[asn(sequence)]
    #[derive(Default, Debug, Clone, PartialEq, Hash)]
    pub struct Pizza {
        #[asn(integer(min..max))]
        pub price: u64,
        #[asn(integer(1..4))]
        pub size: u8,
        #[asn(option(utf8string))]
        pub note: Option<String>,
    }

    #[asn(enumerated)]
    #[derive(Debug, Clone, PartialEq, Hash, Copy, PartialOrd, Eq)]
    pub enum Topping {
        NotPineapple,
        EvenLessPineapple,
        NoPineappleAtAll,
    }

    #[asn(transparent)]
    #[derive(Default, Debug, Clone, PartialEq, Hash)]
    pub struct Custom(#[asn(utf8string)] pub String);

    #[asn(choice)]
    #[derive(Debug, Clone, PartialEq, Hash)]
    pub enum WhatToEat {
        #[asn(complex(Pizza))]
        Pizza(Pizza),
        #[asn(complex(Custom))]
        Custom(Custom),
    }
}

#[test]
fn uper_proof() {
    use asn1rs::syn::io::UperWriter;
    let mut writer = UperWriter::default();
    writer
        .write(&WhatToEat::Pizza(Pizza {
            price: 2,
            size: 3,
            note: Some(String::from("Extra crusty!")),
        }))
        .unwrap();

    // read into the plain type to prove they behave the same
    use what_is_being_generated as g;

    let mut reader = writer.into_reader();
    let read = reader.read::<g::WhatToEat>().expect("Failed to read");

    assert_eq!(
        read,
        g::WhatToEat::Pizza(g::Pizza {
            price: 2,
            size: 3,
            note: Some(String::from("Extra crusty!"))
        })
    );
}
