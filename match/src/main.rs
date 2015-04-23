fn main() {
    let x = 5;

    // Each arm of the branch is of the form val => expression.
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        4 => println!("four"),
        5 => println!("five"),
        _ => println!("default"),
    }

    match_ordering();
    enum_match();
}

use std::cmp::Ordering;

fn match_ordering() {
    fn cmp(a: i32, b: i32) -> Ordering {
        if a < b { Ordering::Less }
        else if a > b { Ordering::Greater }
        else { Ordering::Equal }
    }

    let x = 5;
    let y = 8;

    // With our if/else version, if we had forgotten the Greater case,
    // for example, our program would have happily compiled.
    // If we forget in the match, it will not.
    // Rust helps us make sure to cover all of our bases.
    match cmp(x, y) {
        Ordering::Less => println!("less"),
        Ordering::Greater => println!("greater"),
        Ordering::Equal => println!("equal"),
    }
}

fn enum_match() {
    enum OptionalInt {
        Value(i32),
        Missing,
    }

    let x = OptionalInt::Value(5);
    let y = OptionalInt::Missing;

    match x {
        OptionalInt::Value(n) => println!("x is {}", n),
        OptionalInt::Missing => println!("x is missing"),
    }

    // That is how you can get and use the values contained in enums.
    // It can also allow us to handle errors or unexpected computations;
    // for example, a function that is not guaranteed to be able
    // to compute a result (an i32 here) could return an OptionalInt,
    // and we would handle that value with a match.
    match y {
        OptionalInt::Value(n) => println!("y is {}", n),
        OptionalInt::Missing => println!("y is missing"),
    }
}
