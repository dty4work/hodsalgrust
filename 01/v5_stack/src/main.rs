fn main() {
    let x = 3;
    let v = a(x);
    println!("v is {}", v);

    for i in 0..=50 {
        println!("factorial({}) = {}", i, factorial(i));
    }

    for i in 0..=50 {
        println!("fact_tail({}) = {}", i, fact_tail(i, 1.));
    }
}

fn a(mut n: i32) -> i32 {
    n += 2;
    let res = b(n);
    return res;
}

fn b(n: i32) -> i32 {
    return n * 2;
}

fn factorial(n: i64) -> f64 {
    if n <= 1 {
        1.
    } else {
        n as f64 * factorial(n - 1)
    }
}

fn fact_tail(n: i64, r: f64) -> f64 {
    if n <= 1 {
        r
    } else {
        fact_tail(n - 1, n as f64 * r)
    }
}
