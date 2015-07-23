fn main() {
  // Generics are called parametric polymorphism in type theory,
  // which means that they are types or functions that have multiple forms
  // (poly is multiple, morph is form) over a given parameter (parametric).

  enum Option<T> {
    Some(T),
    None,
  }
  // The <T> part, which you've seen a few times before,
  // indicates that this is a generic data type.
  // Inside the declaration of our enum, wherever we see a T,
  // we substitute that type for the same type used in the generic.

  // This type is generic over two types: T and E.
  enum Result<A, Z> {
    Ok(A),
    Err(Z),
  }

  let x: Result<f64, String> = Ok(2.3f64);
  let y: Result<f64, String> = Err("There was an error.".to_string());

  fn inverse(x: f64) -> Result<f64, String> {
    if x == 0.0f64 { return Err("x cannot be zero!".to_string()); }

    Ok(1.0f64 / x)
  }

  let x = inverse(25.0f64);

  match x {
      Ok(x) => println!("The inverse of 25 is {}", x),
      Err(msg) => println!("Error: {}", msg),
  }
}
