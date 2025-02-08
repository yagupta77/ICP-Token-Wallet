use candid::{CandidType, Deserialize};
use ic_cdk::query;
use ic_cdk::update;
use std::collections::HashMap;
use ic_cdk::export_candid; // ✅ Correct import

type Address = String;
type TokenAmount = u64;

#[derive(CandidType, Deserialize, Clone)]
struct Account {
    balance: TokenAmount,
}

#[derive(Default)]
struct Wallet {
    accounts: HashMap<Address, Account>,
}

impl Wallet {
    fn get_balance(&self, address: &Address) -> TokenAmount {
        self.accounts.get(address).map_or(0, |acc| acc.balance)
    }

    fn send_tokens(&mut self, sender: Address, receiver: Address, amount: TokenAmount) -> Result<(), String> {
        let sender_balance = self.accounts.entry(sender.clone()).or_insert(Account { balance: 0 });

        if sender_balance.balance < amount {
            return Err("Insufficient funds".to_string());
        }

        sender_balance.balance -= amount;
        let receiver_balance = self.accounts.entry(receiver).or_insert(Account { balance: 0 });
        receiver_balance.balance += amount;

        Ok(())
    }

    fn receive_tokens(&mut self, receiver: Address, amount: TokenAmount) {
        let receiver_balance = self.accounts.entry(receiver).or_insert(Account { balance: 0 });
        receiver_balance.balance += amount;
    }
}

// Global Wallet instance
thread_local! {
    static WALLET: std::cell::RefCell<Wallet> = std::cell::RefCell::new(Wallet::default());
}

// ✅ Exported ICP Canister Functions
#[query]  
fn get_balance(address: Address) -> TokenAmount {
    WALLET.with(|wallet| wallet.borrow().get_balance(&address))
}

#[update]
fn send_tokens(sender: Address, receiver: Address, amount: TokenAmount) -> Result<(), String> {
    WALLET.with(|wallet| wallet.borrow_mut().send_tokens(sender, receiver, amount))
}

#[update]
fn receive_tokens(receiver: Address, amount: TokenAmount) {
    WALLET.with(|wallet| wallet.borrow_mut().receive_tokens(receiver, amount));
}

// ✅ Export Candid Interface (Keep it at the END of the file)
export_candid!();
