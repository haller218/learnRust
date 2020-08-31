

fn divisible_of (){
    let num = 7;

    if num % 4 == 0 {
        println!("the number is divisible by 4");
    } else if num % 3 == 0 {
        println!("the number is divisible by 3");
    } else if num % 2 == 0 {
        println!("the number is divisible by 2");
    } else {
        println!("the number is NOT divisible by 2, 3 and 4");
    }
}

fn greather_than_zero(){

    let num = 6;
    //// result in a error because rust not convert
    //  not bool to bool
    // if num {

    if num != 0 {
        println!("the number is something other than zero");
    }
}



fn if_num_value() {

    let num = 6;

    if num > 5 {

        println!("condiction has true");
    } else {

        println!("condiction has false");
    }

//    println!("Hello, world!");
}

fn main (){
    if_num_value();
    greather_than_zero();
    divisible_of();
}
