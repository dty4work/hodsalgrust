fn main() {
    let message = "Hello World";
    let message_2 = print_welcome(message);
    println!("Return message {}", message_2);
}

fn print_welcome(text: &str) -> &str {
    println!("{}", text);
    "Hi There"
}
