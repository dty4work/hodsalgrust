fn main() {
    println!("Generic Lifetime");
    println!("- Lifetime of a reference");
    println!("- Concrete lifetime of referenced values are unknown");
    println!("- Can exists in funcitons, methods, structs, enums, traits");
    println!("- Compiler needs enough info to prove validity for all possible lifetimes");

    println!();
    println!("Function with 2 Reference");

    work()
}

// // Cannot compile, team2 short lived
// fn bomb() {
//     let team1 = String::from("Panthers");
//     let winner = {
//         let team2 = String::from("Yellow Jackets");
//         simulate_gam(&team1, &team2)
//     };
//     println!("Can still discuss {} here", team1);
// }

fn work() {
    let team1 = String::from("Panthers");
    {
        let team2 = String::from("Yellow Jackets");
        let winner = simulate_gam(&team1, &team2);
        println!("{} vs {}: {} won", team1, team2, winner);
    }
    println!("Can still discuss {} here", team1);
}

fn simulate_gam<'a>(home: &'a str, away: &'a str) -> &'a str {
    if rand::random() {
        home
    } else {
        away
    }
}
