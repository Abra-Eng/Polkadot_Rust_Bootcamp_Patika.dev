## Custom Filtering Function 

### Steps 
* Create a new Rust project by running the following command in the terminal:

cargo new my_project 

1. Open the main.rs file in a text editor.
2. Define a struct called FilterCondition with a single field of the desired type for filtering.
```rust
struct FilterCondition {
    desired_value: i32,
}
```
3. Implement a method called is_match on the FilterCondition struct that takes a reference to an item of the same type as the filter condition and returns a boolean indicating whether the item matches the condition.
```rust
impl FilterCondition {
    fn is_match(&self, item: &i32) -> bool {
        item == &self.desired_value
    }
}
```
4. Define a function called custom_filter that takes a collection (e.g., a vector) and a reference to a FilterCondition object as arguments. The function should iterate over the elements in the collection and return a new collection containing only the elements that match the filter condition.
```rust
fn custom_filter(values: &Vec<i32>, condition: &FilterCondition) -> Vec<i32> {
    let mut filtered_vec = Vec::new();
    for item in values {
        if condition.is_match(item) {
            filtered_vec.push(*item); 
        }
    }
    filtered_vec
}
```
5. In the main function, create a collection (e.g., a vector) with some elements and initialize a FilterCondition object with the desired value.
```rust
let values = vec![2,3,4,5,6,8];
    
let value = FilterCondition {desired_value: 2};
```
6. Call the custom_filter function with the collection and the FilterCondition object, storing the result in a new variable.
```rust
let filtered_result = custom_filter(&values, &value);
```
7. Print the filtered result to the console.
```rust
println!("filtered result {:?}", filtered_result);
```
8. Compile and run the program to test its functionality.