// Juan Castro csci-3334
// Word Frequency Counter

// 1. Takes a string of text as input
// 2. Splits the text into words (space as separator) // text.split_whitespace().collect();
// 3. Counts the frequency of each word
// 4. Returns the word with the highest frequency and its count

fn most_frequent_word(text: &str) -> (String, usize) {
    let list: Vec<&str> = text.split_whitespace().collect();

    // setting the base definitions in this scope
    let mut max_word = "";
    let mut max_count = 0;

    // get the first word in the list
    // check against every other word to inc the count or not
    for i in 0..list.len() {
        let mut count = 0;

        for j in 0..list.len() {
            if list[i] == list[j]{
                count += 1;
            }
        }

        // after a word is checked, then place that word and its count
        // into the max values IF greater than previous words
        if count > max_count {
            max_count = count;
            max_word = list[i];
        }
    }

    (max_word.to_string(), max_count) // return tuple
}

fn main() {
    let text = "the quick brown fox jumps over the lazy dog the quick brown fox";
    let (word, count) = most_frequent_word(text);
    println!("Most frequent word: \"{}\" ({} times)", word, count);
}