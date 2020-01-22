// adds the standard io lib to the current namespace
use std::io;

// using the rand package, not sure how Rng is added, why does stdin need io
// but thread uses rand::, and not Rng::
// apparently in this context, ::Rng "trait" defines the the methods that
// rand will implement. I think capitalized postfixes are traits
use rand::Rng;

use std::cmp::Ordering;

fn main() {
    let secret_num = rand::thread_rng().gen_range(1, 101);

    println!("The secret is: {}", secret_num);

    loop {
        println!("guess a number:");

        // let is the notation to create a variable
        // mut makes the variable mutable, default is immutable
        // in this case String is a type, and ::new() is a method of the String type
        let mut guess = String::new();
        // the string in read_line has to be mutable, because the read_line fn
        // changes the variable of the reference that is passed in
        io::stdin().read_line(&mut guess)
            // readline changes the mutable string, but is also returns with a type of io::Result
            .expect("couldn't read that");

        // overwrite guess, and define it as an unsigned 32bit int. trim() removes
        // whitespace and /n char to leave just the string. parse() turns the string
        // into a number
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("you guessed: {}", guess);

        // match expression decides what to do next based on what guess.cmp(secret) matches
        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Too low"),
            Ordering::Greater => println!("Too High"),
            Ordering::Equal => {
                println!("You Win");
                break;
            }
        }
    }

}
