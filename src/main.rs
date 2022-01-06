// use std::io library
use std::io;

// declares a new function
fn main(){
    // print
    println!("guess number");
    println!("input guess");

    // create a place to store the user input
    // use mut before variable make a variable mutable
    // immutable variable can not be assign twice
    // variable is immutable by default
    // mutable means allow the value be change
    // different with mut and shadow - mut not allow to mutate a variable type
    // shadow - use keyword of let, we can change the type of value, and reuse the same variable
    // String::new - a function return a new, empty instance of string
    let mut guess = String::new();

    // stdin function from io module, return an instance of std::io::Stdin
    io::stdin()
        // read_line method - get input from user
        // job for read_line take user type into input and append a string
        // & - indicates reference, reference are immutable by default
        .read_line(&mut guess)
        // handle potential failure with result
        .expect("Failed to read line");

    // {} is a placeholder, first set of curly brackets holds the first value, ans so on
    println!("You guessed {}", guess );

    // first part - get input from keyboard and print it
    // next part - generate a secret number - user will try to guess
    // secret number - should different every time, use a random number between 1 - 100

}