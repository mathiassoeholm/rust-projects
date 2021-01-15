// Lyrics: https://genius.com/Christmas-songs-the-twelve-days-of-christmas-lyrics
fn main() {
    let gifts = [
        "A partridge in a pear tree",
        "Two turtle doves",
        "Three french hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];

    for i in 1..gifts.len() {
        println!("On the first day of Christmas, my true love sent to me");
        for j in (0..i).rev() {
            if j == 1 {
                println!("{}, and", gifts[j]);
            } else {
                println!("{}", gifts[j]);
            }
        }
        println!("");
    }
}
