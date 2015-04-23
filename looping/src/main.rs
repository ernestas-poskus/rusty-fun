fn main() {
    for x in 0..10 {
        println!("{}", x);
    }

    while_loop();
    infinite_loop();
}

fn while_loop() {
    let mut x = 0;
    let mut done = false;

    // while loops are the correct choice when
    // you're not sure how many times you need to loop.
    while !done {
        x += x - 3;
        println!("{}", x);
        if x % 5 == 0 { done = true; }
    }
}

fn infinite_loop() {
    let mut x = 0;

    // Rust's control-flow analysis treats this
    // construct differently than a while true
    loop {
        x += 10;
        if x == 1000 {
            break;
        } else if x % 70 == 0 {
            println!("{}", x);
        } else {
            // Redundant
            continue;
        }
    }
}
