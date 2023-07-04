static something: &str = "Another way"; // we can use this as a global variable

fn main() {
    let name = "Abra Tulpar";
    let person = Person {name: &name}; // valid, they live in same block so they have same lifetime
    println!("The name is: {}", person.name);
    
    // static lifetime
    // lives as long as end of our program, it's immortal
    // all string literals in Rust have a 'static lifetime
    let s: &'static str = "Lifetimes";

    // lifetime mimicking - subtyping
    let parent: &'static str = "Here forever until the program stops";
    let kid: &'static str = {
        let short_lifetime_str = String::from("Just Passing");
        &short_lifetime_str // doesn't live long enough
    };

    //let name = "Grandpa".to_string();
    //let person: Ancestor<'static> = Ancestor(&name);

}

// Rust needs to ensure that the references inside a struct are valid for as long as the struct exists
struct Person<'a> {
    name: &'a str,
}


// bound lifetimes - constrain the lifetimes of generic parameters.

struct LongLived<'a>(&'a str);

struct ShortLived<'a> {
    name: &'a str,
    long: LongLived<'a>,
}


struct Ancestor<'a>(&'a str);

fn problematic_function() -> &str { // borrowed value doesn't long live enough
    let some_string = String::from("some");
    &some_string[..] // goes out of scope when function ends
}

fn problematic_function_solution() -> String { 
    let some_string = String::from("some");
    some_string
}