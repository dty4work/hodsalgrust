fn main() {
    println!("Hello, Buffer Overread!");

    let accts = vec![50298, 7423];

    let bal = accts[11];
    println!("Balance: {}", bal);
}
