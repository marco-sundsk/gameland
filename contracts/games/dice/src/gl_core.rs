use crate::*;
use near_sdk::json_types::ValidAccountId;
use near_sdk::{ext_contract, Gas, PromiseResult};

const GAS_FOR_RESOLVE_TRANSFER: Gas = 5_000_000_000_000;
const GAS_FOR_FT_TRANSFER_CALL: Gas = 25_000_000_000_000 + GAS_FOR_RESOLVE_TRANSFER;

const NO_DEPOSIT: Balance = 0;

pub trait GameLandCore {

    /// called by play-token contract
    fn gl_on_play(&mut self, gross_amount: U128, net_amount: U128, memo: Option<String>) -> String;

    /// called by play-token contract
    fn gl_on_settle(&mut self, gross_amount: U128, net_amount: U128, memo: Option<String>) -> String;
}

#[ext_contract(ext_play_token)]
trait PlayToken {

    ///
    fn gl_reward_coin(&mut self, receiver_id: AccountId, amount: U128);
}

#[near_bindgen]
impl GameLandCore for Contract {

    fn gl_on_play(&mut self, gross_amount: U128, net_amount: U128, memo: Option<String>) -> String {
        // assert_one_yocto();
        // let sender_id = env::predecessor_account_id();
        // assert!(!self.is_shop(&sender_id), "Sender can NOT be a shop.");
        // assert!(!self.is_shop(receiver_id.as_ref()), "Receiver can NOT be a shop.");
        // let amount = amount.into();
        // self.internal_transfer(&sender_id, receiver_id.as_ref(), amount, memo);
        String::from("TODO: Under construction")
    }

    fn gl_on_settle(&mut self, gross_amount: U128, net_amount: U128, memo: Option<String>) -> String {
        String::from("Not applicable in this game.")
    }

}