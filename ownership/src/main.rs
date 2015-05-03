use std::rc::Rc;

fn main() {
    println!("Rust ownership system achieves memory safety");
    println!("ownership, borrowing and lifetimes");

    // META
    // Rust focus: speed & safety
    // through zero-cost abstractions
    // ownership system at compile time

    // Fighting with the borrow checker
    // programmer mental model mismatch with actual Rust rules

    // Tracking memory allocations & deallocations
    // {
    //     int *x = malloc(sizeof(int));
    //     *x = 5;
    //     free(x);
    // }
    // malloc allocates some memory, free deallocates the memory.
    // thus bookkeeping about allocating the right amount of memory.

    // Rust combines these 2 aspects of memory allocation into concept
    // called ownership.
    //
    // Whenever memory request comes in it is called: owning handle.
    // After handle goes out of scope it is automatically deallocated.
    //
    // Allocating i32 pointer to HEAP
    {
        let x = Box::new(5);
    }
    // at block end memory is automatically deallocated
    // each allocation is paired with deallocation
    {
        let z = Box::new(5);

        let y = add_one(z);
        println!("{}", y);
    }
    fn add_one(mut num: Box<i32>) -> Box<i32> {
        *num += 1;

        num // ownership is transferred back
    }

    // Borrowing
    //
    // version of borrowing rather than taking ownership
    {
        let mut x = 7;

        add_one2(&mut x);
        println!("{}", x);
    }
    fn add_one2(num: &mut i32) {
        *num += 1 // ownership borrowing
    }

    // Lifetimes
    // Linking out a reference to a resource that someone else owns, can
    // be complicated, however:
    //
    // 1. I acquire a handle to some kind of resource
    // 2. I lend you a reference to the resource
    // 3. I decide I'm done with the resource, and deallocate it,
    // while still having reference
    // 4. You decided to use the resource
    //
    // Whne reference is pointing to invalid resource - dangling pointer or
    // 'use after free', when the resource is memory
    //
    // To fix this, we have to make sure that step four never happens after
    // step three. The ownership system in Rust does this through a concept
    // called lifetimes, which describe the scope that a reference is valid
    // for.
    //
    fn lifetime1(num: &mut i32) {
        *num += 1
    }
    // Rust: 'lifetime elision' which allows not to write
    // lifetimes in certain circumstances.
    //
    // lifetime1 - without eliding lifetimes
    fn lifetime2<'a>(num: &'a mut i32) {
        *num += 1
    }
    // 'a is called a lifetime
    // lifetime2<'a> - declares one lifetime, it is possible to specify more
    // lifetime2<'a, 'b>.
    // In parameter list - we use declared lifetimes
    //
    // &mut i32 - mutable reference to an i32
    // &'a mut i32 - mutable reference to an i32 with lifetime 'a
    //
    // Why do lifetimes matter ?
    struct Foo<'a> {
        x: &'a i32,
    }
    // we need to ensure that any reference to a Foo cannot outlive the
    // reference to an i32 it contains

    let y = &5; // same as let _y = 5; let y = &_y;
    let f = Foo { x: y };

    println!("{}", f.x);

    // Static lifetime
    // it signals that something has lifetime of entire program.
    let x: &'static str = "Yet another hello";

    // String literals have the type &'static str becuase the reference
    // is always alive: they are baked into the data segment of the final
    // binary.

    // Globals:
    static FOO: i32 = 5;
    let x: &'static i32 = &FOO;
    // adds an i32 to the data segment of the binary, and x is a reference
    // to it.

    // Shared ownership
    // in all considered exampels we've assumed that each handle has a singular
    // owner. But sometimes this doesn't work.
    //
    // E.g.: Car with four wheels, we want to know which wheel it was attached
    // to.

    // use std::rc::Rc;

    struct Car {
        name: String,
    }

    struct Wheel {
        size: i32,
        owner: Rc<Car>,
    }

    {
        let car = Car { name: "MB".to_string() };

        let car_owner = Rc::new(car);

        for _ in 0..4 {
            Wheel { size: 360, owner: car_owner.clone() };
        }
    }
    // We can't do this with Box<T> pointer since it has single owner
    // We need to use Rc<T> getting Rc<Car> we can use clone() to make new
    // references.

    // Arc<T> could be used to make atomic instructions and be thread-safe
    // counterpar of Rc<T>.

    // Lifetime elision
    // Rust supports powerful local type inference in function bodies, but it's
    // forbidden in item signatures to allow reasoning about the types just based
    // in the item signature alone. However for ergonomic reasons a very
    // restricted secondary inference algorithm called 'lifetime elision'
    // applies in function signatures. It infers only based on the signature
    // components themselves and not based on the body of the function, only
    // infers lifetime parameters, and does this with three easily memorizable
    // and unambigous rules. This makes lifetime elision a shorthand for writing
    // an item signature, while not hiding away the actual types involved
    // as full local inference would if applied to it.
    //
    // When talking about lifetime elision, we use the term 'input lifetime'
    // and 'output lifetime'.
    //
    // 'input lifetime' - lifetime associated with a parameter of a function
    // 'output lifetime' - lifetime associated with the return value of a
    // function

    // input lifetime, e.g.:
    // fn foo<'a>(bar: &'a str)

    // output lifetime
    // fn foo<'a>() -> &'a str

    // input && output lifetime
    // fn foo<'a>(bar: &'a str) -> &'a str

    // 3 rules
    // - Each elided lifetime in a function's arguments becomes distinct
    // lifetime parameter
    // - If there is exactly 1 input lifetime, elided or not, that lifetime
    // is assigned to all elided lifetimes in the return values of that function.
    // - If there are multiple input lifetimes, but one of them &self or
    // &mut self, the lifetime of self is assigned to all elided output
    // lifetimes.
    //
    // Otherwise, it is an error to elide an output lifetime.


    // fn print(s: &str); // elided
    // fn print<'a>(s: &'a str); // expanded
    //
    // fn debug(lvl: u32, s: &str); // elided
    // fn debug<'a>(lvl: u32, s: &'a str); // expanded
    //
    // // In the preceding example, `lvl` doesn't need a lifetime because it's not a
    // // reference (`&`). Only things relating to references (such as a `struct`
    // // which contains a reference) need lifetimes.
    //
    // fn substr(s: &str, until: u32) -> &str; // elided
    // fn substr<'a>(s: &'a str, until: u32) -> &'a str; // expanded
    //
    // fn get_str() -> &str; // ILLEGAL, no inputs
    //
    // fn frob(s: &str, t: &str) -> &str; // ILLEGAL, two inputs
    // fn frob<'a, 'b>(s: &'a str, t: &'b str) -> &str; // Expanded: Output lifetime is unclear
    //
    // fn get_mut(&mut self) -> &mut T; // elided
    // fn get_mut<'a>(&'a mut self) -> &'a mut T; // expanded
    //
    // fn args<T:ToCStr>(&mut self, args: &[T]) -> &mut Command // elided
    // fn args<'a, 'b, T:ToCStr>(&'a mut self, args: &'b [T]) -> &'a mut Command // expanded
    //
    // fn new(buf: &mut [u8]) -> BufWriter; // elided
    // fn new<'a>(buf: &'a mut [u8]) -> BufWriter<'a> // expanded'>
}
