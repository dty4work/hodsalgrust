struct Noisy {
    id: i32,
}

impl Drop for Noisy {
    fn drop(&mut self) {
        println!("Noisy number {} going out of scope!", self.id);
    }
}

fn main() {
    println!("Hello, Drop Trait");

    let _n1 = Noisy { id: 1 };
    let _n2 = Noisy { id: 2 };
    println!("End of main");
}
