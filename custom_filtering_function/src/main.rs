fn main() {
    let values = vec![2,3,4,5,6,8];
    
    let value = FilterCondition {desired_value: 2};

    let filtered_result = custom_filter(&values, &value);

    println!("filtered result {:?}", filtered_result);
}

// Define FilterCondition struct
struct FilterCondition {
    // fied for filtering
    desired_value: i32,
}

// implement is_match method
impl FilterCondition {
    fn is_match(&self, item: &i32) -> bool {
        item == &self.desired_value
    }
}

// define the custom_filter function
fn custom_filter(values: &Vec<i32>, condition: &FilterCondition) -> Vec<i32> {
    let mut filtered_vec = Vec::new();
    for item in values {
        if condition.is_match(item) {
            filtered_vec.push(*item); // pushing the value in the item reference
        }
    }
    filtered_vec
}
