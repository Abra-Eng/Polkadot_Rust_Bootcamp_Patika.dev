fn main() {
    let original_string = String::from("Value");
    let _cloned_string = original_string.clone(); // we clone the Value (heap memory)

    // println!("original string is {}", original_string);
    // println!("Cloned string is {}", cloned_string);

    let original_string = String::from("String");
    let modified_string = modify_string(&original_string);

    println!("Original value {}", original_string);
    println!("Modified value {}", modified_string);
}

// using clone with borrowing and references

fn modify_string(s: &String) -> String {
    let mut cloned_string = s.clone(); // we get an immutable reference and without 
    cloned_string.push_str(" is modified"); // effecting the original value, created a mutable reference
    cloned_string
}
