use crate::*;

const ACCOUNT_KEEPALIVE_BALANCE: Balance = 1_000_000_000_000_000_000_000;

pub(crate) fn assert_one_yocto() {
    assert_eq!(
        env::attached_deposit(),
        1,
        "Requires attached deposit of exactly 1 yoctoNEAR"
    )
}

pub(crate) fn assert_self() {
    assert_eq!(
        env::predecessor_account_id(),
        env::current_account_id(),
        "Method is private"
    );
}

impl Contract {
    pub(crate) fn internal_deposit(&mut self, account_id: &AccountId, amount: Balance) {
        self.try_register_user(account_id);
        let balance = self
            .accounts
            .get(&account_id)
            .expect("Internal Error! The account is not registered");
        if let Some(new_balance) = balance.checked_add(amount) {
            self.accounts.insert(&account_id, &new_balance);
        } else {
            env::panic(b"Balance overflow");
        }
    }

    pub(crate) fn internal_withdraw(&mut self, account_id: &AccountId, amount: Balance) {
        let balance = self
            .accounts
            .get(&account_id)
            .expect("The account is not registered");
        if let Some(new_balance) = balance.checked_sub(amount) {
            self.accounts.insert(&account_id, &new_balance);
            self.try_unregister_user(account_id);
        } else {
            env::panic(b"The account doesn't have enough balance");
        }
    }

    pub(crate) fn internal_transfer(
        &mut self,
        sender_id: &AccountId,
        receiver_id: &AccountId,
        amount: Balance,
        memo: Option<String>,
    ) {
        assert_ne!(
            sender_id, receiver_id,
            "Sender and receiver should be different"
        );
        assert!(amount > 0, "The amount should be a positive number");
        self.internal_withdraw(sender_id, amount);
        self.internal_deposit(receiver_id, amount);
        env::log(format!("Transfer {} from {} to {}", amount, sender_id, receiver_id).as_bytes());
        if let Some(memo) = memo {
            env::log(format!("Memo: {}", memo).as_bytes());
        }
    }

    fn try_register_user(&mut self, user: &AccountId) {
        if !self.accounts.contains_key(user) {
            self.accounts.insert(user, &0);
            self.account_num += 1;
        }
    }

    fn try_unregister_user(&mut self, user: &AccountId) {
        if let Some(balance) = self.accounts.get(user) {
            if balance < ACCOUNT_KEEPALIVE_BALANCE {
                self.accounts.remove(user);
                self.account_num -= 1;
                self.total_supply -= balance;
            }
        }
    }

    pub(crate) fn assert_owner(&self) {
        assert_eq!(
            env::predecessor_account_id(),
            self.owner_id,
            "Can only be called by the owner"
        );
    }

    pub(crate) fn assert_sudoer(&self) {
        assert_eq!(
            env::predecessor_account_id(),
            self.sudoer_id,
            "Can only be called by the owner"
        );
    }

    pub(crate) fn assert_sudoers(&self) {
        if env::predecessor_account_id() != self.owner_id 
        && env::predecessor_account_id() != self.sudoer_id {
            env::panic(b"only sudoers are allowed!");
        }
    }

    pub(crate) fn is_shop(&self, account_id: &AccountId) -> bool {
        self.shops.contains_key(account_id)
    }
}
