#[derive(Debug, Clone)]
pub struct Person {
    name: String,
    age: i32,
}

#[derive(Debug, Clone, Copy)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
}

fn main() {
    println!("Hello, Mutability, Copying and Cloning!");

    let mut x = 34;
    let y = x;
    x += 5;
    println!("x = {}, y = {}", x, y);

    let mut p = Person {
        name: "Matt".to_string(),
        age: 35,
    };

    let p2 = p.clone();
    p.name.push_str(" Stoodley");

    println!("p = {:?}, p2 = {:?}", p, p2);

    let p1 = Point::new(3, 4);
    let p2 = p1;

    println!("p1 = {:?}, p2 = {:?}", p1, p2);
}
