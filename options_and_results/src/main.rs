// we use enum Option to see whether a value is present or absence

fn find_square_root(number: f64) -> Option<f64> {
    if number >= 0.0 {
        Some(number.sqrt())
    } else {
        None // indicates that something is wrong
    }
}

// Result is using for if there is an error to show or return error  

fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("Division by zero is not allowed".to_string())
    } else {
        Ok(a / b)
    }  
}

// a simple program to calculate the are of triangle

fn get_from_database(key: &str) -> Option<f64> {
    // an array that contains a key and the value
    let database: [(&str, Option<f64>); 2] = [("base", Some(4.0)), ("height", Some(6.0))]; 

    for (k, v) in database {
        if k == key {
            return v;
        }
    }
    None
}

fn calculate_tri_area(base: Option<f64>, height: Option<f64>) -> Result<f64, String> {
    match (base, height) {
        (Some(b), Some(h)) => {
            if b <= 0.0 || h <= 0.0 {
                Err("Both base and height must be positive numbers".to_string())
            } else {
                Ok(0.5 * b * h)
            }
        }
        // Checking if either one of the values are missing
        (None, _) => Err("Base is missing".to_string()),
        (_, None) => Err("Height is missing".to_string()),
        
    }
}


fn main() {

    // enum Option<T> { T placeholder for data type 
    //     Some(T),
    //     None,
    // } we don't need to implement this for using Option , it's preimplemented

    let number = -4.0;
    let square_root = find_square_root(number);

    match square_root {
        Some(value) => println!("The square root of {} is {}", number, value),
        None => println!("The square root of {} is not a real number", number)
    }

    // enum Result<T, E> {
    //     Ok(T),
    //     Err(E),
    // } same like Option

    let a = 10.0;
    let b = 0.0;
    let division_result = divide(a, b);

    match division_result {
        Ok(value) => println!("{} is divided by {} is {}", a, b, value),
        Err(error_msg) => println!("Error {}", error_msg),
    }

    // calculating tri area

    let base = get_from_database("base");
    let height = get_from_database("height");
    let area_result = calculate_tri_area(base, height);

    match area_result {
        Ok(area) => println!("The are of the triangle is {} square units", area),
        Err(error_msg) => println!("Error {}", error_msg),
    }
}
