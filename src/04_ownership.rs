// Ownership
// Every value has a single owner - every value can only have one owner at a time
// 1. Each value in Rust has a variable that’s called its owner.
// 2. There can only be one owner at a time.
// 3. When the owner goes out of scope, the value will be dropped.

fn main() {
    let s1 = String::from("Rust");
    let len = calculate_length(&s1);  // Acessing the value of s1 by passing a 
    // reference to it
    let s2 = s1;  // s1 no longer is the owner of the value
    // println!("The length of '{}' is {}.", s1, len);
    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}


// cannot find s1 in scope
// fn main() {
//     let s1 = String::from("RUST");
//     let len = calculate_length(&s1);
//     println!("The length of '{}' is {}.", s1, len);
// }

// fn printLost(s: &String) {
//     println!("{}", &s1);
// }

// fn calculate_length(s: &String) -> usize {
//     s.len()
// }