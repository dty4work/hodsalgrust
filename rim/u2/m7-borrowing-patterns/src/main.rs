fn demo_new_scopes() {}

fn demo_temp_variables() {}

fn demo_entry_api() {}

fn demo_splitting_up_structs() {}

fn demo_in_scope_immutablle_only() {
    println!("Hello, borrowing at the same time!");

    let list = vec![1, 2, 3];

    let list_first = list.first();
    let list_last = list.last();

    println!(
        "The first element is {:?} and the last is {:?}",
        list_first, list_last,
    );
}

// fn demo_in_scope_with_mutable_not_compiled() {
//     let mut list = vec![1, 2, 3];

//     let list_first = list.first();
//     let list_last = list.last();

//     let list_first_mut = list.first_mut().expect("list was empty");
//     *list_first_mut += 1;

//     println!(
//         "The first element is {:?} and the last is {:?}",
//         list_first, list_last,
//     );
// }

fn demo_in_scope_with_mutable_compiled() {
    let mut list = vec![1, 2, 3];

    let list_first_mut = list.first_mut().expect("list was empty");
    *list_first_mut += 1;

    let list_first = list.first();
    let list_last = list.last();

    println!(
        "The first element is {:?} and the last is {:?}",
        list_first, list_last,
    );

    // let list_first_mut = list.first_mut().expect("list was empty");
    // *list_first_mut += 1;
}

fn demo_in_scope_with_mutable_new_scope() {
    let mut list = vec![1, 2, 3];

    {
        let list_first = list.first();
        let list_last = list.last();

        println!(
            "The first element is {:?} and the last is {:?}",
            list_first, list_last,
        );
    }

    *list.first_mut().expect("list was empty") += 1;
}

fn main() {
    demo_in_scope_with_mutable_new_scope();
    // demo_in_scope_with_mutable_compiled();
    // demo_in_scope_immutablle_only();
}
