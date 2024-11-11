fn main() {
    println!("Hello, world!");
    another_function();
    another_function_x(123);
    print_labeled_measurement(321, 'k');
}

fn another_function() {
    println!("Another function.");
}

fn another_function_x(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}
