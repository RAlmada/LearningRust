use std::io;

fn main() {
    let mut temp = String::new();

    println!("Input temperature in Fahrenheit: ");

    io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read input.");

    let temp: f64 = temp.trim().parse().expect("Did not provide a valid number.");

    println!("{temp:.2}°f -> {:.2}°c", (temp - 32.0) / (9.0 / 5.0));
}
