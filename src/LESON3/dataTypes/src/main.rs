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

fn main () {

    test_parsing_type();

    float_point_test();

    numeric_operations();

    booleans();

    caracter_testing();
}
