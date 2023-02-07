fn main() {
    println!("Hello, Result Expect!");

    let num: i32 = "10".parse().expect("Parsing Failed");
    println!("The number is {}", num);

    let num: i32 = match "10".parse() {
        Ok(inner) => inner,
        Err(_) => panic!("Parsing Failed"),
    };
    println!("The number is {}", num);
}
