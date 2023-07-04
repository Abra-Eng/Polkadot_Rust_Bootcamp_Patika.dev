fn main() {
    let mut pointer = Box::new(5); // Allocates (forces) memory on the heap and then places (x) into it.
    
    *pointer = 10; // * dereference , gets the value of pointer

    println!("{}", pointer);

    // {
    //     let temp = Box::new("Norm");
    // }

    // println!("Temp: {}", temp); when box goes out of scope, the pointer deallocates the memory
    
    let trait_object: Box<dyn MakeNoise> = Box::new(Bird { 
        name: "sparro".to_string(),
        color: "red".to_string(),
    });// with this we put our trait object at heap memory
    // dyn is used for initializing the trait object
    trait_object.talk();

    invite_to_animal_talks(trait_object);

    let mut speakers: Vec<Box<dyn MakeNoise>> = Vec::new();

    speakers.push(Box::new(Bird {name: "birdy".to_string(), color: "aqua".to_string()}));
    speakers.push(Box::new(Dog {name: "doggy".to_string(), breed: "terrier".to_string()}));

    for speaker in speakers {
        speaker.talk();
    }
}

trait MakeNoise {
    fn talk(&self);
}

struct Bird {
    name: String,
    color: String,
}

struct Dog {
    name: String,
    breed: String,
}

impl MakeNoise for Bird {
    fn talk(&self) {
        println!("Doggy is talking..");
    }
}

impl MakeNoise for Dog {
    fn talk(&self) {
        println!("Birdy is talking..");
    }
}

// using trait objects as a parameter in a function

fn invite_to_animal_talks(speaker: Box<dyn MakeNoise>) {
    println!("Our next speaker is: ");
    speaker.talk(); 
}

// The rules of using trait object
// 1.The trait must not have any associated constants
// 2. The method must have a self parameter, either &self, &mut self, or self.
// 3. we cannot use generics with our method
// by follow,ng these rules, Rust ensures that a trait can be used safely

// trait Calculate<T> {
//     const PI: f64;

//     fn calculate_area(&self, value: T) -> f64;
// } error



