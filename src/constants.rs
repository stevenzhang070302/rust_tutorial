// Constants


fn main() {
    println!("Hello, world!");
    let mut x = 5;
    const Y: i32 = 10; // Const cannot be mutable and needs type
    println!("The value of x is: {}", x);
    println!("The value of Y is: {}", Y);
    println!("The value of PI is: {}", PI);
    println!("The value of THREE_HOURS_IN_SECONDS is: {}", THREE_HOURS_IN_SECONDS);
}



// You can declare const in global scope
const PI: f32 = 3.14159;
const THREE_HOURS_IN_SECONDS: i32 = 60 * 60 * 3;