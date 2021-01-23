fn main() {
    println!(
        "{}",
        to_pig_latin("The Rust Programming Language is the official book on Rust")
    );
}

fn to_pig_latin(text: &str) -> String {
    let consonants = [
        'b', 'c', 'd', 'f', 'g', 'h', 'j', 'k', 'l', 'm', 'n', 'p', 'q', 'r', 's', 't', 'v', 'w',
        'x', 'z',
    ];

    let mut translated_words = Vec::new();

    for word in text.split_whitespace() {
        if consonants
            .iter()
            .any(|&l| word.to_lowercase().starts_with(l))
        {
            translated_words.push(format!("{}-{}ay", &word[1..], &word.to_lowercase()[0..1]));
        } else {
            translated_words.push(format!("{}-hay", word));
        }
    }

    return translated_words.join(" ");
}
