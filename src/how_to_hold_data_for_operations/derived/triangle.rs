// This line is like telling the computer, "Don't worry if we have some tools we don't use."
#![allow(dead_code)]

// This line brings in a special tool that helps us compare things by size.
use core::cmp::Ordering;

// Imagine we want to make different shapes, like triangles.

// Think of this as having special instructions for creating a tool (cookie cutter) to make triangles.
trait Shape {
    fn new(base: f32, height: f32, a: f32, b: f32, c: f32, name: &'static str) -> Self;
    fn area(&self) -> f32;
    fn perimeter(&self) -> f32;
    fn set_base(&mut self, base: f32);
    fn get_base(&self) -> f32;
    fn set_height(&mut self, height: f32);
    fn get_height(&self) -> f32;
    fn set_a(&mut self, a: f32);
    fn get_a(&self) -> f32;
    fn set_b(&mut self, b: f32);
    fn get_b(&self) -> f32;
    fn set_c(&mut self, c: f32);
    fn get_c(&self) -> f32;
    fn set_name(&mut self, name: &'static str);
    fn get_name(&self) -> &str;
}

// Now, we're creating a specific tool (cookie cutter) for making triangular shapes (Triangle).
#[derive(Default, Debug, Clone)]
struct Triangle {
    a: f32,
    b: f32,
    c: f32,
    base: f32,
    height: f32,
    name: &'static str,
}

impl Triangle {
    // Think of this as having a default tool (cookie cutter) to make standard-sized triangles.
    fn default() -> Self {
        Triangle {
            a: 1.0,
            b: 1.0,
            c: 1.0,
            base: 1.0,
            height: 1.0,
            name: "triangle1",
        }
    }
}

// Now, we're telling the computer how to use our tool (cookie cutter) to make triangles.
impl Shape for Triangle {
    fn new(a: f32, b: f32, c: f32, base: f32, height: f32, name: &'static str) -> Self {
        Triangle { a, b, c, base, height, name }
    }

    // This is how the tool (cookie cutter) calculates the area of a triangle.
    fn area(&self) -> f32 {
        (0.5) * self.height * self.base
    }

    // This is how the tool (cookie cutter) calculates the perimeter of a triangle.
    fn perimeter(&self) -> f32 {
        self.a + self.b + self.c
    }

    // These functions let us change and ask about the triangle's base, height, sides, and name.
    fn set_base(&mut self, base: f32) {
        self.base = base;
    }

    fn get_base(&self) -> f32 {
        self.base
    }

    fn set_height(&mut self, height: f32) {
        self.height = height;
    }

    fn get_height(&self) -> f32 {
        self.height
    }

    fn set_a(&mut self, a: f32) {
        self.a = a;
    }

    fn get_a(&self) -> f32 {
        self.a
    }

    fn set_b(&mut self, b: f32) {
        self.b = b;
    }

    fn get_b(&self) -> f32 {
        self.b
    }

    fn set_c(&mut self, c: f32) {
        self.c = c;
    }

    fn get_c(&self) -> f32 {
        self.c
    }

    fn set_name(&mut self, name: &'static str) {
        self.name = name;
    }

    fn get_name(&self) -> &str {
        self.name
    }
}

// Now, we're telling the computer how to compare two triangle-shaped tools (cookie cutters).
impl PartialEq for Triangle {
    // This is how we check if two triangle-shaped tools (cookie cutters) are equal in size.
    fn eq(&self, other: &Self) -> bool {
        print!("{}", self.perimeter());
        self.perimeter() == other.perimeter()
    }

    // This is how we check if two triangle-shaped tools (cookie cutters) are not equal in size.
    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

// Now, we're telling the computer how to order two triangle-shaped tools (cookie cutters) by size.
impl PartialOrd for Triangle {
    // This is how we compare the sizes of two triangle-shaped tools (cookie cutters).
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.perimeter().partial_cmp(&other.perimeter())
    }
    // There are more ways to compare, but we're keeping it simple for now.
}

// Now, we're telling the computer how to turn a recipe (string) into a triangle-shaped tool (cookie cutter).
impl From<&'static str> for Triangle {
    fn from(s: &'static str) -> Triangle {
        let mut parts = s.split(',');

        let a = match parts.next() {
            Some(val) => val.parse::<f32>().unwrap(),
            None => 0.0,
        };

        let b = match parts.next() {
            Some(val) => val.parse::<f32>().unwrap(),
            None => 0.0,
        };

        let c = match parts.next() {
            Some(val) => val.parse::<f32>().unwrap(),
            None => 0.0,
        };

        let base = match parts.next() {
            Some(val) => val.parse::<f32>().unwrap(),
            None => 0.0,
        };

        let height = match parts.next() {
            Some(val) => val.parse::<f32>().unwrap(),
            None => 0.0,
        };

        let name = match parts.next() {
            Some(val) => val,
            None => "",
        };
        Triangle { a, b, c, base, height, name: &name }
    }
}

// Now, we're having fun using our triangle-shaped tools (cookie cutters) to make things!
pub fn run2() {
    // Imagine we have a default triangle-shaped tool (cookie cutter).
    let triangle1 = Triangle::default();
    
    // Imagine asking our tool (cookie cutter) about its base, height, and name, and printing the answers.
    println!("{}", triangle1.base);
    println!("{}", triangle1.height);
    println!("{}", triangle1.name);

    // Imagine using a new triangle-shaped tool (cookie cutter) with specific values and a name.
    let triangle2 = Triangle::new(2.3, 2.4, 3.6, 3.7, 3.8, "triangle2");
    println!("{:?}", triangle2);

}
