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
    fn bank_account_details (&self) -> (&str, &str, u32, &str) {
        (&self.account_number, &self.account_name, self.account_balance, &self.bank_name)
    }
}
fn main() {
        // create account
    let mut account1 = BankAccount::create_bank_account("9143456123".to_string(), "Mubarak23".to_string(), "Access Bank".to_string());
    println!("New user Open account with ${:?}", account1.account_balance());
    
    // Depoosit amount and check balance
   account1.deposit_into_account(5000);
    println!("Balance after making a deposit  is ${}", account1.account_balance());

    // Check account 1 details
    let (account_number, account_name, account_balance, bank_name) = account1.bank_account_details();
    println!("Account 1 details: Number: {}, Name: {}, Balance: ${}, Bank Name: ${}", account_number, account_name, account_balance, bank_name);

    // Create new account
    let mut account2 = BankAccount::create_bank_account("9876543201".to_string(), "Mubitech".to_string(), "Opay".to_string());
    println!("New user Open account balance ${:?}", account2.account_balance());
    println!("Account 2 details {:?}", account2.bank_account_details());


    // Before transfer
    println!("Before transfer:");
    println!("Account 1 balance: ${}", account1.account_balance());
    println!("Account 2 balance: ${}", account2.account_balance());
     

    // Perform transfer
    let transfer_amount = 500;
    match account1.transfer_from_account(transfer_amount, account2.clone()) {
        Some(amount) => println!("Successfully transferred ${}", amount),
        None => println!("Transfer failed"),
    }

    println!("After transfer:");
    println!("Account 1 balance: ${}", account1.account_balance());
    println!("Account 2 balance: ${}", account2.account_balance());



    println!("Hello, world!");
}
