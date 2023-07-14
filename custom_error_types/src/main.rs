fn main() {

    let my_error = RocketError::OutOfFuel;
    handle_error(my_error);
}

// A basic usage
fn divide(numerator: f64, denominator: f64) -> Result<f64, &'static str> {
    if denominator == 0.0 {
        Err("ZeroDivisionError")
    } else {
        Ok(numerator / denominator)
    }

}

// triggering a spesific response when certain error occurs
enum RocketError {
    OutOfFuel,
    NavigationSystemFailure,
    AlienInvasion,
}

fn handle_error(error: RocketError) {
    match error {
        RocketError::OutOfFuel => {
            println!("Out of Fuel!");
            // Fuel replenishment procedures
        }
        RocketError::NavigationSystemFailure => {
            println!("Navigation Error!");
            // Navigation system diagnostic
        }
        RocketError::AlienInvasion => {
            println!("Alien Attack!");
            // intergalactical diplomacy protocols
        }
    }
}
