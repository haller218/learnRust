fn test_parsing_type() {
    // parse type

    let guess: u32 = "42".parse()
        .expect("Not a Number!");

    // type anotation geves a error for more information
    // to compiler to decide the type of date where
   // let guess = "42".parse().expect("Not a number!");
    
    println!("{}",guess);

    
}

// scalar types represents a single value. Rust has four primary scalar types:
// integers, floating-point numbers, Booleans, and characters. You may
// recognize these from other programing languages. Let's jump into
// how they work in Rust

// Integer Types


// 8-bit 	i8 	u8
// 16-bit 	i16 	u16
// 32-bit 	i32 	u32
// 64-bit 	i64 	u64
// 128-bit 	i128 	u128
// arch 	isize 	usize


// Number literals 	Example
// Decimal 	98_222
// Hex 	0xff
// Octal 	0o77
// Binary 	0b1111_0000
// Byte (u8 only) 	b'A'


fn float_point_test () {

    let x = 2.0; // f64

    let y: f32 = 3.0; // f32

    println!("value of float64 x: {}, value of float32 {}", x, y);
}

fn numeric_operations () {

    // adition
    let sum = 5 + 10;

    // subtraction
    let diference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 57.7 / 32.2;

    // remainder
    let remainder = 43 % 5;

    println!("Results: {}, {}, {}, {}, {}", sum, diference, product,
             quotient, remainder)
    
}

//// the boolean type

fn booleans () {


    let t = true;

    let f: bool = false; // the explicity type anotation

    println!("Values: {}, {}", t, f);
}

// the caracter type;;;

fn caracter_testing () {

    let c = 'z';

    let z = 'Z';

    let heart_eyed_cat = '😻';

    println!("Values: {}, {}, {}", c, z, heart_eyed_cat);
}

////// Compound Types.

fn the_tuple_type (){

    let _tup: (i32, f64, u8) = (500, 6.4, 1);
    let _tup = (500, 6.4, 1);

    let (_x, y, _z) = _tup;

    println!("the value of y is : {}",y);

    let _five_hundred = _tup.0;

    let _six_point_for = _tup.1;

    let _one = _tup.2;
}

//// The Array Type.

fn arrays () {
    let _array = [1,2,3,4,5];
}

fn arrays_assing () {

    let _months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];

    let _a: [u8; 5] = [1,2,3,4,5];

}

fn main () {

    test_parsing_type();

    float_point_test();

    numeric_operations();

    booleans();

    caracter_testing();

    the_tuple_type();

    arrays();
    
    arrays_assing();
}
