fn main()
{
    /*
    let mut x: i32 = 5;
    let r: &mut i32 = &mut x;

    *r = *r + 1; // *r += 1; EQUIVALENT
    println!("Value of x: {}", x);
    //println!("Value of r: {}", r);
    */

    let mut account: BankAccount = BankAccount {
        owner: "Alice".to_string(),
        balance: 150.55,
    };

    // Inmutable borrow to check the balance
    account.check_balance();

    // Mutable borrow to withdraw the money
    account.withdraw(50.55);
    account.check_balance();
}

struct BankAccount {
    owner: String,
    balance: f64,
}

impl BankAccount{
    fn withdraw(&mut self, amount: f64){
        println!("Withdrawing {} from account owned by {}", 
        amount, self.owner);

        self.balance -= amount;
    }
    fn check_balance(&self){
        println!("Account owned by {} has the balance of {:.2}", self.owner, self.balance);
    } 
}