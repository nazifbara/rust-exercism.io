use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    let mut reversed_word = String::from("");

    for char in input.graphemes(true) {
        reversed_word = String::from(char.to_string() + reversed_word.as_str())
    }
    reversed_word
}
