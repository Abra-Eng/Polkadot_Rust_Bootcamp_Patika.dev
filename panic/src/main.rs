fn main() {
    // let my_vec = vec![1,3];
    // let my_variable = my_vec[5]; // rust panics

    let veggies = ["carrot", "tomato", "potato", "pepper"];

    chooseVeggie(veggies[0]);
    chooseVeggie(veggies[1]);
    chooseVeggie(veggies[2]);
    chooseVeggie(veggies[3]);
    chooseVeggie("eggplant");
}

fn chooseVeggie(veggie: &str) {
    match veggie {
        "carrot" => println!("Carrot"),
        "tomato" => println!("Tomato"),
        "potato" => println!("Potato"),
        "pepper" => println!("Pepper"),
        _ => panic!("Panic!"),
    }
}
