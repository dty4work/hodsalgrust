fn main() {
    let list = vec![100, 34, 72, 55];
    println!("first two of the list {:?}", return_first_two(&list));
    println!("list is {:?}", list);
    println!("first two of the list {:?}", return_first_two(&list));
}

fn return_first_two(borrowed_list: &[i32]) -> &[i32] {
    &borrowed_list[0..2]
}
