// following the steps as in the rust struct testing lab
mod bank_account;
use bank_account::BankAccount;

use std::io; // use standard input/output

fn main() {
    let mut account = BankAccount::new(100.0);
    
    println!("Initial balance: {}", account.balance());

    loop {
        println!("\nBank of Banking");
        println!("1. Deposit");
        println!("2. Withdraw");
        println!("3. Balance");
        println!("4. Exit\n");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read input");

        let choice: i32 = choice.trim().parse().unwrap_or(0);

        match choice {
            //deposit
            1 => {
                println!("Enter deposit amount:");

                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Failed");

                let amount: f64 = input.trim().parse().unwrap_or(0.0);

                if amount <= 0.0 {
                    println!("Invalid amount");
                }
                else {
                    account.deposit(amount);
                    println!("Successfully deposited");
                }

            }

            //withdraw
            2 => {
                println!("Enter withdraw amount:");

                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Failed");

                let amount: f64 = input.trim().parse().unwrap_or(0.0);

                if amount <= 0.0 {
                    println!("Invalid amount");
                }
                else if amount > account.balance() {
                    println!("Insufficient funds");
                }
                else {
                    account.withdraw(amount);
                    println!("Successfully withdrawn");
                }

            }

            //check balance
            3 => {
                println!("Current balance: {}", account.balance());
            }

            //exit
            4 => {
                println!("Goodbye!");
                break;
            }

            //rand op
            _ => {
                println!("Invalid");
            }
        }
    }
}
