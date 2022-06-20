use base64;
use hex;

//Converts a hex string to base64
fn hex_to_base64(hex_str: &str) -> Result<String, hex::FromHexError>
{
    let byte_array: Vec<u8> = hex::decode(hex_str)?;
    Ok(base64::encode(byte_array))
}

//XORs two byte arrays
fn xor(byte_array1: Vec<u8>, byte_array2: Vec<u8>) -> Vec<u8>
{
    let mut result: Vec<u8> = vec![0; byte_array1.len()];
    let mut i: usize = 0;
    while i < byte_array1.len()
    {
        result[i] = byte_array1[i] ^ byte_array2[i];
        i += 1;
    }
    result
}

//XORs two hex strings
fn xor_hex(hex_str1: &str, hex_str2: &str) -> Result<String, hex::FromHexError>
{
    if hex_str1.len() != hex_str2.len() {
         return Err(hex::FromHexError::InvalidStringLength);
    }
    let byte_array1: Vec<u8> = hex::decode(hex_str1)?;
    let byte_array2: Vec<u8> = hex::decode(hex_str2)?;
    Ok(hex::encode(xor(byte_array1, byte_array2)))
}

#[cfg(test)]
pub mod challenge_1 {
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

#[cfg(test)]
pub mod challenge_2 {
    //https://cryptopals.com/sets/1/challenges/2
    //Fixed XOR
    //Write a function that takes two equal-length buffers and produces their XOR combination.
    //
    //If your function works properly, then when you feed it the string:
    //
    //1c0111001f010100061a024b53535009181c
    //... after hex decoding, and when XOR'd against:
    //
    //686974207468652062756c6c277320657965
    //... should produce:
    //
    //746865206b696420646f6e277420706c6179
    use super::*;

    #[test]
    fn answer()
    {
        assert_eq!(xor_hex("1c0111001f010100061a024b53535009181c", "686974207468652062756c6c277320657965").unwrap(),
        "746865206b696420646f6e277420706c6179");
    }

    #[test]
    fn empty_hex_str()
    {
        assert_eq!(xor_hex("","").unwrap(), "");
    }

    #[test]
    fn bad_hex_char()
    {
        assert_eq!(xor_hex("ta","sg"), Err(hex::FromHexError::InvalidHexCharacter{c:'t', index:0}));
    }

    #[test]
    fn odd_str()
    {
        assert_eq!(xor_hex("a12","b13"), Err(hex::FromHexError::OddLength));
    }

    #[test]
    fn unequal_vectors1()
    {
        assert_eq!(xor_hex("a12ab1","b13c"), Err(hex::FromHexError::InvalidStringLength));
    }

    #[test]
    fn unequal_vectors2()
    {
        assert_eq!(xor_hex("a12a","b13cb1"), Err(hex::FromHexError::InvalidStringLength));
    }
}
