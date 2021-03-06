#![recursion_limit = "512"]

mod test_utils;

use test_utils::*;

asn_to_rust!(
    r"BasicBitString DEFINITIONS AUTOMATIC TAGS ::=
    BEGIN
    
    Unconstrained ::= SEQUENCE {
        abc UTF8String
    }
    
    BasicConstrained ::= SEQUENCE {
        abc UTF8String (SIZE(8))
    }
    
    BasicConstrainedSmall ::= SEQUENCE {
        abc UTF8String (SIZE(4..6))
    }
    
    BasicConstrainedExtensible ::= SEQUENCE {
        abc UTF8String (SIZE(4..6,...))
    } 
    
    END"
);

#[test]
fn test_unconstrained() {
    // from playground
    serialize_and_deserialize_uper(
        8 * 14,
        &[
            0x0D, 0x75, 0x6E, 0x63, 0x6F, 0x6E, 0x73, 0x74, 0x72, 0x61, 0x69, 0x6E, 0x65, 0x64,
        ],
        &Unconstrained {
            abc: "unconstrained".to_string(),
        },
    );
}

#[test]
fn test_fixed_size() {
    // from playground
    serialize_and_deserialize_uper(
        8 * 9,
        &[0x08, 0x65, 0x78, 0x61, 0x63, 0x74, 0x6C, 0x79, 0x38],
        &BasicConstrained {
            abc: "exactly8".to_string(),
        },
    );
}

#[test]
#[should_panic(expected = "SizeNotInRange(8, 4, 6)")]
fn test_too_large() {
    // from playground
    serialize_and_deserialize_uper(
        0,
        &[],
        &BasicConstrainedSmall {
            abc: "exactly8".to_string(),
        },
    );
}

#[test]
fn test_small_min() {
    // from playground
    serialize_and_deserialize_uper(
        8 * 5,
        &[0x04, 0x66, 0x6F, 0x75, 0x72],
        &BasicConstrainedSmall {
            abc: "four".to_string(),
        },
    );
}

#[test]
fn test_small_max() {
    // from playground
    serialize_and_deserialize_uper(
        8 * 7,
        &[0x06, 0x73, 0x2D, 0x69, 0x2D, 0x78, 0x21],
        &BasicConstrainedSmall {
            abc: "s-i-x!".to_string(),
        },
    );
}

#[test]
fn test_extensible_small() {
    // from playground
    serialize_and_deserialize_uper(
        8 * 5,
        &[0x04, 0x66, 0x6F, 0x75, 0x72],
        &BasicConstrainedExtensible {
            abc: "four".to_string(),
        },
    );
}

#[test]
fn test_extensible_extended() {
    // from playground
    serialize_and_deserialize_uper(
        8 * 8,
        &[0x07, 0x73, 0x65, 0x76, 0x65, 0x6E, 0x21, 0x21],
        &BasicConstrainedExtensible {
            abc: "seven!!".to_string(),
        },
    );
}
