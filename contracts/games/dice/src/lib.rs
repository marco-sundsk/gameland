/*
 * This is GameLand_Dice contract:
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
use near_sdk::collections::{Vector, LookupMap};
use uint::construct_uint;

pub use crate::gl_metadata::*;
pub use crate::gl_core::*;
use crate::internal::*;

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
pub struct WinnerInfo {
    pub user: AccountId,  // winner
    pub amount: Balance, // win prize
    pub height: BlockHeight,
    pub ts: u64,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct HumanReadableWinnerInfo {
    pub user: AccountId,
    pub amount: U128,
    pub height: U64,
    pub ts: U64,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct HumanReadableContractInfo {
    pub owner: AccountId,
    pub jack_pod: U128,
    pub dice_number: u8,
    pub rolling_fee: U128,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct HumanReadableDiceResult {
    pub user: AccountId,
    pub user_guess: u8,
    pub dice_point: u8,
    pub reward_amount: U128,
    pub jackpod_left: U128,
    pub height: U64,
    pub ts: U64,
}

// Structs in Rust are similar to other languages, and may include impl keyword as shown below
// Note: the names of the structs are not important when calling the smart contract, but the function names are
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    pub owner_id: AccountId,
    pub dice_number: u8,
    pub rolling_fee: Balance,  // how many token needed to roll once.
    pub jack_pod: Balance,  // half of them would be show to user as jack_pod amount
    pub win_history: Vector<WinnerInfo>,
}

impl Default for Contract {
    fn default() -> Self {
        env::panic(b"dice contract should be initialized before usage")
    }
}

#[near_bindgen]
impl Contract {

    #[init]
    pub fn new(
        owner_id: AccountId,
        dice_number: u8,
        rolling_fee: U128,
    ) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        assert!(
            env::is_valid_account_id(owner_id.as_bytes()),
            "The owner account ID is invalid"
        );
        
        Self {
            owner_id,
            dice_number,
            rolling_fee: rolling_fee.into(),
            jack_pod: 0_u128,
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
    
    pub fn update_dice_number(&mut self, dice_number: u8) {
        self.assert_owner();
        self.dice_number = dice_number;
    }

    pub fn update_rolling_fee(&mut self, rolling_fee: U128) {
        self.assert_owner();
        self.rolling_fee = rolling_fee.into();
    }

    //***********************/
    // view functions
    //***********************/

    fn get_hr_info(&self, index: u64) -> HumanReadableWinnerInfo {
        let info = self.win_history.get(index).expect("Error: no this item in winner history!");
        HumanReadableWinnerInfo {
            user: info.user.clone(),
            amount: info.amount.into(),
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
            jack_pod: self.jack_pod.into(),
            dice_number: self.dice_number,
            rolling_fee: self.rolling_fee.into(),
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
