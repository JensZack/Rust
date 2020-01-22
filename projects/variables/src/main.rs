use std::str;

fn first_word(s: &String) -> &str {
    // return the index for the last letter (or first space) in the argument string
    let bytes = s.as_bytes();
    // let str = str::from_utf8(bytes).expect("why?");

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

fn main() {

    // basic chars are contained by single quotes, and can take any value in
    // unicode scaler values.
    let c = 'c';

    // basic tuple, can contain different types, but the length is fixed
    let tup: (i32, f64, u8) = (-55, 39.443, 5);
    let x: f64 = tup.1;

    // Array can only have elements of the same type, and size is fixed
    // two ways to declare an array
    let mut extra = true;
    if extra { let a: [i32; 5] = [1, 2, 3, 4, 5]; }
    else { let a = [1, 2, 3, 4, 5]; }

    let range = 1..5;

    let mut s: String = String::from("dogs and cats");
    let strFirst = first_word(&s);
    println!("The first word in s is: {}", strFirst);

    // s.insert(1, 'a');  Breaks because there is an immutable reference to s
    // println!("The new first word is: {}", strFirst);
}

