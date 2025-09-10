// Basic declarative macros
//
// In the body of the macro, there's one pattern
// ($x: expr) => { ... }
// This pattern matches any Rust expression (expr) and stores it in a variable $x.
// Upon matching the pattern, the $x placeholder is replaced by the value of $x
// and the output of the macro becomes println!("Hello, {}", $x);
//
macro_rules! greetings {
    ($x: expr) => {
        println!("Hello, {}", $x);
    };
}

// It's possible to have multiple patterns just like a `match` expression.
// Here, the first arm matches the code of the form `struct X {}`
// and replaces it with println!("Hello from struct {}", "X").
// The `ident` type is used to indicate that $x is an identifier
// (name of the struct). For any other code, the other arm is matched
// just like it was in the previous example.
macro_rules! greetings_multiple {
    (struct $x: ident {}) => {
        println!("Hello from struct {}", stringify!($x))
    };
    ($x: expr) => {
        println!("Hello, {}", $x);
    };
}

// It's also possible to match repeated expressions
// using a special syntax.
// Here, the type of `$b` is `tt` which represents a token tree.
// In the macro body, `add!` is used recursively to add
// the inputs in $b. Here, the `*` denotes that the code
// should be generated for each part that matches the
// $()* in the arm. This means that the macro call `add(1,2,3)`
// is expanded to `add(1,add(2,3))` which is further expanded
// to `add(1, add(2, add(3)))` and finally becomes `1+2+3`
macro_rules! add {
    ($a:expr) => {
        $a
    };
    ($a:expr,$b:expr) => {
        $a+$b
    };
    ($a:expr,$($b:tt)*) => {
        $a+add!($($b)*)
    }
}

fn main() {
    // we can use parentheses or curly brackets
    greetings!("Earthly");
    greetings! {"HAHAHA"};

    // call the macro greetings_multiple using a struct
    // as its arguments
    greetings_multiple!(
        struct G {}
    );

    // Call the macro add
    let t = add!(2, 3, 4, 5);
    println!("{t}");
}
