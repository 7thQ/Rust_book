
use core::{error, num};
use std::io;
use colored::*;

fn main() {

    let guess: u32 = "42".parse().expect("Not a number");
    println!("{guess}");

    let x = 64.0;
    let y: f32 = 34.0;

    println!(" floating points x is: {x}, & y is: {y}");

    let sum =  5 + 10;
    let differences =   5 - 10;
    let product = 5 * 10;
    let quotient= 5.0  / 10.0;
    let truncated = -5 / 3; // still gives you the whole # even if you dont syntax it as a floating point, it just drops the decimal 1.66 -> 1
    let remainder = 5 % 10; // still gives remainder even if you dont syntax it as a floating point (5.0 % 10.0) 0.5 -> 5

    let t = true;
    let f: bool = false;
    
    println!("{}", format!("Boolean values: t is {}, f is {}", t, f).bright_yellow());

    let c = 'z';
    let z: char = 'Z';
    let emoji = '😻';

    println!(" char type variables {c} {z} {emoji}");

    let tup = (500, 4.1, 1);
    let ( _x, y, _z) = tup;
    println!("the values of y in this tuple is: {y}");

    let x: (i32, f64, &str, char ) = ( 42 , 5.4 , "53 ", '😻'  );
    let a = x.0;
    let b = x.1;
    let c: u32 = x.2.trim().parse().expect("not a valid number");

    println!("tuple indexing: {a}, {b}, {c}, {}", x.3);

    // array type

    let a = [1,2,3,4,5];
    let b: [i64; 5] = [1,4,5,6,7];
    let c = [2;10];

    println!(" array types: {} {}", a[1], c[4]);

    for value in c {
        print!("{} ", value.to_string().red().bold())

    }
    print!("\n");



    

    println!("{}: {} {}, {} {}, {} {}, {} {}, {} {}, {} {}", 
    "Numeric Operations".bold().cyan(),
    "sum".green(), sum.to_string().bright_green(),
    "difference".red(), differences.to_string().bright_red(),
    "product".yellow(), product.to_string().bright_yellow(),
    "quotient".blue(), quotient.to_string().bright_blue(),
    "truncated".magenta(), truncated.to_string().bright_magenta(),
    "remainder".white(), remainder.to_string().bright_white()
    );



    let a = [10,20,30,40,50];

    

    loop {

        println!("{}", "please enter a array index value:".bright_white().bold());
        
        let mut user_index_input = String::new();

        

        io::stdin()
            .read_line(&mut user_index_input)
            .expect("Input a #");
            print!("\n");


        let user_index_input: usize  = match user_index_input.trim().parse() {

            Ok(num) => num,
            Err(_) => { print!("Not a valid # \n"); continue;}

        };

        if user_index_input < a.len() {

            println!("The value of the element at index {} is : {}", user_index_input, a[user_index_input]);
            break;

        }else {
            println!("{} {}", "Array length is:".red(), a.len().to_string().white() );
            continue;
        }


        

    }


    
}
