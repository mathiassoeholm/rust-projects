use std::collections::HashMap;

fn main() {
    let mut list: Vec<i32> = vec![1, 4, 5, 6, 7, 3, 1];

    list.sort();

    let sum = list.iter().fold(0, |acc, x| acc + x);
    let mean = sum / list.len() as i32;
    let median = list[list.len() / 2];

    let mut occurences = HashMap::new();
    let mut top = (-1, 0);

    for num in list.iter() {
        let count = occurences.entry(num).or_insert(0);
        *count += 1;

        if *count > top.1 {
            top = (*num, *count);
        }
    }

    let mode = top.0;

    println!("mean: {}, median {}, mode {}", mean, median, mode)
}
