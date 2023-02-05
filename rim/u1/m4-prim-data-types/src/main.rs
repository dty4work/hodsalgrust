fn main() {
    println!("Hello, primitive data types");
    demo_bool();
    demo_array();
}

fn demo_array() {
    println!("demo array");
    let mut b = [7.2, 10.4, 345.01];
    b[0] = 0.0;
    println!("{:?}", b);
}

fn demo_bool() {
    println!("demo bool");

    let a = true;
    let b = false;

    if a {
        println!("a is true!");
    }
    if b {
        println!("b is true!");
    }
}
