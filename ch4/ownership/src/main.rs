fn read(y: bool) {
    if y {
        println!("y is true");
    }
}

fn add_suffix(mut first: String) -> String {
    first.push_str(&String::from(" Jr."));
    first
}

fn main() {
    let x = true;
    read(x);

    let a = [1, 2, 3, 4, 5];
    let b = a;

    assert!(!std::ptr::eq(&a, &b));

    let c = Box::new([1, 2, 3, 4, 5]);
    let d = &c;

    assert!(std::ptr::eq(&c, &*d));

    let mut first = String::from("Jamie");
    let full_name = add_suffix(first);
    //first.push_str(" snr.");
    println!("{full_name}");
}
