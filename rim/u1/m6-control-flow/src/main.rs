fn main() {
    roll_two_dices();
    
    demo_match();

    demo_for1();

    secret_word();

    discount(10);
}

fn roll_two_dices() {
    let die1 = 1;
    let die2 = 5;

    match (die1, die2) {
        (1, 1) => println!("Snake eyes! Go back to the beginning"),
        (5, _) | (_, 5) => {
            println!("You rolled at least one 5!");
            println!("Move and then roll again!");
        },
        _ => println!("Move your piece!"),
    }
}

fn demo_match() {
    let x = 3;

    match x {
        1 => println!("One is the lonelist number"),
        2 => println!("Two's company"),
        3 => println!("Three's a crowd"),
        _ => println!("Some other number"),
    }
}

fn discount(day_of_moth: u8) {
    let amount = if day_of_moth % 2 == 0 { 50 } else { 10 };

    println!("Your discount is {}!", amount);
}

fn secret_word() {
    loop {
        println!("What's the secret world?");
        let mut word = String::new();
        std::io::stdin()
            .read_line(&mut word)
            .expect("Failed to read line");

        if word.trim() == "rust" {
            break;
        }
    }

    println!("You known the secret word! Please proceed!");
}

fn demo_for1() {
    for i in 1..11 {
        println!("Now serving number {}", i);
    }
}
