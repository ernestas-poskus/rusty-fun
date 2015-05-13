use std::str;

fn main() {
    // 2 main 'string' types

    // 1. &str
    let string = "Hello there.";
    // string literal is a &'static str. A string slice can be written
    // without an explicit lifetime in many cases, such as in function
    // arguments. In these cases lifetime will be inferred.
    fn takes_slice(slice: &str) {
        println!("Got {}", slice);
    }
    // string slcies are 'view' into an already-allocated string



    struct S {
        s: String,
    }
    let stra = S { s: "awdaw".to_string() };
    // heap-allocated, growable, guaranteed to be UTF-8, commonly created
    // by converting from string slice using { to_string }
    let s = "Hello".to_string();
    println!("{}", s);

    // a reference to a String will automatically coerce to a string slice
    takes_slice(&s);

    // can also get a &str from a stack-allocated array of bytes:

    let x: &[u8] = &[b'a', b'a'];
    let stack_str: &str = str::from_utf8(x).unwrap();


    // Best practise
    // one should prefer String when you need ownership and &str when you
    // just need to borrow a string. Similar to Vec<T> vs. &[T],
    // and T vs &T in general

    // meaning starting off with:
    // fn foo(s: &str)
    // and only moving to this:
    // fn foo(s: String)

    // if you have a good reason. It is not polite to hold on to
    // onwership you don't need, as it can make your lifetimes
    // more complex.

    // Generic functions
    fn some_string_length(x: &str) -> usize {
        x.len()
    }

    let s1 = "heel";
    println!{"{}", some_string_length(s1)};

    let s2 = "awkjdjk2k2".to_string();
    println!{"{}", some_string_length(&s2)};

    // Indexing strings

    // TEMPTED TO DO;
    let i1 = "hello".to_string();
    // println!("{}", s[0]);
    // this does not compile, in the worlf of UTF-8, direct indexing
    // is basically never what you want to do. Reason is that each
    // character can be a variable number of bytes O(n).

    // 3 basic levels of unicode
    // - code units, the underlying data type used to store everything
    // - code points/unicode scalar values(char)
    // - graphemes (visible characters)

    // Rust analog
    // - .bytes()
    // - .chars() will iterate over code points
    // - .graphemes() will iterate over graphemes

    // let s = "u͔n͈̰̎i̙̮͚̦c͚̉o̼̩̰͗d͔̆̓ͥé";
    //
    // for l in s.graphemes(true) {
    //     println!("{}", l);
    // }

    // Deref coercions
    // References to String will automatically coerce into &str

    fn hello(s: &str) {
        println!("{}", s);
    }

    let slice = "Steve";
    let string = "Awdkajkwk".to_string();

    hello(slice);
    hello(&string);
}
