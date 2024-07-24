use std::io;

fn fahrenheit_to_celcius_helper(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

fn celcius_to_fahrenheit_helper(c: f64) -> f64 {
    c * 9.0 / 5.0 + 32.0
}

fn temperature_converter() {
    'processing: loop {
        println!("\"Celcius\", \"Fahrenheit\" or \"Quit\": ");
        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("Failed to read line");

        let input = input.trim();

        match input {
            "Celcius" => {
                println!("Enter Celcius: ");
                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Failed to read line");
                let input: f64 = input.trim().parse().expect("Please type a number!");

                println!("Fahrenheit: {}", celcius_to_fahrenheit_helper(input));
            },
            "Fahrenheit" => {
                println!("Enter Fahrenheit: ");
                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Failed to read line");
                let input: f64 = input.trim().parse().expect("Please type a number!");

                println!("Celcius: {}", fahrenheit_to_celcius_helper(input));
            },
            "Quit" => {
                break 'processing;
            },
            _ => {
                println!("Invalid input");
            }
        }
    }
}

fn generate_nth_fibonacci(n: usize) -> usize {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else {
        return generate_nth_fibonacci(n - 1) + generate_nth_fibonacci(n - 2);
    }
}

fn main() {

    println!("\"Temperature\" or \"Fibonacci\" or \"Quit\": ");
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Failed to read line");

    match input.trim() {
        "Temperature" => {
            temperature_converter();
        },
        "Fibonacci" => {
            let mut input = String::new();
            println!("Enter n: ");
            io::stdin().read_line(&mut input).expect("Failed to read line");
            let input: usize = input.trim().parse().expect("Please type a number!");

            println!("Fibonacci: {}", generate_nth_fibonacci(input));
        }
        "Quit" => {
            return;
        },
        _ => {
            println!("Invalid input");
        }
    }
}
