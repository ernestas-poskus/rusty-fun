fn main() {
    print_number(233);
    print_sum(22, 34);

    let x = 2;
    print_number(add_one(x));
    print_number(x);

    diverges();
}

fn empty_function() {
}

fn print_number(x: i32) {
    println!("x is: {}", x);
}

fn print_sum(x: i32, y :i32) {
    println!("sum x + y = {}", x + y);
}

fn add_one(x: i32) -> i32 {
    x + 1
}

fn diverges() -> ! {
    panic!("Panic about rusty")
}
