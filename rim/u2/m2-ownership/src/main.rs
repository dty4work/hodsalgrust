fn main() {
    println!("Hello, Ownership");

    println!("Each piece of data has one owning variable");
    println!("Owner is responsible for cleaning up that data");
    println!("Cleanup happens when the owner goes out of scope");
    println!("The owner decides on mutability");

    demo_string();
    demo_cat_eyes();
}

fn demo_string() {
    let mut a = String::from("hello");
    a.push_str(" there");

    say(a.clone());

    println!("I say, {}!", a);
}

fn say(s: String) {
    println!("I say, {}!", s);
}

fn heart_eyes_cat() -> String {
    String::from("😻")
}

fn demo_cat_eyes() {
    let a = heart_eyes_cat();

    println!("Cat eyes {}", a);
}
