// Functions

// An functiopns or variables should be writen in snake_case

// Entry Point
fn main() {
    hello_world();
    tell_height(172);
    human_id("John", 25, 172.5);
    // Expression
    let _x: i32 = {
        let price: i32 = 100;
        let qty: i32 = 5;
        price * qty  // Last line in the expression, no semicolons - automatically returns
    };
    println!("Result is: {}", _x);
    add(4, 6);
    let y = add(4, 6);
    println!("Result is: {}", y);
    println!("Result is: {}", add(4, 6));
    // Calling the BMI function
    let weight: f64 = 70.0;
    let height: f64 = 1.72;
    let bmi = calculate_bmi(weight as f64, height as f64);
    println!("BMI is: {:.2}", bmi);
}

// Hoisting is not a thing in Rust - you can't call a function before it's declared
fn hello_world() {
    println!("Hello, world!");
}

// Insert input parameters
fn tell_height(height: u32) {
    println!("You are {} cm tall", height);
}

fn human_id(name: &str, age: u32, height: f32) {
    println!("My name is {}, I am {} years old and {} cm tall", name, age, height);
}

// functions returning values
fn add(a: i32, b: i32) -> i32 {
    a + b
}

// Expressions - returns and Statements - does not return
// Variables decalred globally are consts
// const _X = {
//     // code
// }

// Statements always enf with semicolons
// ley y = let x = 10;  // Statement


// Final Example
// BMI = weight(kg)/height(m)^2
fn calculate_bmi(weight_kg: f64, height_m: f64) -> f64 {
    weight_kg / height_m.powi(2)
}