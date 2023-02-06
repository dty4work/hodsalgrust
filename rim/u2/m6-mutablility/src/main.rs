#[derive(Debug)]
struct Bucket {
    liters: u32,
}

fn pour(source: &mut Bucket, target: &mut Bucket, amount: u32) {
    source.liters -= amount;
    target.liters += amount;
}

fn demo_bucket() {
    let mut bucket1 = Bucket { liters: 20 };
    let mut bucket2 = Bucket { liters: 30 };

    pour(&mut bucket1, &mut bucket2, 3);

    println!("Bucket 1: {:?}", bucket1);
    println!("Bucket 2: {:?}", bucket2);
}

#[derive(Debug)]
struct CarPool {
    passengers: Vec<String>,
}

impl CarPool {
    fn pick_up(&mut self, name: String) {
        self.passengers.push(name);
    }
}

fn demo_carpool() {
    let mut monday_car_pool = CarPool { passengers: vec![] };

    monday_car_pool.pick_up(String::from("Jake"));
    println!("Carpool state: {:?}", monday_car_pool);

    monday_car_pool.pick_up(String::from("Carol"));
    println!("Carpool state: {:?}", monday_car_pool);
}

fn demo_mutability_rules() {
    let mut list = vec![1, 2, 3];
    for i in &list {
        println!("i is {}", i);
        // list.push(i + 1);  // error[E0502]: cannot borrow `list` as mutable because it is also borrowed as immutable
    }
}

fn main() {
    demo_mutability_rules();
    // demo_carpool();
    // demo_bucket();
}
