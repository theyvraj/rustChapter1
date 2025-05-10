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

// }

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
