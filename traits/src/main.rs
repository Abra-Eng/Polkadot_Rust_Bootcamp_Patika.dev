fn main() {
    let dog = Dog {
        name: "Barte".to_string(),
    };

    let cow = Cow {
        name: "Bali".to_string(),
    };

    // dog.speak();
    // cow.speak();

    let original_string = "this is original string".to_string();
    let copy_string = original_string.display();

    println!("{}", copy_string); // prints this is original string

    animal_speak(&dog);
    animal_speak(&cow);


    let cat = Cat;

    cat.make_sound();
    cat.walk();
    cat.sleep();
}

// traits allows us to define shared behavior for multiple types.

trait Speak {
    fn speak(&self); // method signature
}

struct Dog {
    name: String,
}

struct Cow {
    name: String,
}

impl Speak for Dog {
    fn speak(&self) { // signature must be same but body of the function vary
        println!("{} says: Woof", self.name);
    }
}

impl Speak for Cow {
    fn speak(&self) {
        println!("{} says: Moooo!", self.name);
    }
}

trait Display {
    fn display(&self) -> String;
} 

impl Display for String { // implementing traits for an existing type
    fn display(&self) -> String {
        self.clone()
    }
}

// using traits as a function argument by defining trait bounds 
fn animal_speak<T: Speak>(animal: &T) {
    animal.speak();
}

trait Animal {
    fn make_sound(&self);

    fn sleep(&self) { // 
        println!("Animal sleep...");
    }
}

trait Mammal: Animal { 
    fn walk(&self);
}

trait Bird: Animal {
    fn fly(&self);
}

struct Cat;

impl Animal for Cat {
    fn make_sound(&self) {
        println!("Meoow");
    }
}

impl Mammal for Cat {
    fn walk(&self) {
        println!("the cat is walking");
    }
}