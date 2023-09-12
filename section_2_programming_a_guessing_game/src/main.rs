/* To obtain user input and then print the result as output,
we need to bring the io input/output library into scope.
The io library comes from the standard library, known as std:*/

/*By default, Rust has a set of items defined in the standard library that it
brings into the scope of every program. This set is called the prelude */
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let number = rand::thread_rng().gen_range(1..=100); //Using random crate

    // println!("The scecret number is: {number}"); //Printing secret number

    // By default, every variable is immutable.. Use mut to make it mutable
    /* The :: syntax in the ::new line indicates that new is an associated
    function of the String type. An associated function is a function that’s
    implemented on a type, in this case String. This new function creates a new,
    empty string. You’ll find a new function on many types because
    it’s a common name for a function that makes a new value of some kind. */
    loop {
        println!("Please input your guess.");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess) // Reeturns a result .. Either Ok or Err
            .expect("Failed to read the line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
