// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

pub fn run() {
    let student1 = Student {
        name: String::from("John"),
        locker: None,
    };
    let student2 = Student {
        name: String::from("Jane"),
        locker: Some(1),
    };

    match student1.locker {
        Some(locker) => println!("{} has a locker {}", student1.name, locker),
        None => println!("{} has no locker", student1.name),
    }

    match student2.locker {
        Some(locker) => println!("{} has a locker {}", student2.name, locker),
        None => println!("{} has no locker", student2.name),
    }
}

struct Student {
    name: String,
    locker: Option<i32>,
}
