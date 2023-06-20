fn main() {
    
    // Booleans
    
    let _is_rust_fun = true;
    let _is_rust_hard = false;

    // Integers - 8-bit to 128 bit, signed and unsigned

    let _x: i32 = 42;
    let _y: u32 = 12;

    // Rust's integer types also have specific min and max values 

    let min_i32 = i32::MIN;
    let max_i32 = i32::MAX;

    println!("The minimum value of i32 is {} and the maximum value is {}.", min_i32,max_i32);

    // addition to std arithmetic operators integers support bitwise operators too.

    // Floating-point numbers - f32, f64

    let _pi: f64 = 3.14159; 
    // support the std arithmetic op. and comparison operators

    // Characters - hold any Unicode character - (') single quotes

    let _letter_a: char = 'a';

    // Strings - &str and String - (" ")
    // &str is a reference to string slice 
    // String is a growable string type

    let _message: &str = "Hello, world!"; // immutable by default

    let mut _name = String::from("Alice"); // mutable

    // Arrays - fixed size, 1 type - ([])

    let numbers: [i32; 5] = [1,2,3,4,5];

    let second_number = numbers[1]; // indexing
    let last_element = numbers[numbers.len() -1 ];
    println!("The second number in the array is {}.", second_number);
    println!("The last number in the array is {}.", last_element);

    // Slices ([])
    let slice = &numbers[2..4]; // contains the indices 2 and 3
    let firs_element = slice[0];
    println!("Firs element of the slice is {}.", firs_element); // 3

    // Tuples - (()) - immutable by default

    let person = ("Drago", 24);

    let name = person.0;
    let age = person.1;
    println!("Name: {}, Age: {}.",name,age);

    let person = (("Drago","Fei"), 30); // nested tuples
    println!("The name's are {} {} and their's age {}",person.0.0,person.0.1, person.1);

    // Unit Type - represents a value that has no meaningful information - (())

    let _result = ();
    

    //Variables - immutable by default

    let _num = 5;
    // num = 6; - gives an error

    let mut _num = 5;
    _num = 6;

    // Shadowing

    let _x = 52;
    let _x = _x + 1;
}
