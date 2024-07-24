use std::io;

fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is {}", y);
    println!("The first element is {}", tup.0);

    // Experiment: create an array and attempt to read past the valid ranges
    let a = [1, 2, 3, 4, 5];

    let mut index = String::new();

    println!("Enter an array index: ");

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
                        .trim()
                        .parse()
                        .expect("Index must be a number");

    println!("The value of the element at index {} is: {}", index, a[index]);

}
