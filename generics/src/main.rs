use std::clone;

fn main() {
    
}
// happens during compile time
// the binary which we get after compile will be much larger
fn swap<T: Copy>(x: &mut T, y: &mut T){ // T is placeholder
    let temp = *x;
    *x = *y;
    *y = temp;
}



trait Summary {
    fn summarize(&self) -> String;
}

fn print_summary<T: Summary>(item: T) { // works with any type that implements Summary trait, known as Trait bound
    println!("{}", item.summarize());
}

fn print_double<T,U>(item1: T, item2: U)
where
    T: Summary,
    U: Summary + Clone, // needs to implement both type
{
    println!("{}", item1.summarize());
    println!("{}", item2.summarize());
    let cloned_item = item2.clone();
    println!("{}", cloned_item.summarize());
}

enum Option<T>{
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}