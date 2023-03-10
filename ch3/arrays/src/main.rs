use std::{io::stdin, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    let a = [1, 2, 3, 4, 5];
    println!("{:?}", a);

    let b: Vec<u8> = (1..=10).collect();

    println!("enter an index [0->9]: ");
    let mut buf = String::new();
    stdin().read_line(&mut buf)?;
    let val: usize = buf.trim().parse()?;

    println!("{:?}", b[val]);

    Ok(())
}
