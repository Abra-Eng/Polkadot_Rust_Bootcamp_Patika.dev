fn main() {
    
    let sum = add(4, 3);
    println!("The sum is {}", sum);

    no_param();

    // control statements

    let day_of_the_week = "Sunday";

    if day_of_the_week == "Sunday" {
        println!("Festival time");
    } else if day_of_the_week == "Saturday" {
        println!("Prepare for the festival");
    } else {
        println!("Wait for the weekend");
    }

    // Loops
    
    // while
    let mut counter = 0;

    while counter < 5 {
        println!("The counnter is {}",counter);
        counter += 1;
    }

    // for; 
    //let numbers: [i32; 5] = [1,2,3,4,5];
    //for number in numbers {} 

    for number in 1..5 { // to include 5 -- 1..=5
        println!("number is {}",number);
        
    }

    // loop - infinite loop

    counter = 0;
    loop {
        println!("Counter is {}",counter);
        counter += 1;

        if counter == 6 {
            break;
        } 
    }
    
    // match
    
    let num = 5;

    match num {
        1 => println!("the num is one"),
        2 => {
            println!("the num is two"); // multiple match arms
            println!("TWO");
        }
        3 => println!("the num is three"),
        _ => println!("the num is something else"), // using comma (,) at the end is a choice
    }

    let result = match num {
        1 => "the number is one",
        2 => "the number is two",
        3 => "the number is three",
        _ => "something else"
    };

    println!("{}",result);

}

// functions
fn add(x: i32, y: i32) ->i32 { // type of return value
    let result = x + y;
    return result;
    
}

fn no_param() ->i32 {
    println!("No parameter");
    1 // return 1;
}
