use std::{fs::read_dir, error::Error};

fn main() -> Result<(), Box<dyn Error>> {

    // iterative
    for entry in read_dir(".")? {
        let dir = entry?;
        println!("{:?}", dir.path());
    }

    // functional
    read_dir(".")?
        .map(|x| x.unwrap().path())
        .for_each(|y| println!("{:?}", y));

    Ok(())
}
