fn main() {
    arrays();
    vector();
    slices();
}

fn arrays() {
    // fixed-size list of elements of the same type
    // by default immutable
    let arr = [1, 2, 3]; // a: [i32; 3]
    let mut arr2 = [1, 2, 3]; // [i32, 20]

    // Arrays have type [T; N]

    println!("To print array length {}", arr.len());
    for b in arr2.iter() {
        println!("{}", b);
    }

    // You can access a particular element of an array with subscript notation
    let names = ["Grayd", "BR", "A"];

    // array access is bounds-checked at run-time
    // errant access is the source of many bugs
    // in other systems programming languages.
    println!("Second `names` element is: {}", names[1]);
}

fn vector() {
    // Is a dynamic/growable array implemented as the standard
    // library type Vec<T>
    // Vector always allocate their data on the heap.

    // Vectors are to slices what String is to &str
    // square brackets [] with vec!
    let v = vec![1, 2, 3]; // v: Vec<i32>

    // alternative form of vec! for repeating initial value
    let v2 = vec![0; 10];

    // You can get the length of
    // iterate over
    // and subscript vectors just like arrays.
    // In addition, (mutable) vectors can grow automatically
    let mut nums = vec![33, 2];

    nums.push(222);

    println!("The length of nums is {}", nums.len());
}

fn slices() {
    // A slice is a reference to (or "view" into) an array.
    // They are useful for allowing safe, efficient access
    // to a portion of an array without copying.
    // For example, you might want to reference
    // just one line of a file read into memory.
    // By nature, a slice is not created directly,
    // but from an existing variable.
    // Slices have a length, can be mutable or not,
    // and in many ways behave like arrays:
    let a = [2, 39, 10, 20, 8];
    let middle = &a[1..4];

    for el in middle.iter() {
        println!("Middle {}", el);
    }

    // You can also take a slice of a vector, String, or &str,
    // because they are backed by arrays.
    // Slices have type &[T], which we'll talk about when we cover generics.
}
