use profession::cook;

mod sports; 

use sports::{football, FootballPlayer};
// use sports::football;
// use sports::FootballPlayer;

fn main() {
    // Everything in Rust is private by default

    // profession::cook(); // if our function isn't public then throws an error
    cook();

    let teacher = profession::Teacher {
        name: "Alan".to_string(),
        field: "Biology".to_string(),
        age: 23,
    };

    println!("Teacher {:?} ", teacher);

    // mod sports
    let my_player = FootballPlayer {
        name: "Derek".to_string(),
        age: 23,
    };

    football();

}

mod profession {
    pub fn cook() { // pub keyword make function public so we can access that outside of our module
        println!("Cooking...");
    }
    
    #[derive(Debug)]
    pub struct Teacher{
        pub name: String,
        pub field: String,
        pub age: i32,
    }
}