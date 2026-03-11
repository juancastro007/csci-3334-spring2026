// following the steps as in the rust struct testing lab
mod bank_account;
use bank_account::BankAccount;

use std::io // use standard input/output

fn main() {
    let mut account = BankAccount::new(100.0);
    
    println!("Initial balance: {}", account.balance());

    loop {
        println!("Bank of Banking");
        println!("1. Deposit");
        println!("1. Withdraw");
        println!("1. Balance");
        println!("1. Exit");

        //
        let mut choice = String::new();
        io:stdin().read_line(&mut choice).expect("Failed to read input");

        let choice: i32 = choice.trim().parse().unwrap_or
    }
}
