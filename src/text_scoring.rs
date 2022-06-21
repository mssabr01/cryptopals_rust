use std::collections::HashMap;

const english_char_freqs: HashMap<char, f32> = [
('a', 8.34f),
('b', 1.54f),
('c', 2.73f),
('d', 4.14f),
('e', 12.6f),
('f', 2.03f),
('g', 1.92f),
('h', 6.11f),
('i', 6.71f),
('j', 0.23f),
('k', 0.87f),
('l', 4.24f),
('m', 2.53f),
('n', 6.80f),
('o', 7.70f),
('p', 1.66f),
('q', 0.09f),
('r', 5.68f),
('s', 6.11f),
('t', 9.37f),
('u', 2.85f),
('v', 1.06f),
('w', 2.34f),
('x', 0.20f),
('y', 2.04f),
('z', 0.06f)
]

 

//creates a hashmap with each alphabet letter as keys
//values initialized to zero
fn init_text_histogram() -> HashMap<char,usize>
{
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
fn clean_string(text: &str) -> String{

    //lowercase
    let wtext = &text.to_ascii_lowercase();

    //remove special characters
    //find how many alpha characters there are and init a vector
    let mut alpha_count = 0;
    for elem in wtext.chars()
    {
        //lowercase alpha chars are 97-122
        if elem as u8 >= 97 && elem as u8 <= 122
        {
            alpha_count += 1;
        }
    }

    let mut result: String = String::with_capacity(alpha_count);
    for elem in wtext.chars(){
        //lowercase alpha chars are 97-122
        if elem as u8 >= 97 && elem as u8 <= 122
        {
            result.push(elem);
        }
    }
    
    result
    
}

// Takes in a string of english text
// removes all special characters and returns a histogram of 
// each alphabet character in the text
fn text_histogram(text: &str) -> HashMap<char,usize>
{
    //init HashMap
    let mut char_counts = init_text_histogram();
    //remove all non-alpha characters from the text
    let clean_str: String = clean_string(text);

    for elem in clean_str.chars() {
        char_counts.insert(elem, char_counts[&elem] + 1);
    }

    char_counts
}


#[cfg(test)]
pub mod challenge_3 {
    //Single-byte XOR cipher
    //The hex encoded string:

    //1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736
    //... has been XOR'd against a single character. Find the key, decrypt the message.

    //You can do this by hand. But don't: write code to do it for you.

    //How? Devise some method for "scoring" a piece of English plaintext. Character frequency is a good metric. 
    //Evaluate each output and choose the one with the best score.
    use super::*;
    
    #[test]
    fn test_clean_string()
    {
        assert_eq!(clean_string("123QWE"), "qwe");
        assert_eq!(clean_string("QWE"), "qwe");
        assert_eq!(clean_string("$!$(!#@"), "");
        assert_eq!(clean_string("qwe"), "qwe");
    }

    #[test]
    fn test_text_histogram()
    {
        assert_eq!(text_histogram(""), init_text_histogram());
        let mut test_hist = init_text_histogram();
        test_hist.insert('a', 3);
        assert_eq!(text_histogram("aaa"), test_hist);
        test_hist.insert('g', 7);
        assert_eq!(text_histogram("!@#aAGGGa$!  Ggg%^$%$#^g"), test_hist);
    }

    
}