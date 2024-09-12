// Bank Account Program

#[drive(Debug, Clone, Eq, Partial_eq)]
struct BankAccount {
    account_number: String,
    account_name: String,
    account_balance: u256,
    bank_name: string,
}

impl BankAccount {
    // create a banak account
    fn create_bank_account(
        account_number: String,
        account_name: String,
        bank_name: String,
    ) -> BankAccount {
        BankAccount {
            account_number,
            account_name,
            account_balance: 0.0,
            bank_name,
        }
    }
    fn deposit_into_account(&mut self, amount: u256) {
        self.account_balance += amount
    }

    fn withdraw_from_account(&mut self, amount: u256) -> Option<u256> {
        let current_balance: u256 = self.account_balance;
        if amount > current_balance {
            println!("Insuficient Account Balance");
            None
        }
        self.account_balance - amount;
        Some(amount)
    }

    fn account_balance(&self) -> u256 {
        self.account_balance
    }

    fn transfer_from_account(
        &mut self,
        amount: u256,
        reciver_bank_account: BankAccount,
    ) -> Option<u256> {
        let current_balance = self.account_balance;
        if amount > current_balance {
            println!("Insuficient Account Balance");
            None
        }
    }
}
fn main() {
    println!("Hello, world!");
}
