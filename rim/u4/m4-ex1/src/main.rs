fn main() {
    println!("Generic Lifetime");
    println!("- Lifetime of a reference");
    println!("- Concrete lifetime of referenced values are unknown");
    println!("- Can exists in funcitons, methods, structs, enums, traits");
    println!("- Compiler needs enough info to prove validity for all possible lifetimes");

    println!();
    println!("Function with 1 Reference");

    let list = vec![100, 34, 72, 55];
    let first_two = return_first_two(&list);

    println!("first two are {:?}", first_two);
    println!("list is {:?}", list);
    println!("first two are {:?}", first_two);
}

fn return_first_two(borrowed_list: &[i32]) -> &[i32] {
    &borrowed_list[0..2]
}
