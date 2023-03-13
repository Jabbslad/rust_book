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

    assert!(!std::ptr::eq(&a, &b));

    let c = Box::new([1, 2, 3, 4, 5]);
    let d = &c;

    assert!(std::ptr::eq(&c, &*d));

}
