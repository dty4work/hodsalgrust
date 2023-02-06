fn pluralize(singular: String) -> String {
    singular + "s"
}

fn main() {
    let s = String::from("book");

    let pl = pluralize(s.clone());

    // Add code here that calls the pluralize function
    println!("I have one {}, you have two {}", s, pl);
}
