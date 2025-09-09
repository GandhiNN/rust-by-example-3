#![allow(unused_must_use)]
#![allow(unused_variables)]
macro_rules! add {
    // match-like arm for macro
    // first arm match add!(1, 2), add!(2, 3) etc
    // $a = metavariable, expr = fragment-specifier
    // explanation = https://doc.rust-lang.org/reference/macros-by-example.html
    ($a:expr, $b:expr) => {
        // macro expand to this code
        // $a and $b will be templated using the value/variable provided to macro
        { $a + $b }
    };
    // second arm match add!(1), add!(2), etc
    ($a:expr) => {{ $a }};
}

fn main() {
    // Call to macro, $a=1 and $b=2
    add!(1, 2);

    let x = 0;
    add!(2, 3);
    add!(9);
}
