// Error Handling

fn main() {


    // Approach 1
    // Formal definition
    // enum Option<T> {  // Define the generic Option type
    //     Some(T),  // represents a value
    //     None,    // represents no value
    // }

    fn divide(numerator: f64, denominator: f64) -> Option<f64> {
        if denominator == 0.0 {
            None
        } else {
            Some(numerator / denominator)
        }
    }

    let result = divide(10.0, 0.0);
    match result {
        Some(value) => println!("Result: {}", value),
        None => println!("Cannot divide by zero"),
    }

    // Approach 2
    // enum Result<T, E> {  // Define the generic Result type
    //     Ok(T),  // represents success
    //     Err(E), // represents error
    // }

    fn divide_v2(numerator: f64, denominator: f64) -> Result<f64, String> {
        if denominator == 0.0 {
            Err("Cannot divide by zero".to_string())
        } else {
            Ok(numerator / denominator)
        }
    }
    
    let result = divide_v2(10.0, 0.1);
    match result {
        Ok(value) => println!("Result: {}", value),
        Err(message) => println!("{}", message),
    }

    println!("Hello, world!");
}
