use std::io;

fn fahrenheit_to_celcius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

fn celcius_to_fahrenheit(c: f64) -> f64 {
    c * 9.0 / 5.0 + 32.0
}

fn main() {

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

                println!("Fahrenheit: {}", celcius_to_fahrenheit(input));
            },
            "Fahrenheit" => {
                println!("Enter Fahrenheit: ");
                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Failed to read line");
                let input: f64 = input.trim().parse().expect("Please type a number!");

                println!("Celcius: {}", fahrenheit_to_celcius(input));
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
