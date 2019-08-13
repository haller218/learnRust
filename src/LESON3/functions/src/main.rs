fn _another_function(x: i32, y: i32) {

    // println!("yeah, another function call.!");

    println!("the value of x is: {}", x);

    println!("the value of y is: {}", y);
    
}

////
// function parameters


fn _test_functions() {
    
    // println!("Hello, world!");
    
    _another_function(4,5);
}

////
// this is a statement
////
// statement do not return a value.

fn _statement_function () {

    let _a: i32 = 42;
}

////
// Function Bodies Contain Statements and Expressions

fn _functions_bodies_statements () {

    let x = 5;

    let y = {

        let x = 3;

        x + 1
    };

    println!("the value of y is: {}", y);
}


fn main () {

    // https://doc.rust-lang.org/book/ch03-03-how-functions-work.html#functions-with-return-values
}
