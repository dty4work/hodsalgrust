use std::num;

fn main() {
    println!("Hello, M2 Functions!");
    next_birthday("Dennis", 50);
    next_birthday("Vivian", 0);

    println!("The answer is {}", square(3));
}

fn next_birthday(name: &str, current_age: u8) {
    let next_age = current_age + 1;
    println!(
        "Hi {}, on your next birthday, you'll be {}!",
        name, next_age
    );
}

fn square(num: i32) -> i32 {
    num * num

}

