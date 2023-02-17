fn main() {
    c();
    d();
    f();
}

fn a() {
    println!("calling A");
    e();
}

fn b() {
    println!("calling B");
}

fn c() {
    println!("calling C");
}

fn d() {
    println!("calling D");
    a();
}

fn e() {
    println!("calling E");
}

fn f() {
    println!("calling F");
    b();
}
