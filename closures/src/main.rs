fn main() {
    // let my_closure = || println!("Closure"); // |parameter| function;
    // my_closure();

    // ___ = |parameter| -> return type {function body}
    let even_number = |x:i32| -> bool {x % 2 == 0};
   
    let even =  even_number(4);
    let odd = even_number(5);

    println!("is first number even{}", even);
    println!("is second number is even{}", odd);

    //*********//
    let numbers = vec![2,3,4,5,6,7];

    let odd_number: Vec<i32> = numbers.into_iter().filter(|x| x % 2 == 1).collect();
    // fn is_odd(numbers: Vec<i32>) -> Vec<i32> {
    //     let mut odd_numbers_vec = Vec::new();

    //     for number in numbers {
    //         if number % 2 == 1 {
    //             odd_numbers_vec.push(number);
    //         }
    //     }
    //     odd_numbers_vec
    // }

    println!("odd numbers in Vector {:?}", odd_number); 

    // FnOnce
    let print_data = |data: &str| {
        println!("Data: {}", data);
    };

    download_data("NoWhere.dev", print_data);

}


// FnOnce: This trait represents closures that can be called exactly once, use it, and it's gone
// They may move (consume) values from their environment

fn download_data(url: &str, callback: impl FnOnce(&str)) {
    println!("Getting data from {}", url);

    std::thread::sleep(std::time::Duration::from_secs(1));

    let data = format!("Some data from {}", url);

    callback(&data);
} 







// FnMut: This trait is for closures that can be called multiple times and can mutate values from their environment
// Like having a key to the house, we can enter anytime we want and allowed to move furnitures

// Fn: This trait is for closures that can be called multiple times without mutating their environment
// Like being ghost- we can pass through the house as much as we want, but we can't change anything