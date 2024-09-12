// Bank Account Program

#[derive(Debug, Clone, Eq, PartialEq)]
struct BankAccount {
    account_number: String,
    account_name: String,
    account_balance: u32,
    bank_name: String,
}

impl BankAccount {
    // create a banak account
    fn create_bank_account(
        account_number: String,
        account_name: String,
        bank_name: String,
    ) -> BankAccount {
        let new_account_balance: u32 = 0;
        BankAccount {
            account_number,
            account_name,
            account_balance: new_account_balance,
            bank_name,
        }
    }
    fn deposit_into_account(&mut self, amount: u32) {
        self.account_balance += amount
    }

    fn withdraw_from_account(&mut self, amount: u32) -> Option<u32> {
        let current_balance: u32 = self.account_balance;
        if amount > current_balance {
            println!("Insuficient Account Balance");
           return None;
        }
       let _ = self.account_balance - amount;
       return Some(amount);
    }

    fn account_balance(&self) -> u32 {
        self.account_balance
    }

    fn transfer_from_account(
        &mut self,
        amount: u32,
        reciver_bank_account: BankAccount,
    ) -> Option<u32> {
        let current_balance = self.account_balance;
        if amount > current_balance {
            println!("Insuficient Account Balance");
           return None;
        }
        
        // remove from sender account
        let _ = self.account_balance - amount;
        //send the reciver account
        let _ = reciver_bank_account.account_balance + amount;
        return Some(amount);
    }
}
fn main() {
    println!("Hello, world!");
}
