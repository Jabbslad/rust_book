fn main() {
    println!("Hello, world!");

    another_function();
    another_function2(32);
    let unit = 'h';
    print_labeled_measurement(5, &unit);
    println!("unit = {unit}");

    println!("The value of x is: {}", five());

    println!("5 plus 1 = {}", plus_one(5));
}

fn another_function() {
    println!("Another function.");
}

fn another_function2(x: i32) {
    println!("The value of x is: {}", x);
}

fn print_labeled_measurement(value: i32, unit_label: &char) {
    println!("The measurement is {value}{unit_label}");
}

fn five() -> u8 {
    5
}

fn plus_one(x: u8) -> u8 {
    x + 1
}
