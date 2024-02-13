pub fn run() {
    let c = add(5, 15);
    println!("c = {c}");

    let a = 99;
    if a > 98 {
        println!("big");
    } else {
        println!("small");
    }
    let b = 80;
    if b > 100 {
        if b > 200 {
            println!("huge number");
        } else {
            println!("small number");
        }
    } else {
        println!("very small number");
    }

    let d = 150;
    if d > 30 {
        println!("big number");
    } else if d > 20 {
        println!("medium number");
    } else {
        println!("small number");
    }

    let mut e = 0;
    let mut f = 0;
    loop {
        if e == 5 {
            break;
        }
        println!("loop e = {e}");
        e = e + 1;
    }

    while f < 10 {
        println!(" while f = {f}");
        f = f + 1;
    }

    //    all arithmatic functions
    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let multiplication = 4 * 30;
    let div = 56.7 / 32.2;
    let remainder = 43 % 5;

    println!("sum = {sum}");
    println!("difference = {difference}");
    println!("multiplication = {multiplication}");
    println!("div = {div}");
    println!("remainder = {remainder}");

    let some_bool = true;
    match some_bool {
        true => println!("true"),
        false => println!("false"),
    }

    let some_int = 3;
    match some_int {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("other"),
    }

    let go = Direction::Right;
    match go {
        Direction::Up => println!("go up"),
        Direction::Down => println!("go down"),
        Direction::Left => println!("go left"),
        Direction::Right => println!("go right"),
    }

    let cerial = GroceryItem {
        stock: 10,
        price: 99.99,
    };
    println!("stock = {}", cerial.stock);
    println!("price = {}", cerial.price);

    let coordinate = (3, 5);
    println!("{},{}", coordinate.0, coordinate.1);

    let (x, y) = (2, 3);
    println!("{},{}", x, y);

    let user_info = ("John", "Doe", 32);
    println!(
        "Name : {} {}, Age : {}",
        user_info.0, user_info.1, user_info.2
    );

    let (name, age) = ("Emma", 19);
    println!("Name : {}, Age : {}", name, age);

    // only for admin
    let access_level = Access::Guest;
    let can_access_file = match access_level {
        Access::Admin => true,
        _ => false,
    };

    println!("can_access_file = {can_access_file}");

    let dull = Light::Dark;
    display_light(&dull);
    display_light(&dull);

    let book = Book {
        pages: 100,
        rating: 5,
    };
    display_page_count(&book);
    display_rating(book);
    // println!("{}", book.pages);

    let hot = Temperature { degrees_f: 99.9 };
    hot.show_temp();
    let cold = Temperature::freezing();
    cold.show_temp();
    let boil = Temperature::boiling();
    boil.show_temp();
}

struct Temperature {
    degrees_f: f64,
}

impl Temperature {
    fn show_temp(&self) {
        println!("temp = {}", self.degrees_f);
    }
    fn freezing() -> Self {
        Self {
            degrees_f: 32.0,
        }
    }
    fn boiling () -> Self {
        Self {
            degrees_f: 212.0,
        }
    }
}

struct Book {
    pages: i32,
    rating: i32,
}

fn display_page_count(book: &Book) {
    println!("pages = {}", book.pages);
}

fn display_rating(book: Book) {
    println!("rating = {}", book.rating);
}

enum Light {
    Bright,
    Dark,
}

fn display_light(light: &Light) {
    match light {
        Light::Bright => println!("bright"),
        Light::Dark => println!("dark"),
    }
}

enum Access {
    Admin,
    Manager,
    User,
    Guest,
}

struct GroceryItem {
    stock: i32,
    price: f32,
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}
