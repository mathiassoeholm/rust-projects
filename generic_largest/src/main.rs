fn main() {
    let number_list = vec![32, 54, 12, 34, 77];
    println!("The largest number is {}", largest(&number_list));
    let char_list = vec!['a', 'g', 'y', 'e'];
    println!("The largest char is {}", largest(&char_list));
    let string_list = vec!["monkey", "dog", "zebra", "gorilla"];
    println!("The largest string is {}", largest(&string_list));
}

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
