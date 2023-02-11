fn main() {
    println!("Returning Reference from Inner Scope");
    let first_two = {
        let list = vec![100, 34, 72, 55];
        &list[0..2] //  ^^^^ borrowed value does not live long enough
    };

    println!("First two are {:?}", first_two);
}
