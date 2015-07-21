fn main() {
  // Anonymous functions have associated environment are called 'closures'
  // because they close over an environment.

  let plus_one = |x: i32| x + 1;

  assert_eq!(2, plus_one(1));

  // Binding plus_one and assigning it to a closure. Closure arguments
  // go between the pipes (|) and the body is expression.

  let plus_two = |x| {
    let mut result: i32 = x;

    result += 1;
    result += 1;

    result
  };

  assert_eq!(4, plus_two(2));

  // Closure does not need to annotate the types of arguments the closure
  // takes or the values it returns
  let plus_one2 = |x: i32| -> i32 { x + 1 };
  let plus_one3 = |x| { x + 1 };

  assert_eq!(2, plus_one2(1));
  assert_eq!(2, plus_one3(1));

  // Nevertheless specifying the full type for named functions is helpful
  // with things like documentation and type inference, the types of closures
  // are rarely documented since they're anonymous, and they don't cause
  // the kinds of error-at-a-distance that inferring named function types can.
  fn  plus_one_v1 (  x: i32 ) -> i32 { x + 1 }
  let plus_one_v2 = |x: i32 | -> i32 { x + 1 };
  let plus_one_v3 = |x: i32 |          x + 1  ;

  closures_and_their_environment();
  move_closures();
  closure_implementation();
  taking_closures_as_arguments();
  returning_closures();
}

fn closures_and_their_environment() {
  let num = 5;
  let plus_num = |x: i32| x + num;

  assert_eq!(10, plus_num(5));

  // Closure, plus_num, refers to a let binding in its scope: num. More
  // specifically, it borrows the binding.

// let mut num4 = 4;
// let plus_num4 = |x: i32| x + num4;
// let y = &mut num;
// > cannot borrow `num` as mutable because it is also borrowed as immutable

  let mut num5 = 5;
  {
      let plus_num5 = |x: i32| x + num5;
  } // plus_num goes out of scope, borrow of num ends
  let y = &mut num5;


// let nums = vec![1, 2, 3];
// let takes_nums = || nums;
// println!("{:?}", nums);
// > `nums` moved into closure environment here because it has type
// > `[closure(()) -> collections::vec::Vec<i32>]`, which is non-copyable

  // Vec<T> has ownership over its contents, and therefore, when we refer
  // to it in our closure, we have to take ownership of nums.
  // It's the same as if we"d passed nums to a function that took ownership of it.
}

fn move_closures() {
  // it is possible to take ownership of its environment with the keyword
  // `move`

  let num = 6;
  let owns_num = move |x: i32| x + num;

  // Even tough the keyword is `move` the variables follow normal move semantics.
  // In this case 6 implements Copy, and so owns_num takes ownership of
  // copy of num.

  let mut num2 = 2;
  {
    let mut add_num = |x: i32| num2 += x;
    add_num(50);
  }
  assert_eq!(52, num2);

  // In this case, closure took a mutable reference to num, and then when
  // we called add_num, it mutated underlying value, as expected.
  // add_num is declared as mut too, because of mutating environment.

  let mut num5 = 5;
  {
    let mut add_num5 = move |x: i32| num5 += x;

    add_num5(5);
  }

  assert_eq!(5, num5);
  // We only get 5. Rather than taking a mutable borrow out on our num, we took
  // ownership of a Copy.

  // Another way to think about `move` closures: they give a closure its own
  // stack frame. Without move, a closure may be tied to the stack frame that
  // created it, while a move closure is self-contained. This means that you
  // cannot generally return a non-move closure from a function, for example.
}

fn closure_implementation() {
  //  In Rust, we use the trait system to overload operators.
  //  Calling functions is no different
  // pub trait Fn<Args> : FnMut<Args> {
  //   extern "rust-call" fn call(&self, args: Args) -> Self::Output;
  // }
  //
  // pub trait FnMut<Args> : FnOnce<Args> {
  //   extern "rust-call" fn call_mut(&mut self, args: Args) -> Self::Output;
  // }
  //
  // pub trait FnOnce<Args> {
  //   type Output;
  //
  //   extern "rust-call" fn call_once(self, args: Args) -> Self::Output;
  // }

  // This covers all three kinds of self via the usual method call syntax.
  // But we've split them up into three traits, rather than having a single one.
  // This gives us a large amount of control over what kind of closures we can take.

  // The || {} syntax for closures is sugar for these three traits.
  // Rust will generate a struct for the environment, impl the appropriate trait, and then use it.
}

fn taking_closures_as_arguments() {
  // We know that closures are traits, we already know how to accept and return closures.
  // We can choose static vs dynamic dispatch as well.

  // fn call_with_one<F>(some_closure: F) -> i32
  //   where F : Fn(i32) -> i32 {
  //
  //   some_closure(1)
  // }
  //
  // let answer = call_with_one(|x| x + 2);
  //
  // assert_eq!(3, answer);
  // Because Fn is a trait, we can bound our generic with it.
  // In this case, our closure takes a i32 as an argument and returns an i32,
  // and so the generic bound we use is Fn(i32) -> i32.
  //
  // In Rust, we can stack allocate our closure environment, and statically dispatch the call.
  // This happens quite often with iterators and their adapters,
  // which often take closures as arguments.
}

fn returning_closures() {
  fn factory() -> Box<Fn(i32) -> i32> {
    let num = 4;

    Box::new(move |x| x + num)
  }

  let f = factory();

  let answer = f(4);

  assert_eq!(8, answer);
}
