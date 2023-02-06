fn demo_slice() {
    println!("Hello, Slices!");

    let v = vec![10, 20, 30];
    let v_slice = &v[..9];

    println!("v_slice is: {:?}", v_slice);
}

fn demo_slice2() {
    let s = "🌮😺";
    let s_slice = &s[0..1];

    println!("s_slice is: {:?}", s_slice);
}

fn demo_diff_slice() {
    let a = [1, 2, 3];
    let b = [7, 8, 9, 10, 11];
    let v = vec![4, 5, 6];
    let v_slicee = &v[..];

    only_ref_to_array(&a);
    only_ref_to_vector(&v);
    ref_to_either_array_or_vector(&a[..]);
    ref_to_either_array_or_vector(&v[..]);
    ref_to_either_array_or_vector(&b[..]);
}

fn only_ref_to_array(param: &[i32; 3]) {
    println!("this is an array: {:?}", param);
}

fn only_ref_to_vector(param: &Vec<i32>) {
    println!("this is a vector: {:?}", param);
}

fn ref_to_either_array_or_vector(param: &[i32]) {
    println!("this is a slice: {:?}", param);
}

fn either_string_or_literal(param: &str) {
    println!("this is a string slice: {:?}", param);
}

fn demo_string_or_literal() {
    let s = String::from("hi");
    let string_literal = "hello";

    either_string_or_literal(&s);
    either_string_or_literal(&string_literal);
}

fn main() {
    demo_string_or_literal();
    demo_diff_slice();
    demo_slice2();
    demo_slice();
}
