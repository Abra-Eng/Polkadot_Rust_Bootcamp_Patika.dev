fn main() {
    let s1 = String::from("Hello");

    let _s2 = s1; // we moved the ownership of String, s1 to s2

    // println!("value of s1 {}", s1); it gives an error

    let x: i32 = 5; // stored in stack memory - static memory
    let y = String::from("Polkadot"); // stored in heap memor - dynamic memory-it can grow and shrink 
    let z = y;
    println!("value of x is {} and value of z is {}", x, z);
    
    //every value on rust has one owner
}
                                              
