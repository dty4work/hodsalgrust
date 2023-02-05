#[derive(Debug)]
enum HockeyPosition {
    Center,
    Wing,
    Defense,
    Goalie,
}

#[derive(Debug)]
struct HockeyPlayer {
    name: String,
    number: u8,
    position: HockeyPosition,
    goals_ytd: u8,
}

fn demo_struct_player() {
    let mut player = HockeyPlayer {
        name: String::from("Bryan Rust"),
        number: 17,
        position: HockeyPosition::Wing,
        goals_ytd: 7,
    };

    player.goals_ytd += 8;

    println!(
        "{} has scored {} goals this season",
        player.name, player.goals_ytd
    );
}

#[derive(Debug, Clone)]
struct Triangle(u32, u32, u32);

fn is_equilateral(triangle: Triangle) -> bool {
    triangle.0 == triangle.1 && triangle.1 == triangle.2
}

fn demo_struct_tuple() {
    let triangle1 = Triangle(3, 4, 5);
    println!(
        "trinagle {:?} {} equilateral",
        triangle1.clone(),
        if is_equilateral(triangle1) {
            "is"
        } else {
            "not"
        }
    );
}

#[derive(Debug, Clone)]
struct Meters(u8);

fn add_distances(d1: &Meters, d2: &Meters) -> Meters {
    Meters(d1.0 + d2.0)
}

fn demo_struct_unit() {
    let distance1 = Meters(3);
    let distance2 = Meters(7);

    let distance3 = add_distances(&distance1, &distance2);

    println!("d1 {:?} + d2 {:?} = {:?}", distance1, distance2, distance3);
}

fn main() {
    demo_struct_player();

    demo_struct_tuple();

    demo_struct_unit();
}
