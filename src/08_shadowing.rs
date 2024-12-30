// Shadowing
// You can declare a new variable with the same name as a previous variable, and the new variable shadows the previous variable. Rustaceans say that the first variable is shadowed by the second, which means that the 
// second variable’s value is what appears when the variable is used. Here’s an example:
// Shadowing is diff from marking as mut.
fn main() {
    // println!("Hello, world!");
    let x = 5; // Old x
    let x = x + 1; // New x -> shadowed x
    // As many lets as you want, also can allow changing the type too
    // Mut is not allowed to change the type

    {
        let x = x * 2; // New x -> shadowed x
        println!("The value of x in the inner scope is: {}", x); // 12
    }

    println!("The value of x in the outer scope is: {}", x); // 6


}
