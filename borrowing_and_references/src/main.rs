fn main() {
    let my_string = String::from("Value");

    let _my_ref = &my_string; // we created an immutable reference - read only

    // println!("My reference is {}", my_ref);

    let my_string = String::from("Value");
    print_string(&my_string);

    // println!("i still have my string {}", my_string);

    let mut my_string = String::from("Hello");
    change_string(&mut my_string);
    // println!("{}", my_string);

    let first_immutable_reference = &my_string;
    let second_immutable_reference = &my_string;

    println!(
        "first im. reference {} and second im. reference {}",
        first_immutable_reference, second_immutable_reference
    );

    //we can have multiple immutable references at a time 
    // bu we can't have a mutabel and an immutable reference at the same time

    let first_mutable_reference = &mut my_string;
    println!("First mutable reference {}", first_mutable_reference);

    // println!("Immutable reference value", first_immutable_reference); // Gives an error
    
    // after creating the mutable reference, we can't use the immutable reference
    // but we may have both reference at a same time as long as we don't use
    // having both references same time may lead risky situation
    // so generally we don't create both references at to same variable at the same time

    let second_mutable_reference = &mut my_string;
    println!("{}", second_mutable_reference);

   // println!("First mutable reference {}", first_mutable_reference); // gives an error  

   // Reference guide
   // 1. we can have either one mut ref and any number of immut. references to a variable at a time
   // 2. not have both references to the same variable at the same time
   // 3. references always must be valid and point a valid memory location
   // 4. references automatically expire at the end of their scope 

   let new_string = String::from("Dangling");
   let new_string_ref = return_ref(&new_string);

   println!("new string {}", &new_string_ref);

   // let newer_string = new_string; gives an error
   println!("new string reference {}", new_string_ref);
}


fn print_string(s: &String) {
    println!("{}", s);
}

fn change_string(s: &mut String) {
    s.push_str(" world");
}

// dangling references
fn return_ref(some_string: &String) -> &String {
    some_string
} // some_string goes out of scope, but