fn main() {
    let m1 = String::from("hello");
    let m2 = String::from("world");
    greet(m1, m2);
    //let s = format!("{m1} {m2}");
    
    let m3 = String::from("hello");
    let m4 = String::from("world");
    let (m3_again, m4_again) = greet2(m3, m4);
    
    let _s = format!("{m3_again} {m4_again}");

    let m5 = String::from("hello");
    let m6 = String::from("world");
    
    greet3(&m5, &m6);

    let _s = format!("{m5} {m6}");

    //dereferencing
    let mut x = Box::new(2);
    let a = *x;
    println!("{}", a);
    *x += 1;
    println!("{}", x);
    println!("{}", a);

    let r1 = &x;
    let b = **r1;
    println!("b = {b}");
    
    let c = &*x;
    println!("c = {c}");
}

fn greet(g1: String, g2: String) {
    println!("{g1} {g2}");
}


fn greet2(g1: String, g2: String) -> (String, String) {
    println!("{g1} {g2}");
    (g1, g2)
}

fn greet3(g1: &String, g2: &String) {
    println!("{g1} {g2}");
}
