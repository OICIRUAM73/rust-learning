fn main() {
    //mutability
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    
    //shadowing
    let y = 5;
    let y = y + 1;
    let y = y + 2;
    println!("The value of y is: {}", y);

    let spaces = "   ";
    let spaces = spaces.len();

    //data types
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;

    // remainder
    let remainder = 43 % 5;

    let t = true;

    let f: bool = false; // with explicit type annotation
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("{}",heart_eyed_cat);

    //tuple type
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let tup2 = (500, 6.4, 1);

    let (x, y, z) = tup2;

    println!("The value of y is: {}", y);

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;

    //array type
    let a = [1, 2, 3, 4, 5];
    
    let b: [i32; 5] = [1, 2, 3, 4, 5];
    let a = [3; 5]; //5 elements with value 3

    let first = a[0];
    let second = a[1];

    let index = 10;

    let element = a[index]; // runtime error

    println!("The value of element is: {}", element);
}

