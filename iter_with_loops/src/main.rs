use std::{collections::HashMap, vec};

fn main() {
    // let mut my_map = HashMap::new();

    // my_map.insert("Abra".to_string(), 9);
    // my_map.insert("yutpa".to_string(), 2);

    // for (key,value) in my_map.iter() {
    //     println!("{}  {}", key, value);
    // }

    let numbers = vec![1,2,3,4,5,6];

    // numbers.iter() -> we are getting iterator version of our vector
    // __.map(f) -> for every element in the iterator applies f method
    // __.collect() -> collects the result of iterator and turns it to collection
    let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
    
    println!("{:?}", doubled);

    //*****//
    let numbers1 = vec![1,2,3,4,5,6];

    // __.into_iter.filter(f) -> creates a new iterator containing only the elements that satisfy certain condition
    let even_numbers:Vec<i32> = numbers1.into_iter().filter(|x| x % 2 == 0).collect();
    
    println!("{:?}", even_numbers);

    //******//
    let numbers2 = vec![1,2,3,4,5,6];
    
    // __iter().fold(init, f(accumulate, value)) -> accumulates the elements of an iterator into a single value
    let sum: i32 = numbers2.iter().fold(0, |acc, x| acc + x);

    println!("{}", sum);

    // Chaining iterator methods
    let chained: Vec<i32> = numbers
        .into_iter()
        .filter(|x| x % 2 == 0)
        .map(|x| x * 5)
        .collect();

    println!("{:?}", chained);


    // Converting a vector to HashMap using iter
    let numbers = vec![1,2,3,4,5,6];

    let square_num: HashMap<_,_> = numbers.iter().map(|x| (x, x * x)).collect();

    println!("{:?}", square_num);
}
