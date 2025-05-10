// fn hours_to_mins(x : i32) -> i32 {
//     return x * 60
// }
// fn main() {
//     let mut x = 5;
//     println!("The value of x is: {x}");
//     x = 6;
//     println!("The value of x is: {x}");
//     const THREE_HOURS_IN_MINUTES: i32 = 3 * 60;
//     println!("3 hours in minutes is : {}", hours_to_mins(3));
//     println!("Constant 3 hours into minutes is : {}", THREE_HOURS_IN_MINUTES);
// }
// Shadowing
// fn main() {
//     let x = 5;
//     let x = x + 1;
//     {
//         let x = x * 2;
//         println!("The value of x in inner scope is: {x}");
//     }
//     println!("The value of x in outer scope is: {x}");
// }
// fn main() {
//     let spaces = "    ";
//     let spaces = spaces.len();
// /*using let to shadow the previous variable,
// mut wont work here as we will get compile time error,
// we are not allowed to mutate a variable's type*/
//     println!("The length of spaces is: {spaces}");
// }
// Data Types
// fn main() {
//     let guess = "42".parse().expect("Not a number!");
//     println!("The guess is: {guess}");
// }
// Scalar Types
/* there are four primary scaler tpes
integers, floating-point numbers, booleans and characters. */
/*INTEGER TYPES 
i means signed and u means unsigned
signed means it can be negative or positive
unsigned means it can only be positive
Length Signed Unsigned
8-bit i8 u8
16-bit i16 u16
32-bit i32 u32
64-bit i64 u64
128-bit i128 u128
arch isize usize */

// use plotters::prelude::*;

// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let root = BitMapBackend::new("precision_error.png", (800, 600)).into_drawing_area();
//     root.fill(&WHITE)?;
//     let mut chart = ChartBuilder::on(&root)
//         .caption("Absolute Error of f32 vs f64 (Multiples of π)", ("sans-serif", 25))
//         .margin(20)
//         .x_label_area_size(40)
//         .y_label_area_size(60)
//         .build_cartesian_2d(0f64..100f64, 0f64..1e-6)?;

//     chart.configure_mesh().draw()?;

//     let pi = std::f64::consts::PI;

//     let f32_errors: Vec<(f64, f64)> = (1..=100)
//         .map(|i| {
//             let x = pi * i as f64;
//             let x_f32 = x as f32 as f64;
//             let error = (x - x_f32).abs();
//             (x, error)
//         })
//         .collect();

//     let f64_errors: Vec<(f64, f64)> = (1..=100)
//         .map(|i| {
//             let x = pi * i as f64;
//             let x_f64 = x; // No casting, so error should be ~0
//             let error = (x - x_f64).abs();
//             (x, error)
//         })
//         .collect();

//     chart
//         .draw_series(LineSeries::new(f32_errors, &RED))?
//         .label("f32 error")
//         .legend(|(x, y)| Path::new(vec![(x, y), (x + 10, y)], &RED));

//     chart
//         .draw_series(LineSeries::new(f64_errors, &BLUE))?
//         .label("f64 error")
//         .legend(|(x, y)| Path::new(vec![(x, y), (x + 10, y)], &BLUE));

//     chart.configure_series_labels().background_style(&WHITE).draw()?;

//     println!("Plot saved as 'precision_error.png'");
//     Ok(())
// }
// fn main() {
//     let x: f32 =2.5656465465465654654654;
//     let y: f64 = 2.5656465465465654654654;
//     let z = 2.5656465465465654654654;
//     println!("The value of x, y, z is: {x}, {y}, {z} respectively.");
// }

/*NUMERIC OPERATIONS


fn main() {
    let x = 5.5464;
    let y = 10.23544565766;
    let sum = x + y;
    let diff = x - y;
    let prod = x * y;
    let quot = x / y;
    let rem = x % y;
    println!("The sum of {x} and {y} is: {sum}");
    println!("The difference of {x} and {y} is: {diff}");
    println!("The product of {x} and {y} is: {prod}");
    println!("The quotient of {x} and {y} is: {quot}");
    println!("The remainder of {x} and {y} is: {rem}");z
} */

/*BOOLEAN TYPE 
fn main() {
    let f: bool = false;
    println!("The value of f is: {f}");
} */

/*CHARACTER TYPE 
fn main() {
    let z: char = 'Z';
    let heart_eyed_cat = '😻';
    println!("The value of z, heart_eyed_cat is: {z}, {heart_eyed_cat}");
} */

/*COMPOUND TYPES
The Tuple types
 
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // let(x, y, z) = tup;
    // println!("The value of x, y, z is: {x}, {y}, {z}");
    println!("The value of x, y, z is: {}, {}, {}", tup.0, tup.1, tup.2);
} 


The Array types
*/
fn main() {
//    let months: [&String; 12] = ["January", "February", "March", "April", "May", "June", "July",
// "August", "September", "October", "November", "December"];
//    println!("The value of months is: {:?}", months);

let a: [i8; 5] = [1, 2, 3, 4, 5];
println!("The value of a is: {:?}", a);

let first: i8 = a[0];
let second: i8 = a[1];
println!("The value of first and second elemets respectively are {}, {}", first, second);


}

