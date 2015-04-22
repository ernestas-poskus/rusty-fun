fn main() {
    tuples();
    structs();
    tuple_structs();
    enums();
    ordering();
}

fn tuples() {
    // Tuples
    // A tuple is an ordered list of fixed size

    let t1 = (1, "tup");

    // With annotated type
    let t2: (i32, &str) = (292, "string");

    // Destructuring tuple
    let (x, y, z) = (1, 2, 3);
    println!("x is {}", x);

    // Arity
    // Arity - refers to the number of arguments a function or operation takes.
    let mut q = (1, 2);
    let o = (2, 3);

    // Mutating
    q = o;

    let (print1, print2) = q; // Destructuring for printing

    println!("o is ({}, {})", print1, print2);

    // You can also check for equality with ==.
    // Again, this will only compile if the tuples have the same type.
    // Order is considered as well.
    let x1 = (1, 2, 3);
    let x2 = (3, 2, 1);

    if x1 == x2 {
        println!("Equal");
    } else {
        println!("Not Equal");
    }

    // Other use of tuple is returning multiple values from a function
    fn next_two(x: i32) -> (i32, i32) { (x + 1, x + 2) }

    let(z1, z2) = next_two(98);
    println!("z1, z2 = {}, {}", z1, z2);

    // Even though Rust functions can only return one value,
    // a tuple is one value, that happens to be made up of more than one value.
    // You can also see in this example how you can
    // destructure a pattern returned by a function, as well.
}

fn structs() {
    // Convention: struct starts with capital letter, is camel cased

    // A struct is another form of a record type, just like a tuple.
    // There's a difference: structs give each element that they
    // contain a name, called a field or a member. Check it out:
    struct Point {
        x: i32,
        y: i32,
        z: i32
    }

    // Empty field (not initialized) results in compiling error
    let origin = Point { x: 29, y: 101, z: 2992};

    // By default: immutable
    // origin.z = 29292;
    println!("Origin points ({}, {}, {})", origin.x, origin.y, origin.z);

    let mut origin2 = Point { x: 1, y: 2, z: 0};
    origin2.z = 33;
    println!("Origin2 points ({}, {}, {})", origin2.x, origin2.y, origin2.z);
}

fn tuple_structs() {
    // Rust has another data type that's like a
    // hybrid between a tuple and a struct, called a tuple struct.
    // Tuple structs do have a name, but their fields don't:

    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    // Will not be equal even if have same values
    let black = Color(1, 1, 1);
    let origin = Point(1, 1, 1);

    // It is almost always better to use a struct than a tuple struct.

    // There is one case when a tuple struct is very useful,
    // though, and that's a tuple struct with only one element.
    // We call this the newtype pattern,
    // because it allows you to create a new type,
    // distinct from that of its contained value and expressing its own
    // semantic meaning:
    struct Inches(i32);

    let length = Inches(10);

    // As you can see here, you can extract the inner integer
    // type through a destructuring let, as we discussed previously in 'tuples.'
    // In this case, the let Inches(integer_length) assigns 10 to integer_length.
    let Inches(integer_length) = length;
    println!("Length is {} inches", integer_length);
}

fn enums() {
    // An enum is a type which relates a set of alternates to a specific name.
    // Below we define Character to be either a Digit or something else.
    enum Character {
        Digit(i32),
        Other,
    }

    struct Empty;
    struct Color(i32, i32, i32);
    struct Length(i32);
    struct Status { Health: i32, Mana: i32, Attack: i32, Defense: i32 }
    struct HeightDatabase(Vec<i32>);

    // Enum depending on it's type may or may not hold data.
    // In Character, for instance, Digit gives a meaningful name
    // for an i32 value, where Other is only a name.

    let ten = Character::Digit(10);
    let other = Character::Other;
}

use std::cmp::Ordering::{self, Equal, Less, Greater};

fn cmp(a: i32, b: i32) -> Ordering {
    if a < b { Less }
    else if a > b { Greater }
    else { Equal }
}

fn ordering() {
    let x = 5;
    let y = 10;

    let ordering = cmp(x, y);

    if ordering == Less { println!("less") }
    else if ordering == Greater { println!("greater") }
    else if ordering == Equal { println!("equal") }
}
