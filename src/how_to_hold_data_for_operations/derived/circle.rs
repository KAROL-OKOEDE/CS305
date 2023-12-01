// We're telling the computer to not worry about unused code and not give us any warnings.
#![allow(dead_code)]

// We're bringing in a special tool from the toy store that helps us compare sizes of values.
use core::cmp::Ordering;

// Now, we're talking about shapes, like circles and triangles, and how to play with them.
// Think of shapes like different kinds of toys to play with!

// This is a special instruction for creating shapes.
// It's like telling the computer how to make a new toy.
trait Shape {
    fn new(radius: f32, name: &'static str) -> Self;
    fn area(&self) -> f32;
    fn perimeter(&self) -> f32;
    fn set_radius(&mut self, radius: f32);
    fn get_radius(&self) -> f32;
    fn set_name(&mut self, name: &'static str);
    fn get_name(&self) -> &str;
}

// We're creating a special toy called a Circle.
// It can do things like calculate its area and perimeter.
// It also has a size (radius) and a name.
#[derive(Default, Debug, Clone)]
struct Circle {
    radius: f32,
    name: &'static str,
}

// Now, we're telling the computer how to make a Circle.
impl Circle {
    // This is like creating a default Circle toy.
    fn default() -> Self {
        Circle {
            radius: 1.0,
            name: "Circle1",
        }
    }
}

// Now, we're telling the computer how to make a Circle when we want a new one.
impl Shape for Circle {
    fn new(radius: f32, name: &'static str) -> Self {
        Circle { radius, name }
    }

    // This is how the Circle calculates its area.
    fn area(&self) -> f32 {
        (3.14159) * self.radius * self.radius
    }

    // This is how the Circle calculates its perimeter.
    fn perimeter(&self) -> f32 {
        (2.0) * self.radius * (3.14159)
    }

    // This is how we can change the Circle's size.
    fn set_radius(&mut self, radius: f32) {
        self.radius = radius;
    }

    // This is how we can ask the Circle about its size.
    fn get_radius(&self) -> f32 {
        self.radius
    }

    // This is how we can change the Circle's name.
    fn set_name(&mut self, name: &'static str) {
        self.name = name;
    }

    // This is how we can ask the Circle about its name.
    fn get_name(&self) -> &str {
        self.name
    }
}

// Now, we're telling the computer how to compare two Circle toys.
impl PartialEq for Circle {
    // This is how we check if two Circle toys are equal in size.
    fn eq(&self, other: &Self) -> bool {
        print!("{}", self.perimeter());
        self.perimeter() == other.perimeter()
    }

    // This is how we check if two Circle toys are not equal in size.
    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

// Now, we're telling the computer how to order two Circle toys by size.
impl PartialOrd for Circle {
    // This is how we compare the sizes of two Circle toys.
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.perimeter().partial_cmp(&other.perimeter())
    }
    // There are more ways to compare, but we're keeping it simple for now.
}

// Now, we're telling the computer how to turn a string into a Circle toy.
impl From<&'static str> for Circle {
    fn from(s: &'static str) -> Circle {
        // We're splitting the string into parts to get the size and name of the Circle toy.
        let mut parts = s.split(',');

        // We're taking the first part as the size of the Circle toy.
        let radius = match parts.next() {
            Some(val) => val.parse::<f32>().unwrap(),
            None => 0.0,
        };

        // We're taking the second part as the name of the Circle toy.
        let name = match parts.next() {
            Some(val) => val,
            None => "",
        };

        // We're creating a new Circle toy with the size and name we found.
        Circle { radius, name: &name }
    }
}

// Now, we're playing with our Circle toys!
pub fn run2() {
    // We're creating a default Circle toy named 'circle1'.
    let circle1 = Circle::default();

    // We're asking 'circle1' about its size and name, and printing the answers.
    println!("{}", circle1.radius);
    println!("{}", circle1.name);

    // We're creating a new Circle toy named 'circle2' with a specific size and name.
    let circle2 = Circle::new(2.3, "circle2");
    println!("{:?}", circle2);

    // We're creating a new Circle toy named 'circle3' from a string with a size and name.
    let circle3 = Circle::from("2.2,circle3");
    println!("{:?}", circle3);

    // We're comparing 'circle1' and 'circle2' using size.
    let result1 = circle1.partial_cmp(&circle2);
    println!(" result1 = {:?}", result1);

    // We're checking if 'circle1' is less than or equal to 'circle2' in size.
    let result2 = circle1.le(&circle2);
    println!("result2 = {:?}", result2);

    // We're checking if 'circle2' is equal to 'circle3' in size.
    let result3 = circle2.eq(&circle3);
    println!("result3 = {:?}", result3);

    // We're checking if 'circle2' is not equal to 'circle3' in size.
    let result4 = circle2.ne(&circle3);
    println!("result4 = {:?}", result4);
}
