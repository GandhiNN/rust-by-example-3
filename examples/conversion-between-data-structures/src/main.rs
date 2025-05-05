use std::num::ParseIntError;

struct Number {
    value: i32,
}

impl TryFrom<String> for Number {
    type Error = ParseIntError;

    fn try_from(source: String) -> Result<Self, Self::Error> {
        Ok(Number {
            value: source.parse()?,
        })
    }
}

struct A {
    member: String,
}

struct B {
    value: String,
}

impl From<A> for B {
    fn from(source: A) -> Self {
        Self {
            value: source.member,
        }
    }
}

fn main() {
    let a = A {
        member: String::from("something"),
    };
    let b = B::from(a);

    let a2 = A {
        member: String::from("hehe"),
    };
    let b2: B = a2.into();

    println!("{}", b.value);
    println!("{}", b2.value);

    match Number::try_from(String::from("42")) {
        Ok(n) => {
            println!("{}", n.value);
        }
        Err(e) => {
            println!("Conversion failed {:?}", e);
        }
    }
}
