// variables1.rs
// Make me compile! Execute the command `rustlings hint variables1` if you want a hint :)

// About this `I AM NOT DONE` thing:
// We sometimes encourage you to keep trying things on a given exercise,
// even after you already figured it out. If you got everything working and
// feel ready for the next exercise, remove the `I AM NOT DONE` comment below.

// I AM NOT DONE

fn main() {
    let x = 5;
    println!("x has the value {}", x);

    // if you change the variable it will blow up cuz x is
    // immutable. So, lets make a mutable one.
    let mut y = "reeeeeeeeeeeeeeeeeee";
    println!("y contains the string '{}'", y);
    y = "Hello Darkness my old friend";
    println!("y now contains the string '{}'", y);
}
