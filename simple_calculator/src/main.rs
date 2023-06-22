// Simple Calculator using enums and pattern matching
enum Calculate {
    Quit,
    Add,
    Substract,
    Multiply,
    Divide,
}


fn process(calc: Calculate, x:f64, y:f64) -> Option<f64>{ // result or none (zero divison error or Quit)
    match calc {
        Calculate::Quit => {
            None
        }
        Calculate::Add => {
            Some(x + y)
        }
        Calculate::Substract => {
            Some(x - y)
        }
        Calculate::Multiply => {
            Some(x * y)

        }
        Calculate::Divide=> {
            if y == 0.0 {
                None
            } else {
                Some(x / y)
            }
            
        }
    }
}

fn main() {
    let number = 20.0;
    let number2 = 2.0;

    let operator = Calculate::Multiply;
    
    // let operator = Calculate::Divide; if number2 is 0, then returns None

    let operator = Calculate::Substract;
    let number2 = 21.0;

    // checking the return values
    match process(operator, number, number2) {
        Some(result) => println!("The result is {}", result),
        None => println!("ZeroDivisonError or Quited"),
    }
    
}
