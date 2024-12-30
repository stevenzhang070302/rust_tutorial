// Compound Data Types
// arrays, tuples, slices, and strings (slice string)

// Arrays
// fixed size and fixed type
fn main() {
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Array: {:?}", numbers);  // :? formats the output
    // let mix = [1, 2, "apple", true]; // error: expected integer, found `&str`
    // println!("Mix array: {:?}", mix);
    let fruits: [&str; 3] = ["apple", "banana", "cherry"];
    println!("Fruits array: {}", fruits[0]);
    println!("Fruits array: {}", fruits[1]);
    println!("Fruits array: {}", fruits[2]);

    // Tuples
    // fixed size and mixed type
    // Defining a string here is error because "Alice" is not a string but a slice
    // let human = ("Alice", 30, false);
    let human: (String, i32, bool) = ("Alice".to_string(), 30, false);
    println!("Human Tuple: {:?}", human);
    let my_mix_tuple = ("Der", 30, false, [1,2,3,4,5]);
    println!("My Mix Tuple: {:?}", my_mix_tuple.3);

    // Slices: [1,2,3,4,5]
    // A slice is a reference to a contiguous sequence of elements in a collection
    let number_slices:&[i32] = &[1,2,3,4,5];
    println!("Number Slices: {:?}", number_slices);
    let animal_slices:&[&str] = &["dog", "cat", "bird"];
    println!("Animal Slices: {:?}", animal_slices);
    let book_slices:&[&String] = &[&"book1".to_string(), &"book2".to_string()];
    println!("Book Slices: {:?}", book_slices);

    // Strings vs String Slices(&str)
    // Strings are owned and mutable - expandable
    // String slices are references to strings - immutable
    // Both heap and stack are memory locations
    // stack - fixed size, fast, last in first out - static immutable data types
    // heap - dynamic size, slower, first in first out - dynamic mutable data types
    // All variables in rust are immutable by default - add mut to make it mutable
    let mut stone_cold: String = String::from("Hell, ");  // Stored in heap
    println!("Stone Cold: {}", stone_cold);
    stone_cold.push_str("Yeah!");  // push_str() appends a string slice to a String
    println!("Stone Cold: {}", stone_cold);

    // &str (String Slice) - its a reference to a string also immutable; string slices
    // are used for string literals and string references (pointers) without owning the string/copys
    let string: String = String::from("Hello, World!");
    // &string is a reference to the string
    let string_slice: &str = &string[0..5];  // string slice from 0 to 5
    println!("String Slice: {}", string_slice);
}

// Out of scope

// fn print(){
//     println!("Slice: {}", string_slice);
// }