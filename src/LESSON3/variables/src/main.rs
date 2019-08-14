fn tests_using_let () {

    let mut x = 5;

    println!("The value of x is: {}",x);

    x = 6;

    println!("The value of x is: {}",x);

}


fn test_using_const () {

    const MAX_POINTS: u32 = 100_000;

    println!("Max const {}", MAX_POINTS);

}

fn test_shadowed () {

    let x = 5;

    let x = x + 1;

    let x = x * 2;

    println!("The value of x is: {}", x);
}

fn test_change_datatype () {

    let spaces = "  ";

    let spaces = spaces.len();
}

fn main () {

    tests_using_let();

    test_using_const();

    test_shadowed ();

    test_change_datatype ();

}
