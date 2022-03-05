use std::io;

fn input() -> u64 {
    let mut _input_value = String::new();

    io::stdin().read_line(&mut _input_value)
        .expect("Failed to read line");
    
    loop {
        let _input_value: u64 = match _input_value.trim().parse() {
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

fn fibonacci(n: u64) -> u64 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    }
    fibonacci(n - 1) + fibonacci(n - 2)
}

fn main() {
    println!("Please input n.");
    let n: u64 = input();

    let fibonacci_number: u64 = fibonacci(n);
    println!("{}-th fibonacci number is {}.", n, fibonacci_number);
}
