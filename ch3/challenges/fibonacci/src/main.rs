use std::io;

fn main() {
    let mut nth = String::new();

    println!("Enter nth number to find in the fibonacci sequence:");

    io::stdin()
        .read_line(&mut nth)
        .expect("Unable to read line.");

    let nth = nth.trim();
    let mut n : i32 = nth.parse().expect("Failed to parse.");
    
    let mut one = 0;
    let mut two = 1;

    if n == 0 {
        two = one;
    }
    else if n != 1 {
        n -= 2;
        while n >= 0 {
            let new = one + two;
    
            one = two;
            two = new;
    
            n -= 1;
        }
    }

    println!("{nth}th fibonacci number: {two}");
}
