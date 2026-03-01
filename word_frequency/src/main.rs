// Juan Castro csci-3334
// Word Frequency Counter

// 1. Takes a string of text as input
// 2. Splits the text into words (space as separator) // text.split_whitespace().collect();
// 3. Counts the frequency of each word
// 4. Returns the word with the highest frequency and its count

fn most_frequent_word(text: &str) -> (String, usize) {
    //TODO

    (max_word, max_count) // return tuple
}

fn main() {
    let text = "the quick brown fox jumps over the lazy dog the quick brown fox";
    let (word, count) = most_frequent_word(text);
    println!("Most frequent word: \"{}\" ({} times)", word, count);
}