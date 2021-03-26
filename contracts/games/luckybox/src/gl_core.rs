use crate::*;
use near_sdk::{ext_contract, Gas, PromiseOrValue};

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
    fn gl_play(&mut self, amount: U128, op: String) -> PromiseOrValue<String>;

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
    fn reward_coin_multiple(&mut self, receivers: HashMap<AccountId, U128>);
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
        self.total_jackpot += amount;
        format!("{} sponsored {}, jackpod increase to {}, ", sponsor, amount, self.total_jackpot)
    }

    fn gl_play(&mut self, amount: U128, op: String) -> PromiseOrValue<String> {
        env::log(format!("game::gl_play from {}, prapaid_gas {} ", 
            env::predecessor_account_id(), env::prepaid_gas()).as_bytes());

        let amount: u128 = amount.into();
        if amount < self.play_fee {
            return PromiseOrValue::Value(String::from("Insufficient coin inserted."));
        }

        // see if user choose valid box
        let guess: u8 = op.parse::<u8>().unwrap_or(0);
        if guess == 0 || guess > self.box_count {
            return PromiseOrValue::Value(String::from("Invalid box id."));
        } 

        // see if this round ended already
        let (round_end, winners) = self.internal_settle(); 
        if round_end {
            // Distribute reward to each one in winners,
            // should use batch_rewards interface of token contract
            let receivers: HashMap<AccountId, U128> = winners.keys().map(
                |key| (key.clone(), winners.get(key).unwrap().clone().into())
            ).collect::<HashMap<_,_>>();
            ext_play_token::reward_coin_multiple(
                receivers,
                &String::from(TOKEN_CONTRACT),
                NO_DEPOSIT,
                env::prepaid_gas() - GAS_FOR_BASIC,
            );

            return PromiseOrValue::Value(String::from("Round end."));
 
        }

        ext_play_token::insert_coin(
            amount.into(),
            op,
            &String::from(TOKEN_CONTRACT),
            NO_DEPOSIT,
            env::prepaid_gas() - GAS_FOR_BASIC,
        ).into()
    }

    fn gl_on_play(&mut self, gross_amount: U128, net_amount: U128, op: String) -> String {
        env::log(format!("game::gl_on_play from {}, prapaid_gas {} ", 
            env::predecessor_account_id(), env::prepaid_gas()).as_bytes());
        
        let player = env::signer_account_id();
        let box_id = op.parse::<u8>().unwrap_or(0);
        let gross_amount: u128 = gross_amount.into();
        let net_amount: u128 = net_amount.into();
        
        let result = self.internal_play(&player, gross_amount, net_amount, box_id);

        near_sdk::serde_json::to_string(&result).unwrap()

    }

    fn gl_settle(&mut self, op: String) -> String {
        let (round_end, winners) = self.internal_settle(); 
        if round_end {
            // Distribute reward to each one in winners,
            // should use batch_rewards interface of token contract
            let receivers: HashMap<AccountId, U128> = winners.keys().map(
                |key| (key.clone(), winners.get(key).unwrap().clone().into())
            ).collect::<HashMap<_,_>>();
            ext_play_token::reward_coin_multiple(
                receivers,
                &String::from(TOKEN_CONTRACT),
                NO_DEPOSIT,
                env::prepaid_gas() - GAS_FOR_BASIC,
            );
            format!("round end. op is {}", op)
        } else {
            format!("Still in round. op is {}", op)
        }
        
    }

}