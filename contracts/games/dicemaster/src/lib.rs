/*
 * This is DiceMaster contract:
 * 
 * 
 *
 */

// To conserve gas, efficient serialization is achieved through Borsh (http://borsh.io/)
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::wee_alloc;
use near_sdk::json_types::{U64, U128, ValidAccountId};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, near_bindgen, AccountId, Balance, BlockHeight, Promise};
use near_sdk::collections::{Vector};
use uint::construct_uint;

pub use crate::gl_metadata::*;
pub use crate::gl_core::*;
// use crate::internal::*;

mod gl_metadata;
mod gl_core;
mod internal;


construct_uint! {
    /// 256-bit unsigned integer.
    pub struct U256(4);
}

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct BetInfo {
    // 1 - big/small, 2 - even/odd, 3 - weitou, 4 - quantou, 5 - composition, 6 - double-dice
    pub category: u8,
    pub guess1: u8,  // 1: odd or big, 2: even or small, N for other categories
    pub guess2: u8,  // used in category 5: composition
    pub guess3: u8,  // reserve for future use
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct WinnerInfo {
    pub user: AccountId,  // winner
    pub category: u8,  // game category: 
    pub odds: u8,  // bet * odds = reward
    pub reward: Balance, 
    pub height: BlockHeight,
    pub ts: u64,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct HumanReadableWinnerInfo {
    pub user: AccountId,
    pub category: u8,
    pub odds: u8,
    pub reward: U128,
    pub height: U64,
    pub ts: U64,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct HumanReadableContractInfo {
    pub owner: AccountId,
    pub jackpot: U128,
    pub min_bet: U128,
    pub max_bet: U128,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct HumanReadableDiceResult {
    pub user: AccountId,
    pub dice_point: Vec<u8>,
    pub reward_amount: U128,
    pub jackpot_left: U128,
    pub height: U64,
    pub ts: U64,
    pub ret_code: u8,
    pub reason: String,
}

// Structs in Rust are similar to other languages, and may include impl keyword as shown below
// Note: the names of the structs are not important when calling the smart contract, but the function names are
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    pub gl_play_count: u128,
    pub gl_winner_count: u128,
    pub gl_reward_sum: Balance,

    pub owner_id: AccountId,
    pub min_bet: Balance,  // the minimum token needed to play once.
    pub max_bet: Balance,
    pub jackpot: Balance,  
    pub win_history: Vector<WinnerInfo>,
}

impl Default for Contract {
    fn default() -> Self {
        env::panic(b"This contract should be initialized before usage")
    }
}

#[near_bindgen]
impl Contract {

    #[init]
    pub fn new(owner_id: AccountId, min_bet: U128, max_bet: U128) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        assert!(
            env::is_valid_account_id(owner_id.as_bytes()),
            "The owner account ID is invalid"
        );
        
        Self {
            gl_play_count: 0,
            gl_winner_count: 0,
            gl_reward_sum: 0,
            owner_id,
            min_bet: min_bet.into(),
            max_bet: max_bet.into(),
            jackpot: 0_u128,
            win_history: Vector::new(b"w".to_vec()),
        }
    }

    //***********************/
    // owner functions
    //***********************/

    fn assert_owner(&self) {
        assert_eq!(
            env::predecessor_account_id(),
            self.owner_id,
            "Can only be called by the owner"
        );
    }
    
    pub fn update_min_bet(&mut self, min_bet: U128) {
        self.assert_owner();
        self.min_bet = min_bet.into();
    }

    //***********************/
    // view functions
    //***********************/

    fn get_hr_info(&self, index: u64) -> HumanReadableWinnerInfo {
        let info = self.win_history.get(index).expect("Error: no this item in winner history!");
        HumanReadableWinnerInfo {
            user: info.user.clone(),
            category: info.category,
            odds: info.odds,
            reward: info.reward.into(),
            height: info.height.into(),
            ts: info.ts.into(),
        }
    }

    /// Returns the list of winner info in LIFO order
    pub fn get_win_history(&self, from_index: u64, limit: u64) -> Vec<HumanReadableWinnerInfo> {
        let counts: u64 = self.win_history.len() as u64;
        (from_index..std::cmp::min(from_index + limit, counts))
            .map(|index| self.get_hr_info(counts - index - 1))  // reverse to get LIFO order
            .collect()
    }

    pub fn get_contract_info(&self) -> HumanReadableContractInfo {
        HumanReadableContractInfo {
            owner: self.owner_id.clone(),
            jackpot: self.jackpot.into(),
            min_bet: self.min_bet.into(),
            max_bet: self.max_bet.into(),
        }
    }

}

/*
 * The rest of this file holds the inline tests for the code above
 * Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
 *
 * To run from contract directory:
 * cargo test -- --nocapture
 *
 * From project root, to run in combination with frontend tests:
 * yarn test
 *
 */
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};

    // mock the context for testing, notice "signer_account_id" that was accessed above from env::
    fn get_context(input: Vec<u8>, is_view: bool) -> VMContext {
        VMContext {
            current_account_id: "alice_near".to_string(),
            signer_account_id: "bob_near".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: "carol_near".to_string(),
            input,
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view,
            output_data_receivers: vec![],
            epoch_height: 19,
        }
    }

    #[test]
    fn set_then_get_greeting() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = Contract::default();
    }

    #[test]
    fn get_default_greeting() {
        let context = get_context(vec![], true);
        testing_env!(context);
        let contract = Contract::default();
        // this test did not call set_greeting so should return the default "Hello" greeting
    }
}
