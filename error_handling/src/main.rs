use std::fs; 

fn main() {
    let my_content = getFileContent("my_file.txt");

    // if something goes wrong and we don't want our program to stop then we use a match statement
    match my_content {
        Ok(item) => println!("Result {}", item),
        Err(error) => println!("There is an error"),
    }
}
fn getFileContent(file_name: &str) -> Result<String, std::io::Error> {
    // ?  => if everything goes smoothly, we get our file's content but if there is an error ? return this error 
    let content = fs::read_to_string(file_name)?; 
    Ok(content)
}

fn divide(numerator: f64, denominator: f64) -> Option<f64> {
    if denominator == 0.0 {
        return None
    } else {
        return Some(numerator / denominator);
    }
}
