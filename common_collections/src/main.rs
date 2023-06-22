use std::collections::HashMap;

fn main() {
    // vec! macro
    let mut numbers = vec![1, 3, 5, 8];

    // Vec::new()
    let mut names: Vec<String> = Vec::new();
    
    // adding elements to the vector
    names.push(String::from("Abra")); 
    names.push(String::from("Yutpa"));

    // accessing the elements of the vector
    let first_name = &names[0]; 
    let second_name = &names[1];

    println!("First name is {}. Second names is {} ", first_name, second_name);

    // removing or poping elements of the vector
    names.pop(); // remove the element which end of the vector


    // iterating through vectors
    for number in &numbers {

        println!("the number is {}", number);
    }

    // slice a vector
    let slice  = &numbers[1..3]; // [3,5]


    // Strings
    let mut my_string = String::from("Test");
    let mut other_string = "other string".to_string();

    // adding string
    my_string.push_str(" string");
    my_string.push('!'); // just adding a character 

    println!("{}", my_string);

    // iterating through strings
    for c in my_string.chars() {
        println!("Char: {}", c);
    }

    // iterate through strings using bytes
    for b in my_string.bytes() {
        println!("{}", b);
    }


    // HashMaps {key: value}; to use,  use std::collections::HashMap;
    let mut ages = HashMap::new();

    // inserting key and value to hashmap
    ages.insert(String::from("Ragnar"), 32); // [Ragnar: 32]
    ages.insert(String::from("Freya"), 23); // [Ragnar: 32, Freya:23]

    // retrieving the value
    let freya_age = ages.get(&String::from("Freya")); // returns an Option<T>

    println!("{:?}", freya_age); // Some(23)

    println!("{:?}", ages); // {"Freya": 23, "Ragnar": 32}

    //  remove an element from HashMap
    ages.remove(&String::from("Ragnar"));

    println!("{:?}", ages); // {"Freya": 23}

    // iterating through HashMaps
    for (key, value) in &ages {
        println!("{} {}", key, value);
    }


}
