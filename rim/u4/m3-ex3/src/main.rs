fn main() {
    println!("Referencing a moved value");

    let list_a = vec![100, 34, 72, 55];
    let lisb_b = list_a;
    let first_two = &list_a[0..2];
    println!("The first two values are {:?}", first_two);
}
