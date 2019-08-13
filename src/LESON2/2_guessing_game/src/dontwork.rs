
use std::io;
use std::cmp::Ordering;
use rand::Rng;

// Generate Random Number

fn main(){


    let mut guess = String::new();

    let secret_number = rand::thread_rng().gen_range(1,101);

    println!("Please enter to a number: ");

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    // ---- snip ----

    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {

        Ordering::Less => println!("Too small"),
        Ordering::Greater => println!("Too big"),
        Ordering::Equal => println!("You Wins!"),
    }
    
}


