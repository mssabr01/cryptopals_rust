use std::collections::HashMap;
use crate::convert_text::xor_with_key;

//Measure the average distance from the correct frequence each letter is
//the smaller the score the more likely this text is english
fn score_text(text_freqs: HashMap<char, f32>) -> f32 {
    let english_char_freqs: HashMap<char, f32> = HashMap::from([
        ('a', 8.34),
        ('b', 1.54),
        ('c', 2.73),
        ('d', 4.14),
        ('e', 12.6),
        ('f', 2.03),
        ('g', 1.92),
        ('h', 6.11),
        ('i', 6.71),
        ('j', 0.23),
        ('k', 0.87),
        ('l', 4.24),
        ('m', 2.53),
        ('n', 6.80),
        ('o', 7.70),
        ('p', 1.66),
        ('q', 0.09),
        ('r', 5.68),
        ('s', 6.11),
        ('t', 9.37),
        ('u', 2.85),
        ('v', 1.06),
        ('w', 2.34),
        ('x', 0.20),
        ('y', 2.04),
        ('z', 0.06),
    ]);

    //for each letter, calculate the distance from the correct frequency
    let mut total_distance: f32 = 0.0;
    for (letter, freq) in text_freqs.iter() {
        let test_letter = *letter;
        let en_freq = english_char_freqs[&test_letter];
        let mut x: f32 = freq - en_freq;
        x = x * 1000.0; //working with raw floats gives me different answers every time I run this, idk why
        x = x.round();  //so I'm only grabbing up to 7 significant digits
        total_distance += x.abs();
    }
    total_distance / 26.0
}

//creates a hashmap with each alphabet letter as keys
//values initialized to zero
fn init_text_hash() -> HashMap<char, usize> {
    let mut char_counts: HashMap<char, usize> = HashMap::new();
    char_counts.insert('a', 0);
    char_counts.insert('b', 0);
    char_counts.insert('c', 0);
    char_counts.insert('d', 0);
    char_counts.insert('e', 0);
    char_counts.insert('f', 0);
    char_counts.insert('g', 0);
    char_counts.insert('h', 0);
    char_counts.insert('i', 0);
    char_counts.insert('j', 0);
    char_counts.insert('k', 0);
    char_counts.insert('l', 0);
    char_counts.insert('m', 0);
    char_counts.insert('n', 0);
    char_counts.insert('o', 0);
    char_counts.insert('p', 0);
    char_counts.insert('q', 0);
    char_counts.insert('r', 0);
    char_counts.insert('s', 0);
    char_counts.insert('t', 0);
    char_counts.insert('u', 0);
    char_counts.insert('v', 0);
    char_counts.insert('w', 0);
    char_counts.insert('x', 0);
    char_counts.insert('y', 0);
    char_counts.insert('z', 0);
    char_counts
}

// Removes all special chars and makes it lowercase
fn clean_string(text: &str) -> String {
    //lowercase
    let wtext = &text.to_ascii_lowercase();

    //remove special characters
    //find how many alpha characters there are and init a vector
    let mut alpha_count = 0;
    for elem in wtext.chars() {
        //lowercase alpha chars are 97-122
        if elem as u8 >= 97 && elem as u8 <= 122 {
            alpha_count += 1;
        }
    }

    let mut result: String = String::with_capacity(alpha_count);
    for elem in wtext.chars() {
        //lowercase alpha chars are 97-122
        if elem as u8 >= 97 && elem as u8 <= 122 {
            result.push(elem as u8 as char);
        }
    }

    result
}

// Takes in a string of english text
// removes all special characters and returns a hashmap of character frequences for
// each alphabet character in the text
fn count_chars(text: &str) -> HashMap<char, f32> {
    //init HashMap
    let mut char_counts = init_text_hash();
    let mut char_freqs: HashMap<char, f32> = HashMap::new();
    //remove all non-alpha characters from the text
    let clean_str: String = clean_string(text);

    for elem in clean_str.chars() {
        char_counts.insert(elem, char_counts[&elem] + 1);
    }

    for (letter, count) in char_counts.iter() {
        if text.len() == 0 {
            char_freqs.insert(*letter, 0.0);
        }
        else{
            char_freqs.insert(*letter, (*count as f32 / clean_str.len() as f32)* 100.0);
        }
    }

    char_freqs
}

