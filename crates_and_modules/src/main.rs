mod english {
    mod greetings {
        fn hello() -> String {
            return "Hello !".to_string();
        }
    }
    mod farewells {

        // Rust allows you to precisely control which
        // aspects of your interface are public,
        // and so private is the default.
        // To make things public, you use the pub keyword.
        pub fn cya() -> String {
            return goodbye();
        }

        fn goodbye() -> String {
            return "Goodbye".to_string();
        }
    }
}

fn main() {
    // Module names follow the conventions
    // for other Rust identifiers: lower_snake_case.

    // We can refer to sub-modules with double-colon (::)
    // notation: our four nested modules

    // To import
    // use phrases::english::{greetings, farewells};
    // extern crate phrases;

    // use phrases::english::{greetings,farewells};
    // use phrases::japanese;
}
