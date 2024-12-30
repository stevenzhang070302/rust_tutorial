// Primitive Data Types
// int, float, bool, char

// Integer - signed and unsigned
// i8, i16, i32, i64, i128 - signed i^power via base 2
// u8, u16, u32, u64, u128 - unsigned

fn main() {
    let x: i32 = -42;
    let y: u64 = 100;
    println!("Signed Integer: {}", x);
    println!("Unsigned Integer: {}", y);
    // diff between i32 (32 bits) and i64 (64 bits)
    // range:
    // i32 - 2147483647
    // i64 - 9223372036854775807
    let e: i32 = 2147483647;
    let f: i64 = 9223372036854775807;
    println!("Maximum value of i32: {}", e);
    println!("Maximum value of i64: {}", f);

    // ===============
    // Float
    // f32, f64
    let pi: f32 = 3.14159;
    println!("Float: {}", pi);
    // Boolean values: true, false
    let is_snowing: bool = false;
    println!("Boolean: {}", is_snowing);
    // Character - single quotes
    let letter: char = 'A';
    println!("Character: {}", letter);
}