//Attempts to guess the key to an xor cipher
// tries every potential u8 key and returns the one that scores the best
fn guess_u8_xor_key(text: &str) -> u8{

    let mut best_performing_key: u8 = 0;
    let mut best_performance:f32 = 1000000.0;
    let byte_array: Vec<u8> = hex::decode(text).unwrap();

    for test_key in 0..256 as u32{
        
        let xord_text:Vec<u8> = xor_with_key(&byte_array, &vec![test_key as u8]);
        let char_freqs: HashMap<char, f32> = count_chars(&String::from_utf8_lossy(&xord_text).to_string());
        let score: f32 = score_text(char_freqs);

        if score < best_performance{
            best_performance = score;
            best_performing_key = test_key as u8;
        }
    }

    best_performing_key
}

#[cfg(test)]
pub mod challenge_3 {
    //https://cryptopals.com/sets/1/challenges/3
    //Single-byte XOR cipher
    //The hex encoded string:

    //1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736
    //... has been XOR'd against a single character. Find the key, decrypt the message.

    //You can do this by hand. But don't: write code to do it for you.

    //How? Devise some method for "scoring" a piece of English plaintext. Character frequency is a good metric.
    //Evaluate each output and choose the one with the best score.
    use super::*;

    #[test]
    fn guess_key(){
        assert_eq!(guess_u8_xor_key("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736"), 88 as u8);
    }

    #[test]
    fn challenge_3(){
        let key: u8 = guess_u8_xor_key("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");
        let byte_array: Vec<u8> = hex::decode("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736").unwrap();
        let xord_text:Vec<u8> = xor_with_key(&byte_array, &vec![key]);
        assert_eq!( String::from_utf8_lossy(&xord_text).to_string(), "Cooking MC's like a pound of bacon");
    }

    #[test]
    fn test_clean_string() {
        assert_eq!(clean_string("123QWE"), "qwe");
        assert_eq!(clean_string("QWE"), "qwe");
        assert_eq!(clean_string("$!$(!#@"), "");
        assert_eq!(clean_string("qwe"), "qwe");
    }

    #[test]
    fn test_count_chars() {
        assert_eq!(count_chars(""), HashMap::from([('t', 0.0), ('b', 0.0), ('u', 0.0), ('n', 0.0), ('v', 0.0), ('c', 0.0), ('r', 0.0), ('x', 0.0), ('o', 0.0), ('y', 0.0), ('i', 0.0), ('a', 0.0), ('k', 0.0), ('e', 0.0), ('h', 0.0), ('z', 0.0), ('w', 0.0), ('q', 0.0), ('d', 0.0), ('p', 0.0), ('j', 0.0), ('s', 0.0), ('m', 0.0), ('f', 0.0), ('l', 0.0), ('g', 0.0)]));
        assert_eq!(count_chars("aaa").get(&'a').unwrap(), &(100.0));
        assert_eq!(count_chars("tbtbu").get(&'t').unwrap(), &(40.0));
        assert_eq!(count_chars("tbtbu").get(&'b').unwrap(), &(40.0));
        assert_eq!(count_chars("tbtbu").get(&'u').unwrap(), &(20.0));
        assert_eq!(count_chars("tbtbu").get(&'a').unwrap(), &(0.0));
    }

    #[test]
    fn test_score_text() {
        assert_eq!(score_text(count_chars("")) as u32, 3844);     //score text returns a float but I'm getting different answers on different runs
        assert_eq!(score_text(count_chars("tbtbu")) as u32, 6631);  // so I'm only testing against the most significant digits
    }
}
