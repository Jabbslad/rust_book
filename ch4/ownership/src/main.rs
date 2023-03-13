fn read(y: bool) {
    if y {
        println!("y is true");
    }
}

fn main() {
    let x = true;
    read(x);

    let a = [1, 2, 3, 4, 5];
    let b = a;

    assert!(std::ptr::eq(&a, &b));
}
