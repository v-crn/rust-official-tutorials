fn generate_lyric_sentence(n: usize) -> String {
    let mut lyrics = "".to_string();

    let lyric_sentences = [
        "Twelve drummers drumming",
        "Eleven pipers piping",
        "Ten lords a-leaping",
        "Nine ladies dancing",
        "Eight maids a-milking",
        "Seven swans a-swimming",
        "Six geese a-laying",
        "Five golden rings",
        "Four calling birds",
        "Three French hens",
        "Two turtle doves and",
        "A partridge in a pear tree",
    ];
    let start_idx = lyric_sentences.len() - n - 1;
    for lyric_sentence in lyric_sentences[start_idx..].iter() {
        lyrics += lyric_sentence;
        lyrics += "\n";
    }
    return lyrics;
}

fn main() {
    let numeral_list = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "nineth",
        "tenth", "eleventh", "twelfth",
    ];
    let mut lyrics: String = "".to_owned();

    for (i, v) in numeral_list.iter().enumerate() {
        lyrics += &format!("On the {} day of Christmas\nMy true love sent to me:\n", v).to_string();

        lyrics += &generate_lyric_sentence(i).to_string();

        if i < numeral_list.len() - 1 {
            lyrics += "\n";
        }
    }
    println!("{}", lyrics);
}
