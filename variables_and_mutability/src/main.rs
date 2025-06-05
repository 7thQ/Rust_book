
use colored::*;

//consts log
const GLOBAL_SCOPE_CONSTANT : i32 = 3;

fn main() {

    let mut x = 5;
    println!("{}{x}", "x in the main Function is: ".green());
    x = 6;
    println!("{} {x}, {}", "x is now in the main function:".red(), "Do to mutablity specified for the variable".red());


    println!("{} {GLOBAL_SCOPE_CONSTANT}", "value of a const written in the global scope:".green());
    
    shadowing();

}




fn shadowing() {
    let x = 5;

    println!("{}{x}", "X is not made mutable in this new functions scope instead it is shadowed this is X now: ".white());
    
    let x = x + 1;
    
    {
        let x = x * 2;
        println!("{} {x}", "The value of x shadowed & inside this new function that is also in a deeper inner scope with in the new funtion is:".red());
    }

    println!("{} {x}", "The value of x in the new funtions regualer scope is:".green());


    let spaces = "    ";
    let spaces = spaces.len();

    println!("amount of spaces in the variable Spaces: {spaces}");

}

