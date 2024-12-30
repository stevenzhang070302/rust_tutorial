// Collections

fn main() {

    // Vector
    let _v : Vec<i32> = Vec::new();
    let _theVec:Vec<i32> = vec![1, 2, 3];
    let mut _the_numbers_vec:Vec<i32> = Vec::new();
    _the_numbers_vec.push(5);
    _the_numbers_vec.push(6);
    _the_numbers_vec.push(7);
    _the_numbers_vec.push(8);
    // Vector should have homogeneous data type
    println!("The numbers vector is {:?}", _the_numbers_vec);
    
    let _v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &_v[2];  // Reference because we are not taking ownership
    println!("The third element is {}", third);

    let third: Option<&i32> = _v.get(2);  // Option because we are not taking ownership
    match third {
        Some(third) => println!("The third element from get is {}", third),
        None => println!("There is no third element via get."),
    }

    // String -utf8 encoded
    // First
    let s = "whatever".to_string();
    // 2nd
    let s = String::from("whatever");
    // Mutate the varaible [push to it]
    let mut s = String::from("hello");
    s.push_str(", world!");
    s.push('!');
    println!("The string is {}", s);

    // Combining strings
    let s1: String = String::from("Hello, ");
    let s2: String = String::from("world!");
    let s3: String = s1 + &s2;  // s1 has been moved here and can no longer be used
    println!("The combined string is {}", s3);

    // Storing keys and values with HashMap
    use std::collections::HashMap;
    let mut scores: HashMap<String, i32> = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score: i32 = scores.get(&team_name).copied().unwrap_or(0);
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
}
