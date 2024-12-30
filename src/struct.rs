// Structs

fn main() {
    

    // Tuple
    let rect: (i32, i32) = (200, 500);

    // Struct
    struct Book {
        title: String,
        author: String,
        pages: u32,
        avaliable: bool,
    }

    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }

    // Create an instance of the struct
    let mut user1: User = User{  // Stuc has to be mutable if we want to update contents
        username: String::from("user1"),
        email: String::from("someusername@m.com"),
        sign_in_count: 1,
        active: true,
    };

    user1.email = String::from("anotheremail@gmail.com");
    println!("User1 email: {}", user1.email);

    // Return a struct from a function
    fn build_user(email: String, username: String) -> User {
        User {
            email,  // Field init shorthand
            username,
            active: true,
            sign_in_count: 1,
        }
    }

    // Create instances from other instances
    let user2: User = User {
        email: String::from("anotheremail@m.com"),
        ..user1  // Use the rest of the fields from user1
    };

    // Tuple structs
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black: Color = Color(0, 0, 0);
    let white: Color = Color(255, 255, 255);

    // Unit-like structs
    struct AlwaysEqual;
    let subject: AlwaysEqual = AlwaysEqual;

}
