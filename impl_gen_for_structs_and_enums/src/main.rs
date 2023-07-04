fn main() {
    let int_container = Container {value: 2};
    let str_container = Container {value: "My lucky number".to_string()};

    println!("{}", int_container.value);
    println!("{}", str_container.value);



    let success_result: MagicalResult<i32, String> =  MagicalResult::Success(42);
    let failure_result: MagicalResult<i32, String> = MagicalResult::Failure("Failed".to_string());
    
    match success_result {
        MagicalResult::Success(value) => println!("Success {}", value), // prints Success 42
        MagicalResult::Failure(value) => println!("Failure {}", value),
    }

    match failure_result {
        MagicalResult::Success(value) => println!("Success {}", value), 
        MagicalResult::Failure(value) => println!("Failure {}", value), // prints Failure Failed
    }



    let mut telsa = ElectricCar {battery_level: 50}; // we are going to alter the level
    let mut volsa = GasCar {gas_level: 34}; 

    telsa.refuel(43);
    volsa.refuel(0.3);
}

struct Container<T> {
    value: T,
}

// enums with generics

enum MagicalResult<T, E> {
    Success(T),
    Failure(E),
}


//Generics and Associated types

trait Vehicle {
    type Fuel;

    fn refuel(&mut self, fuel: Self::Fuel);
}

struct ElectricCar {
    battery_level: u32,
}

struct GasCar {
    gas_level: u32,
}

impl Vehicle for ElectricCar {
    type Fuel = u32;

    fn refuel(&mut self, charge: Self::Fuel) {
        self.battery_level += charge;
        println!("Charged to {}%", self.battery_level);
    }
}

impl Vehicle for GasCar {
    type Fuel = f64;

    fn refuel(&mut self, gas: Self::Fuel) {
        self.gas_level += (gas * 100.0) as u32;
        println!("Gas tank filled to {}%", self.gas_level);
    }
}

