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
*/
fn main() {
    let s1 = give_back();
    let s2 = String::from("hello");
    let s3 = take_back(s2);
    println!("{}, {}", s1, s3);
}

fn give_back() -> String {
    let some_string = String::from("yours");
    some_string
}
fn take_back(a_string: String) -> String {
    a_string
}