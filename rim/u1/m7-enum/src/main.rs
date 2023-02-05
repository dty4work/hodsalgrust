use std::cmp::min;

enum HockyePosition {
    Center,
    Wing,
    Defense,
    Goalie,
}

enum Clock {
    Sundial(u8),
    Digital(u8, u8),
    Analog(u8, u8, u8),
}

fn tell_clock(clock: Clock) {
    match clock {
        Clock::Sundial(hours) => println!("It is about {} o'clock", hours),
        Clock::Digital(hours, minutes) => println!("It is {} minutes past {}", minutes, hours),
        Clock::Analog(hours, minutes, seconds) => println!(
            "It is {} minutes and {} seconds past {} o'clock",
            minutes, seconds, hours
        ),
    }
}

fn next_player(position: HockyePosition) {}

fn main() {
    let position = HockyePosition::Defense;

    println!("Hello, Enum");

    tell_clock(Clock::Analog(9, 25, 45));
}
