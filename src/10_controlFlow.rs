// Control Flow
// Conditions - if ... else
// Repetition - loop, while, for

#![allow(warnings)]

fn main() {
    // let age: u16 = 18;
    // // Condition
    // if age >= 18 {
    //     println!("You are an adult");
    // } else {
    //     println!("You are a minor");
    // }
    
    // Multiple conditions with else if
    let number = 6;
    if number % 4 == 0 {
        println!("Number is divisible by 4");
    } else if number % 3 == 0 {
        println!("Number is divisible by 3");
    } else if number % 2 == 0 {
        println!("Number is divisible by 2");
    } else {
        println!("Number is not divisible by 4, 3, or 2");
    }

    // Condition
    let condition = true;
    // let number = if condition {5} else {'six'};  // If and else have incompatible types
    let number = if condition {5} else {6};  // If and else have compatible types
    println!("The value of number is: {}", number);
}
