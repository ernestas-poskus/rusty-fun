fn main() {

  for x in 1..10 {
    println!("{}", x);
  }

  // An iterator is something that we can call the .next() method on repeatedly,
  // and it gives us a sequence of things.

  let mut num_range = 0..5;

  loop {
    match num_range.next() {
      Some(x) => {
        println!("{}", x);
      },
      Node => { break }
    }
  }
  // we make mutable binding to the num_range, which is iterator
  // then loop with inner match
  // match is result of num_range.next() which gives reference to the next
  // value of the iterator next() returns an Option<i32> which will become
  // Some(i32) when we have a value and None once we run out.
  // The for loop is just a handy way to write this loop/match/break construct.

  let nums = vec![1, 2, 3];

  for num in &nums {
      println!("{}", num);
  }
  // iterate through items directly, avoid indexing
  // with references, we're just borrowing a reference to the data,
  // and so it's just passing a reference, without needing to do the move.


  // iterators - gives you a sequence of values
  // iterator apapters - operate on iterators, producing a new iterator with a different output sequence.
  // consumers - operate on an iterator, producing some final set of values

  consumers();
  iterators();
  iterator_adapters();
  combo();
}

fn consumers() {

  // - .collect()
  let one_to_one_hundred = (1..101).collect::<Vec<i32>>();

  // partial hint
  let one_to_one_hundred2 = (1..101).collect::<Vec<_>>();

  // - .find()
  // takes a closure and works on a reference to each element of an iterator
  let greater_than_forty_two = (0..100).find(|x| *x > 42);

  match greater_than_forty_two {
      Some(_) => println!("We got some numbers!"),
      None => println!("No numbers found :("),
  }

  // - .fold()
  // looks like fold(base, |accumulator, element| ...)
  // Upon each iteration, the closure is called,
  // and the result is the value of the accumulator on the next iteration.
}

fn iterators() {
  // (1..).step_by(5);
  // This iterator counts up from one, adding five each time.
  // It will give you a new integer every time, forever
  // (well, technically, until it reaches the maximum number representable by an i32)
}

fn iterator_adapters() {
  // (1..100).map(|x| x + 1);
  // map is called upon another iterator, and produces a new iterator
  // where each element reference has the closure it's been given as an argument called on it

  // for i in (1..).step_by(5).take(5) {
  //   println!("{}", i);
  // }

  for i in (1..100).filter(|&x| x % 2 == 0) {
    println!("{}", i);
  }
}

fn combo() {
  let nums = (1..1000)
    .filter(|&x| x % 2 == 0)
    .filter(|&x| x % 3 == 0)
    .take(5)
    .collect::<Vec<i32>>();

  for num in &nums {
    println!("num {}", num);
  }
}
