// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function

pub fn run() {
    let people = vec![
        Person {
            name: String::from("John"),
            age: 10,
            favorite_color: String::from("blue"),
        },
        Person {
            name: String::from("Jane"),
            age: 9,
            favorite_color: String::from("green"),
        },
        Person {
            name: String::from("Jack"),
            age: 12,
            favorite_color: String::from("red"),
        },
    ];

    for i in people{
        if i.age <= 10 {
            print_name(&i.name);
            println!("Age: {}", i.age);
            print_color(&i.favorite_color);
        }
    }
}

fn print_name(name: &str) {
    println!("Name: {}", name)
}
fn print_color(color: &str) {
    println!("Color: {}", color)
}

struct Person {
    name: String,
    age: i32,
    favorite_color: String,
}
