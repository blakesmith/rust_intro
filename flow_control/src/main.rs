fn if_else() {
    let x = 5;
    if x < 10 {
        println!("Less than: {}", x);
    } else {
        println!("Greater than: {}", x);
    }
}

fn for_in() {
    for i in 0..5 {
        println!("Value is: {}", i);
    }
}

fn while_loop() {
    let mut x = 5;
    while x > 0 {
        println!("The value is: {}", x);
        x -= 1;
    }
}

fn plain_loop() {
    let mut x = 0;
    loop {
        println!("The value is: {}", x);
        if x > 20 {
            break;
        }
        x += 1;
    }
}

fn main() {
    if_else();
    for_in();
    while_loop();
    plain_loop();
}
