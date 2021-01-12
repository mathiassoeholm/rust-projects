use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];
    let temperature: f64 = input.parse().unwrap();
    let converted_to_celcius = (temperature - 32.0) * (5.0 / 9.0);
    let converted_to_fahrenheit = (temperature * 1.8) + 32.0;

    println!("{}°F is {}°C", temperature, converted_to_celcius);
    println!("{}°C is {}°F", temperature, converted_to_fahrenheit);
}