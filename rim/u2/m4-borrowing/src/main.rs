#[derive(Debug)]
struct Person {
    name: String,
}

fn congratulate(person: &Person) {
    println!("Congratulations, {}!!!", person.name);
}

fn demo_congratulate() {
    let p = Person {
        name: String::from("Jake"),
    };

    congratulate(&p);
    println!("Can still use p here: {}", p.name);
    println!("Can still use p here: {}", p.name);
}

// // Does not work, n will be dropped after return....
// fn name() -> &str {
//     let n = String::from("Caro");
//     &n
// }

fn name() -> String {
    let n = String::from("Caro");
    n
}

fn demo_ptr_str() {
    let my_name = name();
    println!("name is {}", my_name);
}

fn pluralize(single: &str) -> String {
    single.to_owned() + "s"
}

fn demo_pluralize() {
    let s = String::from("book");
    let pl = pluralize(&s);

    println!("I have one {}, you have two {}", s, pl,);
}

fn main() {
    demo_pluralize();
    demo_ptr_str();
    demo_congratulate();
}
