fn main() {

    let mut counter = 0;

    let result = loop {
        counter += 1;
        
        if counter == 10 {
            break counter * 2;
        }
    };

    println!("{result}");


    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("end count = {count}");


    let mut number = 3;
    
    while number != 0 {
        println!("{number}");
        number -= 1;
    }

    println!("LIFTOFF!");

    let a = [1, 2, 3, 4, 5];

    for element in a {
        println!("{element}");
    }

    for element in (1..=3).rev() {
        println!("{element}");
    }
    println!("LIFTOFF!");
}
