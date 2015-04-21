fn main() {
    println!("Hello, world!");

    let x = 5;

    if x == 5 {
        println!("x == 5 {}", x == 5);
    } else if x == 2 {
        println!("x == 2 {}", x == 2);
    } else {
        // Unreachable but throw re-assignment error
        // x = 202
    }


    // Short hand IF & assignment

    let z = 4;

    let y = if z == 4 {
        202
    } else {
        22
    };

    println!("Y {}", y);

    // OR

    let y: i32 = if x == 5 { 20202 } else { 1 };

}
