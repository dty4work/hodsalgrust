use std::collections::HashMap;

fn demo_api_entry_works_in_newer_compiler() {
    println!("work with newer compiler");

    let text = "hello world hello";

    let mut freqs = HashMap::new();

    for word in text.split_whitespace() {
        match freqs.get_mut(word) {
            Some(value) => *value += 1,
            None => {
                freqs.insert(word, 1);
            }
        }
    }

    println!("Word frequencies: {:#?}", freqs);
}

fn demo_api_entry_only() {
    println!("Using entry api fix");

    let text = "hello world hello";

    let mut freqs = HashMap::new();

    for word in text.split_whitespace() {
        *freqs.entry(word).or_insert(0) += 1;
    }

    println!("Word frequencies: {:#?}", freqs);
}

fn main() {
    demo_api_entry_only();
    demo_api_entry_works_in_newer_compiler()
}
