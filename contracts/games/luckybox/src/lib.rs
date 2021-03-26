/*
 * This is GameLand_Luckybox contract:
 * 
 * 
 *
 */
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::wee_alloc;
use near_sdk::json_types::{U64, U128, ValidAccountId};
use near_sdk::serde::{Serialize};
use near_sdk::{env, near_bindgen, AccountId, Balance, BlockHeight, Promise};
use near_sdk::collections::{Vector, UnorderedMap};
use std::collections::HashMap;
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

#[derive(BorshDeserialize, BorshSerialize)]
pub struct BoxInfo {
    pub box_id: u8,
    pub investors: HashMap<AccountId, Balance>,
    pub total_amount: Balance,
    // start height and time of this round
    pub height: BlockHeight,
    pub ts: u64,
}

#[derive(Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct HumanReadableBoxInfo {
    pub box_id: u8,
    pub investors: HashMap<String, U128>,
    pub total_amount: U128,
    pub height: U64,
    pub ts: U64,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct WinnerInfo {
    pub round: u128,
    pub total_reward: Balance, 
    pub winners: HashMap<AccountId, Balance>,  // winner->reward
    // settle height and time of this round
    pub height: BlockHeight,
    pub ts: u64,
}

#[derive(Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct HumanReadableWinnerInfo {
    pub round: U128,
    pub total_reward: U128,
    pub winners: HashMap<String, U128>,
    pub height: U64,
    pub ts: U64,
}

#[derive(Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct HumanReadableContractInfo {
    pub owner: AccountId,
    pub total_jackpot: U128,
    pub box_count: u8,
    pub play_fee: U128,
    pub current_round: U128,
    pub round_start_height: U64,
    pub round_start_ts: U64,
    pub round_period: u32,
    pub cur_win_box: u8,
}

#[derive(Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct HumanReadablePlayResult {
    pub user: AccountId,
    pub round: U128,
    pub box_id: u8,
    pub box_amount: U128,
    pub total_jackpot: U128,
    pub height: U64,
    pub ts: U64,
}

// Structs in Rust are similar to other languages, and may include impl keyword as shown below
// Note: the names of the structs are not important when calling the smart contract, but the function names are
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    // params
    pub owner_id: AccountId,
    pub box_count: u8,
    pub per_round_last: u32,
    pub play_fee: Balance,  // the minimum token needed to play once.
    // status
    pub total_jackpot: Balance,  // totoal jackpot
    pub boxes: UnorderedMap<u8, BoxInfo>,
    pub win_history: Vector<WinnerInfo>,
    pub current_round: u128,
    pub current_min_box_id: u8,
    // the real endtime is start_height + per_round_last + random
    pub current_round_start_height: BlockHeight,
    pub current_round_start_ts: u64,
    
}

impl Default for Contract {
    fn default() -> Self {
        env::panic(b"this contract should be initialized before using")
    }
}

#[near_bindgen]
impl Contract {

    #[init]
    pub fn new(
        owner_id: AccountId,
        box_count: u8,
        per_round_last: u32,
        play_fee: U128,
    ) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        assert!(
            env::is_valid_account_id(owner_id.as_bytes()),
            "The owner account ID is invalid"
        );
        
        Self {
            owner_id,
            box_count,
            per_round_last,
            play_fee: play_fee.into(),
            total_jackpot: 0_u128,
            boxes: UnorderedMap::new(b"b".to_vec()),
            win_history: Vector::new(b"w".to_vec()),
            current_round: 0,
            current_min_box_id: 0,
            current_round_start_height: 0,
            current_round_start_ts: 0,
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
    
    pub fn update_box_count(&mut self, box_count: u8) {
        self.assert_owner();
        if self.current_round_start_height == 0 {
            self.box_count = box_count;
        } else {
            env::panic(b"box count can only be updated between rounds.")
        }
    }

    pub fn update_play_fee(&mut self, play_fee: U128) {
        self.assert_owner();
        self.play_fee = play_fee.into();
    }

    //***********************/
    // view functions
    //***********************/

    fn get_box_info(&self, box_id: u8) -> HumanReadableBoxInfo {
        if let Some(the_box) = self.boxes.get(&box_id) {
            HumanReadableBoxInfo {
                box_id: the_box.box_id,
                investors: {
                    the_box.investors.keys().map(
                        |key| ( 
                            key.clone(), 
                            the_box.investors.get(key).unwrap().clone().into()
                        )
                    ).collect::<HashMap<_,_>>()
                },
                total_amount: the_box.total_amount.into(),
                height: the_box.height.into(),
                ts: the_box.ts.into(),
            }
        } else {
            HumanReadableBoxInfo {
                box_id,
                investors: HashMap::new(),
                total_amount: 0.into(),
                height: 0.into(),
                ts: 0.into(),
            }
        }
    } 
    /// get all occupied houses info
    pub fn get_boxes_info(&self) -> HashMap<u8, HumanReadableBoxInfo> {
        let keys = self.boxes.keys_as_vector();

        (0..keys.len()).map(
            |index| (
                keys.get(index).unwrap(), 
                self.get_box_info(keys.get(index).unwrap()))
        ).collect::<HashMap<_,_>>()
    }

    fn get_hrw_info(&self, index: u64) -> HumanReadableWinnerInfo {
        let info = self.win_history.get(index).expect("Error: no this item in winner history!");
        HumanReadableWinnerInfo {
            round: info.round.into(),
            total_reward: info.total_reward.into(),
            winners: {
                info.winners.keys().map(
                        |key| ( 
                            key.clone(), 
                            info.winners.get(key).unwrap().clone().into()
                        )
                    ).collect::<HashMap<_,_>>()
            },
            height: info.height.into(),
            ts: info.ts.into(),
        }
    }

    /// Returns the list of winner info in LIFO order
    pub fn get_win_history(&self, from_index: u64, limit: u64) -> Vec<HumanReadableWinnerInfo> {
        let counts: u64 = self.win_history.len() as u64;
        (from_index..std::cmp::min(from_index + limit, counts))
            .map(|index| self.get_hrw_info(counts - index - 1))  // reverse to get LIFO order
            .collect()
    }

    pub fn get_contract_info(&self) -> HumanReadableContractInfo {
        HumanReadableContractInfo {
            owner: self.owner_id.clone(),
            total_jackpot: self.total_jackpot.into(),
            box_count: self.box_count,
            play_fee: self.play_fee.into(),
            current_round: self.current_round.into(),
            round_start_height: self.current_round_start_height.into(),
            round_start_ts: self.current_round_start_ts.into(),
            round_period: self.per_round_last,
            cur_win_box: self.current_min_box_id,
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
