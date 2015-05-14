fn main() {
    // Patterns

    let x = 8;

    // Ranges are mostly used with integers and single characters.
    // If you're matching multiple things, via a | or a ...,
    // you can bind the value to a name with @:
    match x {
        1 | 2 => println!("one or two"),
        var @ 3 ... 10 => println!("range from 3 to 10, got {}", var),
        _ => println!("anything"),
    }

    // If you're matching on an enum which has variants, you can use ..
    // to ignore the value and type in the variant

    enum OptionalInt {
      Value(i32),
      Missing,
    }

    let x = OptionalInt::Value(2922);

    // You can introduce match guards with if
    match x {
      OptionalInt::Value(i) if i > 5 => println!("Got an int bigger than five!"),
      OptionalInt::Value(..) => println!("Got an int!"),
      OptionalInt::Missing => println!("No such luck"),
    }


    // If you're matching on a pointer, you can use the
    // same syntax as you declared it with. First, &:
    let x = &5;

    match x {
      &val => println!("Got value {}", val),
    }
    // Here, the val inside the match has type i32.
    // In other words, the left-hand side of the
    // pattern destructures the value.
    // If we have &5, then in &val, val would be 5.

    let z = 11;

    match z {
      ref r => println!("Got reference to {}", r)
    }
    // Here, the r inside the match has the type &i32.
    // In other words, the ref keyword creates a reference,
    // for use in the pattern.
    // If you need a mutable reference,
    // ref mut will work in the same way:
    let mut u = 1010;

    match u {
      ref mut mu => println!("Got a mutable reference to {}", mu)
    }

    // If you have a struct, you can destructure it inside of a pattern:
    struct Po {
      x: i32,
      y: i32,
    }
    let origin = Po { x: 2, y: 1 };
    match origin {
      Po { x: x, .. } => println!("Matching Po struct {}", x)
    }

    // If you want to match against a slice or array, you can use &
    let v = vec!["match_this", "1"];

    // experimental
    // match &v[..] {
    //   ["match_this", second] => println!("Second is {}", second),
    //   _ => {},
    // }

    // Mix matching
    // match x {
    //     Foo { x: Some(ref name), y: None } => ...
    // }
}
