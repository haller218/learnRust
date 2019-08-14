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

    let _x = 5;

    let y = {

        let _x = 3;

        _x + 1
    };

    println!("the value of y is: {}", y);
}

//// 
// Return value type function

fn _five() -> i32 {
    5
}

fn _testing_return_i32_values () {

    let x = _five();

    println!("Value of x is: {}", x);
}

fn _plus_one (x: i32) -> i32 {

    x + 1
}

fn main () {

    let x = _plus_one (5);

    println!("Value of x is: {}", x);
}

// the code not needs the previus declaration in initial scorpe of the file,
// it's just define then and run.

