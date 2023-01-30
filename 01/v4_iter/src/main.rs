pub struct Stepper {
    curr: i32,
    step: i32,
    max: i32,
}

impl Iterator for Stepper {
    type Item = i32;

    fn next(&mut self) -> Option<i32> {
        if self.curr >= self.max {
            return None;
        }
        let res = self.curr;
        self.curr += self.step;
        Some(res)
    }
}

fn main() {
    // demo_loopbreak();
    // demo_while();
    // demo_for();
    // demo_stepper();
    demo_stepper_while();
}

fn demo_stepper_while() {
    let mut st = Stepper {
        curr: 3,
        step: 4,
        max: 15,
    };

    while let Some(n) = st.next() {
        println!("while {}", n)
    }

    let fstepper = Stepper {
        curr: 5,
        step: 10,
        max: 50,
    };

    for i in fstepper {
        println!("for loop {}", i);
    }
}

fn demo_stepper() {
    let mut st = Stepper {
        curr: 2,
        step: 3,
        max: 15,
    };

    loop {
        match st.next() {
            Some(v) => println!("loop {}", v),
            None => break,
        }
    }
}

fn demo_for() {
    for n in 1..=10 {
        println!("Hello, {}!", n);
    }
}

fn demo_while() {
    let mut n = 0;

    while n <= 10 {
        n += 1;
        println!("Hello, {}!", n);
    }

    println!("All done while");
}

fn demo_loopbreak() {
    let mut n = 0;

    loop {
        n += 1;
        if n > 10 {
            break;
        }

        println!("Hello, {}!", n);
    }

    println!("All done loop/break");
}
