fn main() {
    //Simple hands-on project 
    // finding maximum value in a array of integers

    // creating the array
    let numbers: [i32; 7]= [2,5,8,19,7,23,1];

    // getting max value from the find_max funtion
    let max_number = find_max(&numbers);
    
    println!("The max number in this slice is {}", max_number);


}

fn find_max(num: &[i32]) -> i32 {
    // set the max_value to the num[0]
    let mut max_value = num[0];
    
    // iterate through other elements in the array
    for &number in num {
        if  number > max_value{
            max_value = number;
        }
    }
    // return the max value
    max_value
}