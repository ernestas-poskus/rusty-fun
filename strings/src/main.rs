fn main() {
    // All strings are guaranteed to be validly encoded UTF-8 sequences

    // Rust has two main types of strings: &str and String.

    // 1. is a &str. These are called string slices
    // statically allocated & exists for entire duration it runs
    let string = "hee"; // string: &str

    // The string binding is a reference to this
    // statically allocated string.
    // String slices have a fixed size, and cannot be mutated.

    // 2. String, on the other hand, is a heap-allocated string
    // This string is growable, and is also guaranteed to be UTF-8.
    // Strings are commonly created by
    // converting from a string slice using the to_string method.
    let mut s = "he".to_string();
    println!("{}", s);

    s.push_str("llo");
    println!("{}", s);


    // Strings will coerce into &str with an &:
    // &str are reference to another string

    fn takes_slice(slice: &str) {
        println!("Got {}", slice);
    }

    let s2 = "HHEE".to_string();

    // Viewing a String as a &str is cheap,
    // but converting the &str to a String
    // involves allocating memory.
    takes_slice(&s2);
}
