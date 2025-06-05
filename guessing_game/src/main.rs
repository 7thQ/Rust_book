




use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored::Colorize;




fn main() {

    println!("Guess the #");

    let  secret_number = rand::thread_rng().gen_range(1..=100);

    // println!("The computer generated: {secret_number}");

    loop {
        println!("Input your guess.");


        let mut guess = String::new();

        io::stdin()

            .read_line(&mut guess)
            .expect("Failure to read line.");




        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {println!("{}","Fucking idoit try again".red()); continue;}
        };



        println!("Your guess: {}", guess);



        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}","Too small".red()),
            Ordering::Greater => println!("{}","Too big like my COCK!".red()),
            Ordering::Equal => {println!("{}","Perfect size!".green()); break; }
        }
    }
}
