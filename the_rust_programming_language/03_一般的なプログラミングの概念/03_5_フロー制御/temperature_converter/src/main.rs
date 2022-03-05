use std::io;

fn input() -> f64 {
    let mut _input_value = String::new();

    io::stdin().read_line(&mut _input_value)
        .expect("Failed to read line");
    
    loop {
        let _input_value: f64 = match _input_value.trim().parse() {
            Ok(num) => {
                return num;
            },
            Err(_) => {
                println!("Hey, input a number!");
                continue;
            },
        };
    }
}

fn convert_fahrenheit_to_celcius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) / 1.8
}

fn convert_celcius_to_fahrenheit(celcius: f64) -> f64 {
    1.8 * celcius + 32.0
}

fn main() {
    println!("Please input fahrenheit temperature.");
    let mut _input_temperature: f64 = input();
    let output_temperature: f64 = convert_fahrenheit_to_celcius(_input_temperature);
    println!("{}[°F] is {} [°C].", _input_temperature, output_temperature);

    println!("Please input celcius temperature.");
    let mut _input_temperature: f64 = input();
    let output_temperature: f64 = convert_celcius_to_fahrenheit(_input_temperature);
    println!("{}[°C] is {} [°F].", _input_temperature, output_temperature);
}
