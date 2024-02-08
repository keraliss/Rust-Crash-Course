// Topic: Functions
//
// Program requirements:
// * Displays your first and last name
//
// Notes:
// * Use a function to display your first name
// * Use a function to display your last name
// * Use the println macro to display messages to the terminal

pub fn run() {
    println!("My name is {}{}!", first_name(), last_name());
}
fn first_name() -> String {
    String::from("Kera")
}

fn last_name() -> String {
    String::from("liss")
}

