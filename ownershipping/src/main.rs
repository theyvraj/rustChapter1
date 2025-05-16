/* un-healing - https://www.youtube.com/watch?v=_Qr2T1az1Ck
healing - https://www.youtube.com/watch?v=XDX4ZwUeOok
fn main() {
    let s: String = String::from("Hello");
    println!("s is {}", s);
}
 fn main() {
     let x = 6;
     let y = x;
     println!("value of x and y is {} and {}",x, y);
 }


A string is made up of three parts :
    STACK               HEAP
1. Pointer (ptr) -> index | Value
2. Length (len)         0   h
3. Capacity (cap)       1   e
                        2   l
                        3   l
                        4   o
length is how much memory in bytes is the content of the string are currently using.
capacity is the total amount of memory in bytes the string has received from the allocator.

fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {s1}, s2 = {s2}");
    // let s1 = String::from("hello");
    // let s2 = s1;
    // println!("{s1},{s2}");
    let x = 5;
    let y = x;
    println!("x = {x}, y = {y}");
}

fn take_ownership(some_string: String) {
    println!("{}", some_string);
}
fn make_copy(some_integer: i32) {
    println!("{}", some_integer);
}
fn main() {
    let s = String::from("hello");
    take_ownership(s.clone());
    let x = 5;
    make_copy(x);
}

use std::mem::take;

// fn main() {
//     let s1 = give_back();
//     let s2 = String::from("hello");
//     let s3 = take_back(s2);
//     println!("{}, {}", s1, s3);
// }
//
fn give_back() -> String {
    let some_string = String::from("yours");
    some_string
}
fn take_back(a_string: String) -> String {
    a_string
}
fn main() {
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{s2}' is {len}.");
}
fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String
    (s, length)
}
this is called tuple code

 fn main() {
     let s1 = String::from("hello");
     let s1_len = calculate_len(&s1);
     println!("the length of {} is {}", s1, s1_len);
 }
fn calculate_len(s: &String) -> usize {
    s.len()
}

fn main() {
    let mut s = String::from("hello");
    change(&mut s);
    println!("string value is {}", s);

}
fn change(some_string: &mut String) {
    some_string.clear();
}

fn main() {
    let mut s = String::from("hello");
    let r1 = &mut s;
    r1.push_str(", world");
    println!("{}", r1);
    let r2 = &mut s;
    r2.push_str(", batman");
    println!("{}", r2);
}

fn main() {
    let mut s = String::from("hello");

    let mut original = s.clone();  // Clone the original value

    let r1 = &mut s;
    r1.push_str(", world");
    println!("{}", r1);        // Prints: hello, world

    let r2 = &mut original;     // Use the unmodified clone
    r2.push_str(", batman");
    println!("{}", r2);        // Prints: hello, batman
}

fn main() {
    let mut s = String::from("hello");
    {
        let r1 = &mut s;
        r1.push_str(" there");
        println!("r1={}", r1);
        
    }
    {
        let r2 = &mut s;
        r2.push_str(" human");
        println!("r2={}", r2);
    }
}
*/
fn main() {
    
}