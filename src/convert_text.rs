//https://cryptopals.com/sets/1/challenges/1
//Convert hex to base64
//The string:
//
//49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d
//Should produce:
//
//SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t
//So go ahead and make that happen. You'll need to use this code for the rest of the exercises.
//
//Cryptopals Rule
//Always operate on raw bytes, never on encoded strings. Only use hex and base64 for pretty-printing.

use base64;
use hex;

fn hex_to_base64(hex_str: &str) -> Result<String, hex::FromHexError>
{
    let byte_array: Vec<u8> = hex::decode(hex_str)?;
    Ok(base64::encode(byte_array))
}

#[cfg(test)]
pub mod challenge_1 {
    use super::*;
    
    #[test]
    fn answer()
    {
        assert_eq!(hex_to_base64("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d").unwrap(),
        "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t");
    }

    #[test]
    fn empty_hex_str()
    {
        assert_eq!(hex_to_base64("").unwrap(), "");
    }

    #[test]
    fn bad_hex_char()
    {
        assert_eq!(hex_to_base64("ta"), Err(hex::FromHexError::InvalidHexCharacter{c:'t', index:0}));
    }

    #[test]
    fn odd_str()
    {
        assert_eq!(hex_to_base64("a12"), Err(hex::FromHexError::OddLength));
    }
}
