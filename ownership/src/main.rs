fn main() {  // s is not valid here, it’s not yet declared
    let s = "hello";   // s is valid from this point forward
    // do stuff with s
    println!("{}", s);
    let mut a = String::from("hello");

    a.push_str(", world!"); // push_str() appends a literal to a String

    println!("{}", a); // This will print `hello, world!`

    // MOVE INTERACT
    let s1 = String::from("hello");
    let s2 = s1;

    // println!("{}, world!", s1); //Fails: value borrowed here after move

    // CLONE INTERACT
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

    // COPY INTERACT
    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);

}// this scope is now over, and s is no longer valid