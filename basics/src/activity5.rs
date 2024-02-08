// Topic: Looping using the loop statement
//
// Program requirements:
// * Display "1" through "4" in the terminal
//
// Notes:
// * Use a mutable integer variable
// * Use a loop statement
// * Print the variable within the loop statement
// * Use break to exit the loop

pub fn run() {
    let mut num = 1;
    loop {
        if num > 4 {
            break;
        }
        println!("{}", num);
        num += 1;
    }
} 