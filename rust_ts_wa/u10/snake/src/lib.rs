mod another_lib;

use another_lib::another_mod;

fn outsider() {
    another_mod::another_fn();
    println!("Outsider Function");
}

pub mod education {

    pub mod learning_rust {
        use std::fmt;

        mod top_level {
            pub fn hi_there() {
                println!("hi there!");
            }

            pub mod low_level {
                pub fn hello_world() {
                    println!("hello world");
                }
            }
        }
        pub trait Log {
            fn display_info(&self) {}

            fn alert_soemthing(&self) {
                println!("Default  implementation!!!!!!")
            }
        }

        #[derive(Debug)]
        pub enum PersonId {
            Passport(u32),
            IndentiftyCard(u32, u32, u32),
        }

        impl fmt::Display for PersonId {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                match self {
                    PersonId::Passport(x) => {
                        write!(f, "{}", x)
                    },
                    PersonId::IndentiftyCard(x, y, z) => {
                        write!(f, "({}, {}, {})", x, y, z)
                    },
                }
                
            }
        }

        pub struct Person {
            name: String,
            last_name: String,
            age: u32,
            pub id: PersonId,
        }

        pub struct Animal(pub String);

        impl Log for Animal {
            fn display_info(&self) {
                println!("{}", self.0)
            }

            fn alert_soemthing(&self) {
                println!("ANIMAL implementation!!!!!!");
            }
        }

        impl Log for Person {
            fn display_info(&self) {
                // crate::learning_rust::top_level::hi_there();
                // top_level::low_level::hello_world();

                crate::outsider();
                super::super::outsider();

                println!(
                    "{} {} {} {:?}",
                    self.name, self.last_name, self.age, self.id
                )
            }
        }

        impl Person {
            pub fn new() -> Person {
                Person {
                    name: "Default".to_string(),
                    last_name: "Default".to_string(),
                    age: 0,
                    id: PersonId::IndentiftyCard(540, 320, 100),
                }
            }

            pub fn change_age(&mut self, new_age: u32) {
                self.age = new_age;
            }

            pub fn from(name: String, last_name: String, age: u32, id: PersonId) -> Person {
                Person {
                    name,
                    last_name,
                    age,
                    id,
                }
            }

            pub fn name(&self) -> &String {
                &self.name
            }
        }

        pub fn log_info(val: impl Log) {
            val.alert_soemthing();
        }

        pub fn log_info2(val: &dyn Log) {
            val.alert_soemthing();
        }
    }
}
