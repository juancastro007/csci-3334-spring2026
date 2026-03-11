// Juan Castro
// Bank Account

// Create a 'BankAccount' struct w/ methods to:
// deposit, withdraw, and check on balance

#[derive(Debug)]
pub struct BankAccount {
    balance: f64,
}

impl BankAccount {
    pub fn new(initial_balance: f64) -> BankAccount {
        // Implement this method
        BankAccount {
            balance: initial_balance,
        }
    }

    pub fn deposit(&mut self, amount: f64) {
        // Implement this method
        // cant deposit negative amount of money, losing money
        if amount > 0.0 {
            self.balance += amount;
        }
    }

    pub fn withdraw(&mut self, amount: f64) {
        // Implement this method
        // cant withdraw if balance is in the negative, free money
        if amount > 0.0 && amount <= self.balance {
            self.balance -= amount;
        }
    }

    pub fn balance(&self) -> f64 {
        // Implement this method
        // non-mutable self means balance cannot be modified only read
        self.balance
    }

    /*pub fn apply_interest(&mut self, interest: f64) {
        // Implement this method
        if interest > 0.0 {
            self.balance += self.balance * interest;
        }
    }*/
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_account() {
        // Write a test for creating a new account
        let account = BankAccount::new(100.0);

        assert_eq!(account.balance(), 100.0);
    }

    #[test]
    fn test_deposit() {
        // Write a test for depositing money
        let mut account = BankAccount::new(100.0);

        account.deposit(10.0);

        assert_eq!(account.balance(), 110.0);
    }

    #[test]
    fn test_withdraw() {
        // Write a test for withdrawing money
        let mut account = BankAccount::new(100.0);

        account.withdraw(10.0);

        assert_eq!(account.balance(), 90.0);
    }

    // Add more tests here
    /////////////////////////////////////////////////////////////////////////////
    #[test]
    fn test_overdraw() {
        // Write a test for withdrawing more than balance
        let mut account = BankAccount::new(100.0);

        account.withdraw(110.0);

        assert_eq!(account.balance(), 100.0);
    }
    
    #[test]
    fn test_negative_deposit() {
        // Write a test for depositing a invalid amount
        let mut account = BankAccount::new(100.0);

        account.deposit(-50.0);

        assert_eq!(account.balance(), 100.0);
    }

    #[test]
    fn test_negative_withdraw() {
        // Write a test for withdrawing a invalid amount
        let mut account = BankAccount::new(100.0);

        account.withdraw(-50.0);

        assert_eq!(account.balance(), 100.0);
    }

    /*#[test]
    fn test_interest() {
        // Write a test for applying interest
        let mut account = BankAccount::new(100.0);

        account.apply_interest(0.10);

        assert_eq!(account.balance(), 110.0);
    }

    #[test]
    fn test_invalid_interest() {
        // Write a test for invalid interest 
        let mut account = BankAccount::new(100.0);

        account.apply_interest(0.0);

        assert_eq!(account.balance(), 100.0);
    }*/
}