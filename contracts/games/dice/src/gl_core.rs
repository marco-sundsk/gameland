use crate::*;
use near_sdk::json_types::ValidAccountId;
use near_sdk::{ext_contract, Gas, PromiseResult};

const TOKEN_CONTRACT: &str = "playtoken.testnet";

const GAS_FOR_BASIC: Gas = 10_000_000_000_000;


const NO_DEPOSIT: Balance = 0;

pub trait GameLandCore {

    /// signer is the sponsor,
    /// predecessor could be gameland or sponsor himself,
    fn gl_sponsor(&mut self, amount: U128) -> Promise;

    /// token contract invoke this method, 
    /// to indicate that the amount of token 
    /// has successfully deposit to the shop.
    fn gl_on_sponsor(&mut self, amount: U128) -> String;

    /// signer is the player,
    /// predecessor could be gameland or player himself,
    fn gl_play(&mut self, amount: U128, op: String) -> Promise;

    /// token contract invoke this method, 
    /// to indicate that the net_amount of token 
    /// has successfully deposit to the shop.
    fn gl_on_play(&mut self, gross_amount: U128, net_amount: U128, op: String) -> String;

    /// signer is the trigger,
    /// predecessor could be gameland or trigger himself,
    /// return a json-str as result
    fn gl_settle(&mut self, op: String) -> String;
}

#[ext_contract(ext_play_token)]
trait PlayToken {
    fn sponsor_shop(&mut self, amount: U128) -> Promise;
    fn insert_coin(&mut self, amount: U128, op: String) -> Promise;
    /// call this to distribute reward to receiver
    fn reward_coin(&mut self, receiver_id: AccountId, amount: U128);
}

#[near_bindgen]
impl GameLandCore for Contract {

    fn gl_sponsor(&mut self, amount: U128) -> Promise {
        env::log(format!("game::gl_sponsor from {}, prapaid_gas {} ", 
            env::predecessor_account_id(), env::prepaid_gas()).as_bytes());
        ext_play_token::sponsor_shop(
            amount,
            &String::from(TOKEN_CONTRACT),
            NO_DEPOSIT,
            env::prepaid_gas() - GAS_FOR_BASIC,
        )
    }

    fn gl_on_sponsor(&mut self, amount: U128) -> String {
        env::log(format!("game::gl_on_sponsor from {}, prapaid_gas {} ", 
            env::predecessor_account_id(), env::prepaid_gas()).as_bytes());
        let amount: u128 = amount.into();
        let sponsor = env::signer_account_id();
        self.jack_pod += amount;
        format!("{} sponsored {}, jackpod increase to {}, ", sponsor, amount, self.jack_pod)
    }

    fn gl_play(&mut self, amount: U128, op: String) -> Promise {
        env::log(format!("game::gl_play from {}, prapaid_gas {} ", 
            env::predecessor_account_id(), env::prepaid_gas()).as_bytes());
        ext_play_token::insert_coin(
            amount,
            op,
            &String::from(TOKEN_CONTRACT),
            NO_DEPOSIT,
            env::prepaid_gas() - GAS_FOR_BASIC,
        )
    }

    fn gl_on_play(&mut self, gross_amount: U128, net_amount: U128, op: String) -> String {
        env::log(format!("game::gl_on_play from {}, prapaid_gas {} ", 
            env::predecessor_account_id(), env::prepaid_gas()).as_bytes());

        let player = env::signer_account_id();

        let guess = op.parse::<u8>().unwrap_or(0);
        let result = self.internal_play(&player, gross_amount.into(), net_amount.into(), guess);
        let reward: u128 = result.reward_amount.into();
        if reward > 0 {
            ext_play_token::reward_coin(
                player.clone(),
                result.reward_amount,
                &String::from(TOKEN_CONTRACT),
                NO_DEPOSIT,
                GAS_FOR_BASIC,
            );
        }
        near_sdk::serde_json::to_string(&result).unwrap()

    }

    fn gl_settle(&mut self, op: String) -> String {
        String::from("Not applicable in this game.")
    }

}