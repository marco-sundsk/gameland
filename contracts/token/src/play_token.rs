use crate::*;
use near_sdk::{ext_contract, Gas};

const GAS_FOR_BASIC: Gas = 5_000_000_000_000;
const NO_DEPOSIT: Balance = 0;

#[ext_contract(ext_game)]
trait Game {
    fn gl_on_sponsor(&mut self, amount: U128) -> String;
    fn gl_on_play(&mut self, gross_amount: U128, net_amount: U128, op: String) -> String;
}

#[near_bindgen]
impl Contract {
    /// Mint token to the signer account.
    /// Features:
    /// * The signer account would be auto-register if needed.
    /// * Requires positive attached deposit.
    /// * Requires called by GameLand contract (the owner)
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
    /// * `amount` must be a positive integer.
    /// * The signer account should have at least the `amount` of tokens.
    /// * If remaining balance lower than ACCOUNT_KEEPALIVE_BALANCE, the account would be removed
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

    /// shop contract call this method to sponsor shop reward pool
    /// predecessor should be shop contract
    /// signer should be the sponsor
    pub fn sponsor_shop(&mut self, amount: U128) -> Promise {
        let shop_id = env::predecessor_account_id();
        let sponsor = env::signer_account_id();
        let amount: u128 = amount.into();
        assert!(self.is_shop(&shop_id), "Must be called by shop/game contract.");

        self.internal_transfer(&sponsor, &shop_id, amount, Some(String::from("sponsor")));

        ext_game::gl_on_sponsor(
            amount.into(),
            &shop_id,
            NO_DEPOSIT,
            env::prepaid_gas() - GAS_FOR_BASIC,
        )
    }

    /// shop contract call this method to let a user play one round
    /// predecessor should be shop contract and the amount goes to it
    /// signer should be a user
    /// owner and shop-owner get fee from amount
    pub fn insert_coin(&mut self, amount: U128, op: String) -> Promise {

        let caller = env::predecessor_account_id();
        let user = env::signer_account_id();

        assert!(!self.is_shop(&user), "Signer must be a user.");
        let shop_owner = self.shops.get(&caller).expect("Predecessor must be a shop.");
        
        let amount: u128 = amount.into();
        let shop_fee = self.game_ratio_for_play.multiply(amount);
        let owner_fee = self.owner_ratio_for_play.multiply(amount);
        let net_amount = amount - shop_fee - owner_fee;
        self.internal_transfer(&user, &self.owner_id.clone(), owner_fee, Some(String::from("owner_fee")));
        self.internal_transfer(&user, &shop_owner, shop_fee, Some(String::from("shop_fee")));
        self.internal_transfer(&user, &caller, net_amount, Some(String::from("insert_coin")));

        ext_game::gl_on_play(
            amount.into(),
            net_amount.into(),
            op,
            &caller,
            NO_DEPOSIT,
            env::prepaid_gas() - GAS_FOR_BASIC,
        )
    }

    /// game contract call this method to send reward to winner
    /// predecessor should be game contract and the reward comes from it
    /// owner and shop-owner get fee from reward
    pub fn reward_coin(&mut self, receiver_id: AccountId, amount: U128) {

        let caller = env::predecessor_account_id();
        let shop_owner = self.shops.get(&caller).expect("Predecessor must be a shop.");
        
        let amount: u128 = amount.into();

        let shop_fee = self.game_ratio_for_win.multiply(amount);
        let owner_fee = self.owner_ratio_for_win.multiply(amount);
        let net_amount = amount - shop_fee - owner_fee;
        self.internal_transfer(&receiver_id, &self.owner_id.clone(), owner_fee, Some(String::from("owner_tip")));
        self.internal_transfer(&receiver_id, &shop_owner, shop_fee, Some(String::from("shop_tip")));
        self.internal_transfer(&receiver_id, &caller, net_amount, Some(String::from("reward_coin")));
    }

    pub fn register_shop(&mut self, shop_id: AccountId, shop_owner_id: AccountId) {
        self.assert_owner();
        if !self.shops.contains_key(&shop_id) {
            self.shop_num += 1;
        }
        self.shops.insert(&shop_id, &shop_owner_id);
    }

    pub fn unregister_shop(&mut self, shop_id: AccountId) {
        self.assert_owner();
        if self.shops.contains_key(&shop_id) {
            self.shops.remove(&shop_id);
            self.shop_num -= 1;
        }
        
    }
}
