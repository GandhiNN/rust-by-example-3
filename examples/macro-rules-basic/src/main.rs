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

// In this code block, we use the $typ argument with token type `ty`
// as a datatype like `u8`, `u16`, etc. This macro converts to a
// particular type before adding the numbers.
macro_rules! add_as {
    // using a ty token type for matching datatypes passed to macro.
    ($a:expr, $b:expr, $typ:ty) => {
        $a as $typ + $b as $typ
    };
}

// Rust macros also support taking a nonfixed number of arguments.
// The operators are very similar to the regular expression.
// `*` is used for zero or more token types and `+` for zero or one
// argument.
//
// The token type that repeats is enclosed in `$()` followed by a separator
// and a `*` or a `+` indicating the number of times the token will repeat.
// The separator is used to distinguish the tokens from each other.
// The `$()` block followed by `*` or `+` is used to indicate the repeating
// block of code
macro_rules! add_as_nonfixed {
    (
        // repeated block
        $($a:expr),* // separated by comma and zero or more, i.e. variadic number of arguments separated by a comma
    ) => {
        {
            // to handle the case without any arguments
            0
            // block to be repeated
            $(+$a)*
        }
    };
}

// recursive macro
macro_rules! add_recursive {
    // first arm in case of single argument and last remaining variable/number
    ($a:expr) => {
        $a
    };
    // second arm in case of two arguments are passed and stop recursion in case of odd number of arguments
    ($a:expr,$b:expr) => {{ $a + $b }};
    // add the number and the result of remaining arguments
    ($a:expr,$($b:tt)*) => {
        {
            $a+add_recursive!($($b)*)
        }
    }
}

// The macro arguments don't need to be comma-separated. Multiple tokens
// can be used with different token types. For example, brackets can be
// used with the `ident` token type. The Rust compiler takes the matched
// arm and extracts the variable from the argument string.
macro_rules! ok_or_return {
    // match function_name(q, r, t, 6, 7, 8) etc
    // compiler extracts function name and arguments.
    // it injects the values in respective variables.
    ($a:ident($($b:tt)*)) => {
        {
            match $a($($b)*) {
                Ok(value) => value,
                Err(err) => {
                    return Err(err);
                }
            }
        }
    }
}

// Few macros need to be grouped into a single macro.
// In these cases, internal macro rules are used.
// It helps to manipulate the macro inputs and write
// clean TT munchers.
macro_rules! ok_or_return_internal {
    // Internal rule
    (@error $a:ident, $($b:tt)*) => {
        {
            match $a($($b)*) {
                Ok(value) => value,
                Err(err) => {
                    return Err(err);
                }
            }
        }
    };
    // Public rule can be called by the user.
    ($a:ident($($b:tt)*)) => {
        ok_or_return_internal!(@error $a,$($b)*)
    };
}

fn some_work(i: i64, j: i64) -> Result<(i64, i64), String> {
    if i + j > 2 {
        Ok((i, j))
    } else {
        Err("error".to_owned())
    }
}

fn main() -> Result<(), String> {
    // Call to macro, $a=1 and $b=2
    add!(1, 2);

    let x = 0;
    add!(2, 3);
    add!(9);

    // Call the macro add_as
    println!("add_as");
    println!("{}", add_as!(0, 2, u8));

    // Call the macro add_as_nonfixed
    println!("add_as_nonfixed");
    println!("{}", add_as_nonfixed!(1, 2, 3, 4));

    // Call the macro add_recursive
    println!("add_recursive");
    println!("{}", add_recursive!(3, 4, 5, 7));

    // Call the macro ok_or_return
    println!("ok_or_return");
    let a = ok_or_return!(some_work(1, 4));
    let b = ok_or_return!(some_work(3, 4));
    println!("{:?}", a);
    println!("{:?}", b);

    // Call the macro ok_or_return_internal
    println!("ok_or_return_internal");
    // instead of round bracket, curly brackets can also be used
    ok_or_return_internal!(some_work(1, 0));
    ok_or_return_internal! {some_work(1,4)};

    Ok(())
}
