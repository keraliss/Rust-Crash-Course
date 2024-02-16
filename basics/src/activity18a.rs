// Topic: Result
//
// Requirements:
// * Create an structure named `Adult` that represents a person aged 21 or older:
//   * The structure must contain the person's name and age
//   * Implement Debug print functionality using `derive`
// * Implement a `new` function for the `Adult` structure that returns a Result:
//   * The Ok variant should contain the initialized structure, but only
//     if the person is aged 21 or older
//   * The Err variant should contain a String (or &str) that explains why
//     the structure could not be created
// * Instantiate two `Adult` structures:
//   * One should be aged under 21
//   * One should be 21 or over
// * Use `match` to print out a message for each `Adult`:
//   * For the Ok variant, print any message you want
//   * For the Err variant, print out the error message

pub fn run() {
    let child = Adult::new("Child".to_string(), 10);
    let john = Adult::new("John".to_string(), 23);

    match child {
        Ok(c) => println!("{} is {} years old", c.name, c.age),
        Err(e) => println!("{:?}", e),
    }
    match john {
        Ok(j) => println!("{} is {} years old", j.name, j.age),
        Err(e) => println!("{:?}", e),
    }
}

#[derive(Debug)]
struct Adult {
    name: String,
    age: u8
}

impl Adult {
    fn new (name: String, age: u8) -> Result<Self, String> {
        if age < 21 {
            Err(String::from("Not old enough"))
        }
        else {
            Ok(Self { name: name.to_string(), age })
        }
    }
}