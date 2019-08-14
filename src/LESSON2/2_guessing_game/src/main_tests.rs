use std::io;

fn exemplo () {

    let foo = "bar"; // imutable

//    foo = "11"; // Error, cannot assing twice to immutable variable

    println!("{}",foo);
}

fn test_input_io() {

    
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new(); // mutable

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}

fn test_curly_brackets () {

    let x = 5;
    let y = 10;

    println!("x = {} and y = {}", x, y);

}

fn main () {

    //// test Values and frist part of program is ok
    // now generating a Secret Number
    
    exemplo();

    test_curly_brackets ();
    
    test_input_io();

}
