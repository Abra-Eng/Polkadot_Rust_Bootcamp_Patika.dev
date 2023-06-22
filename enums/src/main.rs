fn main() {
    let current_weather = Weather::Cloudy; // use of the double colon (::) is to access the enum variants.

    let msg = Message::Write(String::from("Rust"));
    process_message(msg); // Message is 'Rust'

    let my_pet = Animal::Cat("Teki".to_string());
    // to handle single enum variant
    if let Animal::Cat(name) = my_pet {
        println!("Cat name is: {}", name);
    } else {
        println!("Not a cat");
    }

    let msg = Message::Quit;
    msg.call();
}

// Enums, short for enumerations, are a powerful feature in Rust
// that allows you to represent multiple related values within a single data type.
enum Weather {
    Sunny,
    Cloudy,
    Rainy,
}

enum Message {
    // enums can store data associated with each variant
    Quit,
    Move {x: i32, y:i32},
    Write (String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) { // self is insctance which is calling this method
        match self {
            Message::Quit => println!("Quit"),
            Message::Move { x, y } => println!("Move to x: {}, y: {}", x, y),
            Message::Write(s) => println!("Text: {}", s),
            Message::ChangeColor(r, g, b) => {
                println!("Change colot to red: {}, green: {}, blue: {}", r, g, b);
            },
        }
    }
}


fn process_message(msg: Message) { // takes Message enum instances as an argument
    match msg {
        // destructuring associated data within the enum
        Message::Quit => {
            println!("The Quit variant has no data.");
        }
        Message::Move { x, y } => {
            println!("Coordinates are {} and {}", x, y);
        }
        Message::Write(text) => {
            println!("Text message is {}", text);
        }
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red: {} green: {} blue: {}", r, g, b);
        }
        // since enum has 4 variants and we checking all of them, rust doen't complain for the unchecked possibilites

    }
}

enum Animal {
    Dog(String),
    Cat(String),
    Bird(String),
}