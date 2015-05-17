fn main() {
    println!("Method syntax");

    // Struct methods and chaining
    struct Circle {
      x: f64,
      y: f64,
      radius: f64,
    }

    impl Circle {
      fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
      }
    }

    let c = Circle { x: 21.22, y: 0.0, radius: 10.0 };
    println!("{}", c.area());

    impl Circle {
      fn reference(&self) {
        println!("taking self by reference");
      }

      fn mutable_reference(&mut self) {
        println!("taking self by mutable reference");
      }

      fn takes_ownership(self) {
        println!("taking ownership of self");
      }
    }

    // chaining method calls
    impl Circle {
      fn grow(&self, increment: f64) -> Circle {
        Circle { x: self.x, y: self.y, radius: self.radius + increment }
      }
    }

    let d = c.grow(2.34444).area();
    println!("{}", d);

    // static methods
    impl Circle {
      fn new(x: f64, y: f64, radius: f64) -> Circle {
        Circle {
          x: x,
          y: y,
          radius: radius,
        }
      }
    }

    let b = Circle::new(0.0, 0.0, 2.0);

    // Builder Pattern
    // Let"s say that we want our users to be able to create Circles,
    // but we will allow them to only set the properties they care about.
    // Otherwise, the x and y attributes will be 0.0,
    // and the radius will be 1.0. Rust doesn"t have method overloading,
    // named arguments, or variable arguments.
    // We employ the builder pattern instead. It looks like this:
    struct CircleBuilder {
        x: f64,
        y: f64,
        radius: f64,
    }

    impl CircleBuilder {
        fn new() -> CircleBuilder {
          CircleBuilder { x: 0.0, y: 0.0, radius: 0.0, }
        }

        fn x(&mut self, coordinate: f64) -> &mut CircleBuilder {
          self.x = coordinate;
          self
        }

        fn y(&mut self, coordinate: f64) -> &mut CircleBuilder {
          self.x = coordinate;
          self
        }

        fn radius(&mut self, radius: f64) -> &mut CircleBuilder {
          self.radius = radius;
          self
        }

        fn finalize(&self) -> Circle {
          Circle { x: self.x, y: self.y, radius: self.radius }
        }
    }
}
