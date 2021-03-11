use crate::*;

#[near_bindgen]
impl Contract {
    /// Mint token to the signer account.
    /// Requirements:
    /// * The signer account should be registered.
    /// * Requires positive attached deposit.
    /// * Requires called by owner
    #[payable]
    pub fn mint_playtoken(&mut self) {
        self.assert_owner();
        let amount = env::attached_deposit();
        assert!(amount > 0, "Requires positive attached deposit");
        let account_id = env::signer_account_id();
        
        // figure out actual mint amount
        let token_mint = amount / self.mint_price as u128 * PRICE_DEMONINATOR as u128;
        let mint_fee = self.burn_ratio.multiply(amount);
        self.owner_profit += mint_fee;
        self.total_collateral += amount - mint_fee;
        self.total_supply += token_mint;
    
        self.internal_deposit(&account_id, amount);

        env::log(format!("Mint {} token to {} cost {} NEAR", token_mint, account_id, amount).as_bytes());
    }

    /// Burn token and get NEAR back to the signer account.
    /// Requirements:
    /// * The signer account should be registered.
    /// * `amount` must be a positive integer.
    /// * The signer account should have at least the `amount` of tokens.
    /// * Requires called by owner
    pub fn burn_playtoken(&mut self, amount: U128) -> Promise {
        self.assert_owner();
        let account_id = env::signer_account_id();
        let amount = amount.into();
        
        self.internal_withdraw(&account_id, amount);

        self.total_supply -= amount;
        let gross_collateral = amount / PRICE_DEMONINATOR as u128 * self.mint_price as u128;
        let mint_fee = self.burn_ratio.multiply(gross_collateral);
        let net_collateral = gross_collateral - mint_fee;
        assert!(self.total_collateral>=net_collateral, "Internal Error! collateral insufficient.");
        self.total_collateral -= net_collateral;

        env::log(format!("Burn {} token get {} NEAR back from {}", amount, net_collateral, account_id).as_bytes());
        // Transfer NEAR
        Promise::new(account_id).transfer(net_collateral)
    }

    pub fn withdraw_owner_profit(&mut self, amount: U128) -> Promise {
        self.assert_owner();
        let amount: u128 = amount.into();
        assert!(amount < self.owner_profit, "Insurfficent profit for withdraw.");
        let transfer_amount: u128;
        if amount > 0 {
            transfer_amount = amount;
            self.owner_profit -= amount;
        } else {
            transfer_amount = self.owner_profit;
            self.owner_profit = 0;      
        }
        env::log(format!("Withdraw owner profit {} ", transfer_amount).as_bytes());
        Promise::new(self.owner_id.clone()).transfer(transfer_amount)
    }

    /// shop contract call this method to let a user play one round
    /// predecessor should be shop contract and the amount goes to it
    /// signer should be a user
    /// owner and shop-owner get fee from amount
    pub fn insert_coin(&mut self, amount: U128) -> Promise {
        // todo:
        Promise::new(self.owner_id.clone())
    }

    /// game contract call this method to send reward to winner
    /// predecessor should be game contract and the reward comes from it
    /// owner and shop-owner get fee from reward
    pub fn reward_coin(&mut self, receiver_id: AccountId, amount: U128) -> Promise {
        // todo:
        Promise::new(self.owner_id.clone())
    }

    pub fn register_shop(&mut self, shop_id: AccountId, shop_owner_id: AccountId) {
        self.assert_owner();
        self.shops.insert(&shop_id, &shop_owner_id);
    }

    pub fn unregister_shop(&mut self, shop_id: AccountId) {
        self.assert_owner();
        self.shops.remove(&shop_id);
    }
}
