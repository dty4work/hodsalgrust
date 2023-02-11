fn main() {
    println!("Returning Reference from function");
    let first_two = return_first_two();
    println!("First two are {:?}", first_two);
}

// ^ expected named lifetime parameter
fn return_first_two() -> &[i32] {
    let list = vec![100, 34, 72, 55];
    &list[0..2]
}
