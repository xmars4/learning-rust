fn is_consonant(ch: char) -> bool {
    let lowercase_ch = ch.to_ascii_lowercase();
    lowercase_ch.is_alphabetic() && !matches!(lowercase_ch, 'a' | 'e' | 'i' | 'u' | 'o')
}

pub fn convert_text_to_pig_latin(text: &str) -> String {
    let mut latin_words = String::from("");
    let words: Vec<&str> = text.split_whitespace().collect();
    for word in words {
        let first_char = word.chars().nth(0);
        let first_char_value = match first_char {
            Some(c) => c,
            None => continue,
        };
        if is_consonant(first_char_value) {
            match word.get(1..) {
                Some(r) => latin_words.push_str(r),
                _ => (),
            }
            latin_words.push('-');
            latin_words.push(first_char_value);
            latin_words.push_str("ay");
        } else {
            latin_words.push_str(word);
            latin_words.push('-');
            latin_words.push_str("hay");
        }
        latin_words.push(' ');
    }
    latin_words
}
