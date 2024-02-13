// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics

pub fn run() {
   let small_dimensions = Dimensions {
       height: 1.0,
       width: 1.0,
       depth: 1.0
   };
   let small_box = Box::new(Color::Red, 1.0, small_dimensions);
   small_box.print();
}

enum Color {
    Red,
    Green,
    Blue 
}

impl Color {
   fn print(&self) {
       match self {
           Color::Red => println!("Red"),
           Color::Green => println!("Green"),
           Color::Blue => println!("Blue")
           
       }
   }
}

struct Box {
    color: Color,
    weight: f64,
    dimensions : Dimensions
}

impl Box {
    fn new(color: Color, weight: f64, dimensions: Dimensions) -> Self {
        Self {
            color,
            weight,
            dimensions
        }
    }
    fn print(&self) {
        self.color.print();
        self.dimensions.print();
        println!("Weight: {}", self.weight);

    }
}

struct Dimensions {
    height: f64,
    width: f64,
    depth: f64
}

impl Dimensions {
    fn print(&self) {
        println!("Width: {}", self.width);
        println!("Height: {}", self.height);
        println!("Depth: {}", self.depth);
    }
}