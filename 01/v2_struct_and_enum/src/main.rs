#[derive(Debug)]
pub struct Person {
    name: String,
    age: i32,
    children: i32,
    favorite_color: Color,
}

impl Person {
    pub fn print(&self) -> String {
        format!(
            "name = {}, age = {} has {} children",
            self.name, self.age, self.children
        )
    }
}

#[derive(Debug)]
pub enum Color {
    Red(String),
    Green,
    Blue,
}

fn main() {
    let p = Person {
        name: "matt".to_string(),
        age: 35,
        children: 4,
        favorite_color: Color::Green,
    };

    let c = Color::Red("hello".to_string());

    match c {
        Color::Red(s) => println!("It' Red {}", s),
        Color::Green => println!("It's Green"),
        Color::Blue => println!("It's Blue"),
    };

    println!("Hello, algorithms people from {}", p.print());
    println!("Hello, algorithms people from {:?}", p);
}
