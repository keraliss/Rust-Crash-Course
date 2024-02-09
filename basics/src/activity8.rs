// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor

pub fn run() {
    let sweet = Drink {
        flavor: Flavor::Sweet,
        fluid_oz: 12.0,
    };
    let sparkling = Drink {
        flavor: Flavor::Sparkling,
        fluid_oz: 6.0,
    };
    let sour = Drink {
        flavor: Flavor::Sour,
        fluid_oz: 8.0,
    };
    print_drink(sweet);
    print_drink(sparkling);
    print_drink(sour);
}


fn print_drink(drink: Drink) {
    match drink.flavor {
        Flavor::Sparkling => {
            println!("Flavor: Sparkling");
        }
        Flavor::Sweet => {
            println!("Flavor: Sweet");
        }
        Flavor::Sour => {
            println!("Flavor: Sour");
        }
    }
    println!("{} oz", drink.fluid_oz);
}

enum Flavor {
    Sparkling,
    Sweet,
    Sour,
}

struct Drink {
    flavor: Flavor,
    fluid_oz: f32,
}
