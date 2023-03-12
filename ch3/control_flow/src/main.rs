fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    if number > 3 {
        println!("number greater than 3");
    } else if number < 3 {
        println!("number less than 3");
    } else {
        println!("number is 3!");
    }
}
