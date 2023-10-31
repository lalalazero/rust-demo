fn main() {
    let s = String::from("first apple");
    println!("{s}");
    let s = pig_latin(s);
    println!("{s}");
}

fn pig_latin(s: String) -> String {
    let mut result = String::from("");

    for word in s.split(" ") {
        result.push_str(&deal_word(word.to_string()));
        result.push(' ');
    }
    result
}

fn deal_word(word: String) -> String {
    let mut new_word = String::from("");
    let mut flag = false;

    for c in word.chars() {
        flag = is_vowel(c);
        break;
    }

    if flag {
        new_word.push_str(&word);
        new_word.push_str("-hay");
    } else {
        new_word.push_str(&word[1..]);
        new_word.push_str("-");
        new_word.push_str(&word[0..1]);
        new_word.push_str("ay")
    }
    new_word
}

fn is_vowel(c: char) -> bool {
    let s = "aeiouAEIOU";

    for ch in s.chars() {
        if ch == c {
            return true;
        }
    }
    false
}
