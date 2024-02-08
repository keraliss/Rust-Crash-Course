// Topic: Basic arithmetic
//
// Program requirements:
// * Displays the result of the sum of two numbers
//
// Notes:
// * Use a function to add two numbers together
// * Use a function to display the result
// * Use the "{:?}" token in the println macro to display the result

pub fn run() {
    let result = sum(1, 2);
    println!("The result is {:?}", result);
}

fn sum(x: i32, y: i32) -> i32 {
    x + y
}