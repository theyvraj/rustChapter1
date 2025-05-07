use std::io; // the io library comes from the standard library called std.
use rand::Rng; // Rng is a trait that defines methods for generating random numbers
use std::cmp::Ordering; /*Ordering is an enum that represents the result of a comparison
Ordering type is another enum and has the varients 
Less, Greater and Equal. */ 

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=10);
/* gen_range method is inclusive meaning start and end are included
thread_rng() returns a random number generator
that is local to the current thread of execution. */  
    loop {   
        println!("Please input your guess.");
        let mut guess = String::new(); 
    /*this returns a new instance of a String,
    :: syntax in the ::new line indicates that new is an associated function
    of the string type. An assosicated function is a function that is implemented
    on a type.*/
        io::stdin() 
    /*the stdin fuction returns a function returns 
    an instance of std::io::Stdin, which is a type that 
    represents a handle to the standard input for your termminal. */
            .read_line(&mut guess)
    /* the & symbol indicates that guess is a reference to the string 
    the .read_line method is a 'Result' type. 
    Values of a result type, like values of any type, have methods defined on them. */
            .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number!");
                continue; // this will skip the rest of the loop and start over
            }
        };
    /*  removes whitespace from the beginning and end of the string
    which was caused when pressing Enter to satisfy the read_line. */
    // converts the string to a number
            
    /* if it fails, it will panic with this message
    we shadowed the guess variable we initialized in line 15. */
        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
/* a match expression is made up of *arms*. 
An arm consists of a pattern to match against,
and the code that should be run if the value given to match
fits that arm's pattern. */

/* each possible state of an enum is called a varient. */
}