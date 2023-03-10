use std::io::stdin;

fn main() {
    let a = [1, 2, 3, 4, 5];
    println!("{:?}", a);

    let b: Vec<u8> = (1..=10).collect();

    println!("enter an index [0->9]: ");
    let mut buf = String::new();
    stdin().read_line(&mut buf).expect("need to enter valid index");
    let val: usize = buf.trim().parse().expect("failed to parse index");

    println!("{:?}", b[val]);

}
