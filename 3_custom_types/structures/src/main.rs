// An attribute to hide warnings for unused code.
#![allow(dead_code)]

use std::fmt;

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// A unit struct
struct Unit;

// A tuple struct
struct Pair(i32, f32);

// A struct with two fields
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another struct
struct Rectangle {
    // A rectangle can be specified by where the top left and bottom right
    // corners are in space.
    top_left: Point,
    bottom_right: Point,
}

impl fmt::Display for Rectangle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Rectangle: tl: ({}, {}), br: ({}, {})",
            self.top_left.x, self.top_left.y, self.bottom_right.x, self.bottom_right.y
        )
    }
}

impl Rectangle {
    fn area(&self) -> f32 {
        let Rectangle {
            top_left: Point { x: tx, y: ty },
            bottom_right: Point { x: bx, y: by },
        } = self;
        let height = ty - by;
        let width = bx - tx;
        width * height
    }
}

fn square(point: Point, size: f32) -> Rectangle {
    // Add a function square which takes a Point and a f32 as arguments, and returns a Rectangle with its top left corner on the point, and a width and height corresponding to the f32.
    let bottom_right = Point {
        x: point.x + size,
        y: point.y - size,
    };
    Rectangle {
        top_left: point,
        bottom_right: bottom_right,
    }
}

fn main() {
    // Create struct with field init shorthand
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };

    // Print debug struct
    println!("{:?}", peter);

    // Instantiate a `Point`
    let point: Point = Point { x: 10.3, y: 0.4 };
    let another_point: Point = Point { x: 5.2, y: 0.2 };

    // Access the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Make a new point by using struct update syntax to use the fields of our
    // other one
    let bottom_right = Point {
        x: 5.2,
        ..another_point
    };

    // `bottom_right.y` will be the same as `another_point.y` because we used that field
    // from `another_point`
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // Destructure the point using a `let` binding
    let Point {
        x: left_edge,
        y: top_edge,
    } = point;

    let _rectangle = Rectangle {
        // struct instantiation is an expression too
        top_left: Point {
            x: left_edge,
            y: top_edge,
        },
        bottom_right: bottom_right,
    };
    println!("{}", _rectangle);
    println!("rectangle area: {}", _rectangle.area());

    // Instantiate a unit struct
    let _unit = Unit;

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);

    println!("{}", square(Point { x: 2.0, y: 3.0 }, 4.0));
}
