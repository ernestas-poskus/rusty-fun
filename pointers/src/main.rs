fn main() {
    // Rust: use them for a deliberate purpose

    // Pointer basics
    let x = 5;
    let y = 9;

    // location     value
    // 0xd3e030     5
    // 0xd3e028     9

    pass_by_reference_by_value();
    common_pointer_problems();
    pointer_aliasing();

    // The initial * dereferences the pointer,
    // and then & takes a reference to those contents
    reference_pointer();
    boxes_pointer();
    best_practices();
    patterns_and_ref();

    // Type         Name                    Summary
    // &T           Reference               Allows 1 or more references to read T
    // &mut T       Mutable reference       Allows 1 reference to read/write T
    // Box<T>       Box                     Heap allocated T with single owner that may read and write T.
    // Rc<T>        "arr cee" pointer       Heap allocated T with many readers
    // Arc<T>       Arc pointer             Heap allocated T with many readers + safe sharing across threads
    // *const T     Raw pointer             Unsafe read access to T
    // *mut T       Mutable raw pointer     Unsafe read/write access to T
}

// 1. Basic `reference` pointer
fn reference_pointer() {
    let x = 5;
    let y = 9;
    let z = &y;

    // location     value
    // 0xd3e030     5
    // 0xd3e028     9
    // 0xd3e020     0xd3e028

    println!("y memory address: {:p}", &y);
    println!("z memory address: {:p}", z);
    println!("Dereference z: {}", *z);
    println!("Auto dereference referent z to value: {}", z);

    println!("x + z = {}", x + *z);

    let a = 2;

    fn succ(x: &i32) -> i32 { *x + 1 }

    println!("{}", a);
    println!("a after succ {}", succ(&a));

    // References are immutable by default
    let x = 5;
    let y = &x;

    // error cannot assign to immutable borrowed content `*y`
    // *y = 50;

    // it is possible to mutate reference with mut,
    // but only if its referent is also mutable
    let mut xx = 22;
    let y = &mut xx;

    // ONLY Immutable pointers are allowed to alias
    let u = 129;
    let i = &u;
    let o = &u;

    // error: cannot borrow `u` as mutable more than once at a time
    let mut j = 292;
    let k = &mut j; // ONLY once
    // let l = &mut j;

    // Use references when you want to use a pointer,
    // but do not want to take ownership.
    // References just borrow ownership,
    // which is more polite if you don't need the ownership.

    // As a corollary to that rule, references allow you to
    // accept a wide variety of other pointers, and so are useful
    // so that you don't have to write a number of variants per
    // pointer.
}

fn boxes_pointer() {
    // Boxes provide simplest form of heap allocation in Rust.

    let x = Box::new(5);

    // boxes are heap allocated and they are deallocated
    // automatically when out of scope
    {
        let y = Box::new(5);
    }
    // boxes do NOT use reference counting or GC
    // boxes are - affine type

    // affinity (from the Latin, affinis, "connected with") is a function
    // between affine spaces which preserves points, straight lines and
    // planes. Also, sets of parallel lines remain parallel after an
    // affine transformation.

    // This means Rust compiler at compile time, determines when the box
    // comes into and goes out of scope, and inserts appropriate calls there.

    // you get the semantics of malloc/free but with some improvements
    // 1. impossible to allocate incorrect amount of memory
    // 2. cannot forget free
    // 3. Insurance that free happens right time after it is not used
    // 4. No other writable pointers to this heap memory
    fn add_one(x: &i32) -> i32 {
        *x + 1
    }

    let x = Box::new(100);

    // Rust knows x is being borrowed by the add() and since it is
    // only reading value, allows it
    fn add(x: &i32, y: &i32) -> i32 {
        *x + *y
    }
    println!("{}", add_one(&*x));
    println!("{}", add(&*x, &*x));


    // it is possible to borrow x multiple times, but only if 'x' is mutable
    // and it may be simultaneously borrowed
    fn increment(x: &mut i32) {
        *x += 1;
    }

    let mut r = Box::new(2);
    increment(&mut r);
    increment(&mut r);
    increment(&mut r);
    increment(&mut r);
    println!("{}", r);
}

fn best_practices() {
    // boxes are most appropriate for recursive data structures

    #[derive(Debug)]
    enum List<T> {
        Cons(T, Box<List<T>>),
        Nil,
    }

    let list: List<i32> = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))));

    println!("{:?}", list);

    // the reference to another List must be a Box because of unknown
    // length of the list. Since length is unknown, size is unknown too
    // and therefore, we need heap to allocate our list
}

fn patterns_and_ref() {
    // matching pointers

    fn possibly_print(x: &Option<String>) {
        match *x {
            // BAD: cannot move out of a `&`

            // GOOD: instead take a reference into the memory of the `Option`
            Some(ref s) => println!("{}", *s),
            None => {}
        }
    }
}
// ref s here means that s will be of type &String, rather than type String.

// In a language which is pass by value,
// i will be 5 at the comment.
// You see, because the argument x is a pointer,
// we do send a copy over to foo,
// but because it points at a memory location,
// which we then assign to, the original value is still changed.
// This pattern is called pass-reference-by-value.
fn pass_by_reference_by_value() {
//     func foo(&i32 x) {
//         *x = 5;
//     }
//
//     let i = 1;
//     func(&i);
//     println!("{}", i);
}

fn common_pointer_problems() {
//     fn make_pointer(): &int {
//         let x = 5;
//         return &x
//     }
//
//     &int i = make_pointer();
//     *i = 8;
}

fn pointer_aliasing() {
    // func mutate(&int i, int j) {
    //     *i = j;
    // }

    // x = 5;
    // y = &x;
    // z = &x; // y & z are aliased

    // run_in_new_thread(mutate, y, 1);
    // run_in_new_thread(mutate, z, 100);

    // what is value of x ?
}
