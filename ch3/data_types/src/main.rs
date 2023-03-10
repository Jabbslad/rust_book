fn main() {
    let x: u8 = 255;
    println!("wrapping add: {}", x.wrapping_add(1));
    
    let (y, z) = x.overflowing_add(1);
    println!("overflowing add: {} ({})", y, z);
    
    if let Some(n) = 220u8.checked_add(35) {
        println!("checked add: {}", n);    
    }
}

