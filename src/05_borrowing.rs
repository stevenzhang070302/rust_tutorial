// References and Borrowing
// Safety and Performance
// Borrowing and references are same

// References and Borrowing
// References allow you to refer to some value without taking ownership of it.
// Borrowing is the act of creating references.
// Mutable or inmutable references - &mut T or &T
fn main() {
    let mut x: i32 = 5;
    // let r: i32 = x; // r is a copy of x or transfers the ownership of x to r so x doesnt exist anymore
    let _r: &mut i32 = &mut x; // r is a reference to x

    // Increment x by 1
    *_r += 1; // error: cannot assign to immutable borrowed content `*_r`
    *_r -= 3;

    println!("The value of x is: {}", x);
    // Either one mutable reference or any number of immutable references
    // println!("The value of r is: {}", _r);  // This will not work because _r is a reference

    // Bank Account
    let mut account = BankAccount {
        owner: "John Doe".to_string(),
        balance: 100.0,
    };
    // Immutable borrow to check the balance
    account.check_balance();

    // Mutable borrow to withdraw money
    account.withdraw(50.0);

    // Immutable borrow to check the balance
    account.check_balance();
}





// Struc
struct BankAccount {
    owner: String,
    balance: f64,
}

impl BankAccount {
    // No simultaneous mutable and immutable borrows - end of scope
    fn withdraw(&mut self, amount: f64){
        println!("Withdrawing ${} from account of {}", amount, self.owner);
        self.balance -= amount;
    }

    // Immutable access
    fn check_balance(&self) {
        println!("The balance of account of {} is ${}", self.owner, self.balance);
    }
}
