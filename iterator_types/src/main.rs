fn main() {
    // iter - we can access the elements in a collection without modifying or consuming 
    // by borrowing its elements
    let vec = vec![1,2,3,4,5];

    for item in vec.iter() {
        println!("{}", item);
    }

    // iter_mut - with this we can modify the values inside of a collection
    let mut vec = vec![6,7,8,9];

    for item in vec.iter_mut() {
        *item *= 3;
        println!("{}", item);
    }

    // into_iter -> consumes the collection - after that collection will no longer be available
    let vec = vec![2,3,4,5];

    for item in vec.into_iter() { 
        println!("{}", item);
    }
    
    // let my_value = vec.get(0);  error
}
