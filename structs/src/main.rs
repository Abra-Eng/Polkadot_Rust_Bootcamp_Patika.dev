fn main() {
    
    // creating an instance of struct
    let film = Film {
        name: String::from("Tenet"),
        director: String::from("Cristopher Nolan"),
        release_year: 2020
    };

    println!("The film is {} and directed by {} at {}", film.name, film.director, film.release_year);

    // mutable instance of struct
    let mut film = Film {
        name: String::from("Tenet"),
        director: String::from("Cristopher Nolan"),
        release_year: 2020
    };

    film.name = String::from("The Prestige");
    film.release_year = 2006;

    println!("The film is {} and directed by {} at {}", film.name, film.director, film.release_year);

    let film_data = get_film_data(film);

    for data in film_data {
        println!("{data}");
    }

    let added_film = add_film("Inception".to_string(), "Cristopher Nolan".to_string(), 2010);

    println!("The film is {} and directed by {} at {}", added_film.name, added_film.director, added_film.release_year);

    println!("The film is {:?}", added_film);

    // creating an instance of tuple struct and accessing it's values
    let tuple_film = TupleFilm("Ferza".to_string(), "Abra".to_string(), 2025);

    let _name = tuple_film.0;
    let _director = tuple_film.1;
    let _release_year = tuple_film.2;

    // instance of unit type struct
    let _unit_film = UnitFilm; // we can attach methods to this type


    // methods calling
    let my_rectangle = Rectangle {
        width: 21.0,
        height: 10.9,
    };

    let area = my_rectangle.area();
    println!("Area: {}", area);

}



// struct 
#[derive(Debug)] // by using {:?} in the println we can see our struct in output
struct Film {
    name: String,
    director: String,
    release_year: u32,
}

// tuple structs - we can use them when we don't need field names 
struct TupleFilm(String, String, u32); 

// unit type struct
struct UnitFilm;

// a function take struct as a parameter and returns a array of strings
fn get_film_data(film: Film) -> [String; 3] {
    let name = film.name;
    let director = film.director;
    let release_year = film.release_year;

    let data: [String; 3] = [name, director, release_year.to_string()];

    // returning array
    data 
}

// a function that returns a struct
fn add_film(name: String, director: String, release_year: u32) -> Film {
    // passing parameters to the our instance
    let film = Film {
        name: name, // if the parameter and variable names are same we can use without pointing again
        director,
        release_year,
    };
    
    // returns a struct named film
    film
}


// method creating
struct Rectangle {
    width: f64,
    height: f64
}
// we define our methods related to the struct at the outside of our struct and using impl Struct_Name
impl Rectangle { 
    fn area(&self) -> f64{
        self.width * self.height
    }
}