pub struct Stemmer {
    pub suffix: String,
}

impl Stemmer {
    pub fn stem<'a>(&self, word: &'a str) -> &'a str {
        if word.ends_with(&self.suffix) {
            let index = word
                .rfind(&self.suffix)
                .expect("found because ends_with returned true");
            &word[0..index]
        } else {
            word
        }
    }
}

fn main() {
    println!("Generic Lifetime");
    println!("- Lifetime of a reference");
    println!("- Concrete lifetime of referenced values are unknown");
    println!("- Can exists in funcitons, methods, structs, enums, traits");
    println!("- Compiler needs enough info to prove validity for all possible lifetimes");

    println!();
    println!("Method with 1 Reference");

    let word = String::from("credited");
    let word_stem = {
        let stemmer = Stemmer {
            suffix: String::from("ed"),
        };
        stemmer.stem(&word)
    };
    println!("The stem of {} is {}", word, word_stem);
}
