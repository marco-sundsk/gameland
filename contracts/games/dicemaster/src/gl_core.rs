use crate::*;

use near_sdk::{ext_contract, Gas, PromiseOrValue};

const TOKEN_CONTRACT: &str = "playtoken.testnet";

const GAS_FOR_BASIC: Gas = 20_000_000_000_000;


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
        self.jackpot += amount;
        format!("{} sponsored {}, jackpot increase to {}, ", sponsor, amount, self.jackpot)
    }

    fn gl_play(&mut self, amount: U128, op: String) -> PromiseOrValue<String> {
        env::log(format!("game::gl_play from {}, prapaid_gas {} ", 
            env::predecessor_account_id(), env::prepaid_gas()).as_bytes());
        self.gl_play_count += 1;
        // see if bet amount is valid
        let mut result = HumanReadableDiceResult {
            user: env::signer_account_id(),
            dice_point: vec![0,0,0],
            bet_amount: amount,
            reward_amount: 0.into(),  
            jackpot_left: self.jackpot.into(),
            height: env::block_index().into(),
            ts: env::block_timestamp().into(),
            ret_code: 1,
            reason: String::from(""),
        };
        let amount: u128 = amount.into();
        if amount < self.min_bet || amount > self.max_bet {
            result.reason = "Invalid bet amount".to_string();
            return PromiseOrValue::Value(near_sdk::serde_json::to_string(&result).unwrap());
        }
        // see if category is valid
        let bet_info: BetInfo = near_sdk::serde_json::from_str(&op).unwrap();
        if bet_info.category < 1 || bet_info.category > 6 {
            result.reason = "Invailid category".to_string();
            return PromiseOrValue::Value(near_sdk::serde_json::to_string(&result).unwrap());
        }
        // see if jackpot can support the bet amount
        if self.jackpot / 5 < self.internal_max_reward(amount, &bet_info) {
            result.reason = "Overflow on bet amount".to_string();
            return PromiseOrValue::Value(near_sdk::serde_json::to_string(&result).unwrap());
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
        
        let gross_amount: u128 = gross_amount.into();
        let net_amount: u128 = net_amount.into();
        let player = env::signer_account_id();

        self.jackpot += net_amount;

        // recheck that jackpot can support max reward
        let bet_info: BetInfo = near_sdk::serde_json::from_str(&op).unwrap();
        if self.jackpot / 5 < self.internal_max_reward(gross_amount, &bet_info) {
            env::log(format!("Overflow on bet amount, refund bet amount.").as_bytes());
            ext_play_token::reward_coin(
                player.clone(),
                gross_amount.into(),
                &String::from(TOKEN_CONTRACT),
                NO_DEPOSIT,
                GAS_FOR_BASIC,
            );
            // this is small probability event, and not player's fault,
            // so return all bet amount including play commission fee
            self.jackpot -= gross_amount;
            let result = HumanReadableDiceResult {
                user: player.clone(),
                dice_point: vec![0,0,0],
                bet_amount: gross_amount.into(),
                reward_amount: 0.into(),  
                jackpot_left: self.jackpot.into(),
                height: env::block_index().into(),
                ts: env::block_timestamp().into(),
                ret_code: 1,
                reason: String::from("Overflow on bet amount, refund bet amount"),
            };
            return near_sdk::serde_json::to_string(&result).unwrap();
        }

        let result = self.internal_play(&player, net_amount, &bet_info);
        let reward: u128 = result.reward_amount.into();
        if reward > 0 {
            self.gl_winner_count += 1;
            self.gl_reward_sum += reward;
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
        format!("Not applicable in this game, op is {}", op)
    }

}