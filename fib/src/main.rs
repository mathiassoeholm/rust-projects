use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];
    let n: i32 = input.parse().unwrap();

    let mut prev: i32 = 0;
    let mut current: i32 = 0;
    for i in 0..n {
        let temp = current;
        current = if i == 1 { 1 } else {prev + current};
        prev = temp;
        println!("{}: {}", i, current)
    }
}

