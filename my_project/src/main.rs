fn main() {
    println!("Hello, world!");

    // variable declaration

    let _message ="Hello, world!";

    // built-in data types

    let _x: i32 = 42;
    let _pi: f64 = 3.14159;
    let _is_rust_fun: bool = true;
    let _letter_a: char = 'a';

    // control flow statements

    let x = 42;

    if x>= 0 {
        println!("x is non-negative")
    } else {
        println!("x is negative")
    }

    //functions

    fn add(x: i32,y: i32) -> i32 {// means our return value will be i32 type{
        x + y // alternate usage return x + y; 
    }

    // while loop - counts from zero to 5

    let mut i = 1;
   while i <= 5 {
        println!("{}",i);
        i += 1;
    }

}
