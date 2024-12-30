// Loops
// Loop
// While
// For

fn main() {

    // Loop - infinite loop
    let mut counter = 0;
    loop{
        println!("Loop forever!");
        counter += 1;
        if counter == 5 {
            break;
        }
    }

    let mut counter = 0;
    let result = loop{
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is: {}", result);

    // Loop Labels to disambiguate between multiple nested loops
    let mut count = 0;
    'counting_up: loop {
        println!("Counting {}!", count);
        let mut remaining = 10;
        loop {
            println!("Remaining {}!", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }

    // While loop
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }

    // Looping through a collection with for loop
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("The value is: {}", element);
    }

}
