// use snake::Animal;
// use snake::Log;
// use snake::Person;
// use snake::log_info;
// use snake::log_info2;

// use snake::{log_info, log_info2, Animal, Log, Person};
// use snake::*;

use snake::education::learning_rust::{Log, Person};

fn main() {
    let person = Person::new();
    println!("{}", person.id)

    // let id = PersonId::Passport(432);
    // println!("{}", id);
    // println!("{}", person.name());

    // let person_2 = Person::from(
    //     String::from("John"),
    //     String::from("Snow"),
    //     35,
    //     PersonId::Passport(123172371),
    // );
    // let animal = Animal(String::from("dog"));

    // person.change_age(38);
    // person.display_info();
    // // person.alert_soemthing();
    // // log_info(person);
    // log_info(person);

    // animal.display_info();
    // // animal.alert_soemthing();
    // // log_info(animal);
    // log_info2(&animal);
}
