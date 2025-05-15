/*
fn main() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {result}");
}

fn main() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 2 {
                break;
            }
            if count == 3 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");
}

fn main() {
    let mut number = 3;
    let mut i = 0;
    while number > i {
        println!("nice sits");
        i += 1;
        
    }
}
fn main() {
    let a: [i32; 5] = [43, 12, 67, 38, 22];
    for element in a {
        println!("The value of current value is {element}");
    }
}

fn main() {
    let a: [i32; 5] = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("The value of current value is {}", a[index]);
        index += 1;
    }
}
fn main() {
    for i in (1..4).rev() {
        println!("{i}");
    }
    println!("LIFTOFF");
}

fn main() {
    for i in 1..4 {
        println!("{i}");
    }
    println!("LIFTOFF");
}
*/
fn main() {}