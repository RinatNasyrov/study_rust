use rand::prelude::*;

fn main() {
    let foo = 69;
    let mut r: &i32;

    {
        let x = 42;
        // r = &x; - так не пойдет, время жизни r больше, чем у x
        // println!("{}", *r);
    }

    if random_bool() {
        r = &foo;
    }

    // println!("{}", *r);
}

fn random_bool() -> bool {
    rand::rng().random_range(1..=10) % 2 == 0
}
