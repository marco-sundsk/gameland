use crate::*;
use near_sdk::{ext_contract, Gas};

const TOKEN_CONTRACT: &str = "playtoken.testnet";
const GAS_FOR_BASIC: Gas = 5_000_000_000_000;
const NO_DEPOSIT: Balance = 0;


#[ext_contract(ext_game)]
trait Game {
    fn gl_sponser(&mut self, amount: U128) -> Promise;
    fn gl_play(&mut self, amount: U128, op: String) -> Promise;
    fn gl_settle(&mut self, op: String) -> String;
}

#[ext_contract(ext_play_token)]
trait Token {
    fn mint_playtoken(&mut self);
    fn burn_playtoken(&mut self, amount: U128) -> Promise;
}


#[near_bindgen]
impl Contract {

    pub fn sponser(&mut self, shop_id: AccountId, amount: U128) -> Promise {
        let shop_form = self.internal_get_shop(&shop_id);
        ext_game::gl_sponser(
            amount,
            &shop_form.shop_id,
            NO_DEPOSIT,
            env::prepaid_gas() - GAS_FOR_BASIC,
        )
    }

    pub fn play(&mut self, shop_id: AccountId, amount: U128, op: String) -> Promise {
        let shop_form = self.internal_get_shop(&shop_id);
        ext_game::gl_play(
            amount,
            op,
            &shop_form.shop_id,
            NO_DEPOSIT,
            env::prepaid_gas() - GAS_FOR_BASIC,
        )
    }

    pub fn settle(&mut self, shop_id: AccountId, op: String) -> Promise {
        let shop_form = self.internal_get_shop(&shop_id);
        ext_game::gl_settle(
            op,
            &shop_form.shop_id,
            NO_DEPOSIT,
            env::prepaid_gas() - GAS_FOR_BASIC,
        )
    }

    #[payable]
    pub fn buy_playtoken(&mut self) -> Promise {
        let amount = env::attached_deposit();
        ext_play_token::mint_playtoken(
            &String::from(TOKEN_CONTRACT),
            amount,
            env::prepaid_gas() - GAS_FOR_BASIC,
        )
    }

    pub fn sell_playtoken(&mut self, amount: U128) -> Promise {
        ext_play_token::burn_playtoken(
            amount,
            &String::from(TOKEN_CONTRACT),
            NO_DEPOSIT,
            env::prepaid_gas() - GAS_FOR_BASIC,
        )
    }

}