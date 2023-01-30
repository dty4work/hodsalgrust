#[derive(Debug)]
pub struct LinkedList<T> {
    data: T,
    next: Option<Box<LinkedList<T>>>,
}

impl<T: std::ops::AddAssign> LinkedList<T> {
    pub fn add_up(&mut self, n: T) {
        self.data += n;
    }
}

fn main() {
    println!("Hello, Box!");
    let mut ll = LinkedList {
        data: 3,
        next: Some(Box::new(LinkedList {
            data: 2,
            next: None,
        })),
    };
    println!("Hello, {:?}", ll);

    if let Some(ref mut v) = ll.next {
        v.add_up(10);
    }
    println!("Hello, {:?}", ll);

    let mut v: Vec<String> = Vec::with_capacity(100);
    v.push("hello ".to_string());
    v.push("goodbye".to_string());
    println!("v.len = {}, v.capacity = {}", v.len(), v.capacity());

    for i in 0..105 {
        v.push(i.to_string());
    }
    println!("v.len = {}, v.capacity = {}", v.len(), v.capacity());

    let mut s = "   hello   ".to_string();
    let p = s.trim().to_string();

    s.push_str("goodbyte");

    println!("p = {}", p);
    println!("s = {}", s);

    let fstr = "help me find home";
    let ffstr = string_find_f(fstr);
    println!("ffstr = '{}'", ffstr);
    println!("choosen = '{}'", choose_str(1));
}

fn string_find_f<'a>(s: &'a str) -> &'a str {
    // Box<str> === String
    let n = 0;
    for (n, x) in s.char_indices() {
        if x == 'f' {
            return &s[n..];
        }
    }
    s
}

fn choose_str(n: i32) -> &'static str {
    match n {
        0 => "hello",
        1 => "goodbye",
        _ => "other",
    }
}
