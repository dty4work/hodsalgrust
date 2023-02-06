fn main() {
    println!("Hello, world!");
}

// With complex Monster, it will not compile......
// pub struct Monster {
//     hp: u8,
//     sp: u8,
//     friends: Vec<Friend>,
// }

// pub struct Friend {
//     loyalty: u8,
// }

// impl Monster {
//     pub fn final_breath(&mut self) {
//         if let Some(friend) = self.friends.first() {
//             self.heal(friend.loyalty);
//             println!("Healing for {}", friend.loyalty);
//         }
//     }

//     pub fn heal(&mut self, amount: u8) {
//         self.hp += amount;
//         self.sp -= amount;
//     }
// }

// Splitting Monster to Stats and Friends - help compile and organize
pub struct Monster {
    st: Stats,
    friends: Vec<Friend>,
}

pub struct Stats {
    hp: u8,
    sp: u8,
}

impl Stats {
    pub fn heal(&mut self, amount: u8) {
        self.hp += amount;
        self.sp -= amount;
    }
}

pub struct Friend {
    loyalty: u8,
}

impl Monster {
    pub fn final_breath(&mut self) {
        if let Some(friend) = self.friends.first() {
            self.st.heal(friend.loyalty);
            println!("Healing for {}", friend.loyalty);
        }
    }
}
