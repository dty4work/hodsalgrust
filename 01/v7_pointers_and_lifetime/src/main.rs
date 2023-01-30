#[derive(Debug, Clone)]
pub struct Person {
    name: String,
    age: i32,
}

impl Person {
    pub fn new(name: String, age: i32) -> Self {
        Person { name, age }
    }

    pub fn greet(&self) -> String {
        format!("Hi my name is {}", self.name)
    }

    pub fn age_up(&mut self, n: i32) {
        self.age += n;
    }

    pub fn dropme(self) {}
}

pub fn get_age(s: &Person) -> &i32 {
    &s.age
}

fn main() {
    println!("Hello, pointer and lifetime!");

    let mut p = Person::new("matt".to_string(), 35);
    p.age_up(3);
    let s = p.greet();
    println!("{}", s);

    let a = get_age(&p);
    // p.age_up(2);                            // won't work
    println!("person's age is {}", a);
    p.age_up(2);

    // p.dropme();

    let s2 = p.greet();
    println!("really : {}", s2);
}
