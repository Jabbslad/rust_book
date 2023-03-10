fn main() {
    let x: u8 = 255;
    println!("wrapping add: {}", x.wrapping_add(1));
    
    let (y, z) = x.overflowing_add(1);
    println!("overflowing add: {} ({})", y, z);
    
    if let Some(n) = 220u8.checked_add(35) {
        println!("checked add: {}", n);    
    }

    // floating point
    let x = 1.0; // f64
    let mut y = 2.0f32;
    
    y += x;

    println!("{}", y);

    let t: (u8, i32, f32) = (8, 23, 1.0);
    println!("{}, {}, {}", t.0, t.1, t.2);
    // or
    let (x, y, z) = t;
    println!("{x}, {y}, {z:.2}");
    
}

