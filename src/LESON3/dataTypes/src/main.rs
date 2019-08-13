fn test_parsing_type() {
    // parse type

    let guess: u32 = "42".parse()
        .expect("Not a Number!");

    // type anotation geves a error for more information
    // to compiler to decide the type of date where
   // let guess = "42".parse().expect("Not a number!");
    
    println!("{}",guess);

    
}

fn main () {

    test_parsing_type();

}
