// use std::io;
// fn main() {
//     println!("Enter your index position : ");
//     let a: [&str; 5] = ["what", "is", "your", "name", "human?"];
//     let mut index = String::new();

//     io::stdin()
//         .read_line(&mut index)
//         .expect("Failed to read line.");

//     let index: usize = index.trim()
//                             .parse()
//                             .expect("Index entered was not a number.");
//     let element = a[index];
//     println!("The value of the element on index position {}, is {}", index, {element});

/*
use std::io;

fn main() {
    println!("Enter your index position: ");
    let a: [&str; 5] = ["what", "is", "your", "name", "human?"];
    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line.");

    let index: Result<usize, _> = index.trim().parse();

    match index {
        Ok(i) => match a.get(i) {
            Some(element) => println!("The value at index position {} is '{}'", i, element),
            None => println!("Index {} is out of bounds. Please enter a value between 0 and {}.", i, a.len() - 1),
        },
        Err(_) => println!("Invalid input: please enter a valid number."),
    }
}
// FUNCTIONS
fn main() {
    println!("Hello World");
    let sum = add_numbers(3, 8);
    println!("the sum is {}", sum);
    
}

fn add_numbers(x: i32, y: i32) -> i32 {
    return x+y
}
// STATEMENTS AND EXPRESSIONS
fn main() {
    let y = {
        let x = 7;
        x + 1
    };
    println!("The value of y is {y}");
    let y = five();
    println!("The value of y is {y}");
    let z = plus_one(8);
    println!("The value of z is {z}");
}
fn five() -> i32 {
    5
}
fn plus_one(x: i32) -> i32 {
    return x + 1;
}
fn main() {
    let x = 5;
    println!("{x}");
}
// CONTROL FLOW
fn main() {
    let number = 3;
    if number < 5 {
        println!("Condition was true");
    } else {
        println!("Condition was not met");
    }
}
fn main() {
    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 2, 3 or 4");
    }
}
*/
fn main() {
    let condition = false;
    let number = if condition {5} else {6};
    println!("the value of the number is {number}");
}
