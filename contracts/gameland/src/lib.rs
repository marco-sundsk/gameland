/*
 * This is GameLand contract:
 * 
 * 
 */

// To conserve gas, efficient serialization is achieved through Borsh (http://borsh.io/)
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::wee_alloc;
use near_sdk::json_types::{U64, U128, ValidAccountId};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, near_bindgen, AccountId, Balance, BlockHeight, Promise};
use near_sdk::collections::{UnorderedMap, LookupMap};
use uint::construct_uint;

pub use crate::gameland::*;
pub use crate::gov::*;
use crate::internal::*;

mod gameland;
mod gov;
mod internal;


construct_uint! {
    /// 256-bit unsigned integer.
    pub struct U256(4);
}

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Metadata {
    version: String,
    name: String,
    logo_url: String,
    thumb_url: String,
    description: String,
}

#[derive(BorshDeserialize, BorshSerialize, Clone)]
pub struct RegForm {
    pub shop_id: AccountId,
    pub owner_id: AccountId,
    pub flag: u8,  // 1 - reg; 2 - unreg
    pub refs: String, // url to the application form off-line
    pub height: BlockHeight,  // apply height
    pub ts: u64,  // apply time
    pub status: u8, // 0 - applying; 1 - evaluating; 2 - OK; 9 - Rejected;
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct HumanReadableRegForm {
    pub shop_id: AccountId,
    pub owner_id: AccountId,
    pub flag: u8, 
    pub refs: String,
    pub height: U64,
    pub ts: U64, 
    pub status: u8,
}

// Structs in Rust are similar to other languages, and may include impl keyword as shown below
// Note: the names of the structs are not important when calling the smart contract, but the function names are
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    pub owner_id: AccountId,
    pub shops: UnorderedMap<AccountId, RegForm>,
    pub registers: UnorderedMap<AccountId, RegForm>,
}


impl Default for Contract {
    fn default() -> Self {
        env::panic(b"GameLand contract should be initialized before usage")
    }
}

#[near_bindgen]
impl Contract {

    #[init]
    pub fn new(
        owner_id: AccountId,
    ) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        assert!(
            env::is_valid_account_id(owner_id.as_bytes()),
            "The owner account ID is invalid"
        );
        
        Self {
            owner_id,
            shops: UnorderedMap::new(b"s".to_vec()),
            registers: UnorderedMap::new(b"r".to_vec()),
        }
    }


    //***********************/
    // view functions
    //***********************/
    pub fn metadata() -> Metadata {
        Metadata {
            version: String::from("0.1.1"),
            name: String::from("Game Zone"),
            logo_url: String::from("gameland.testnet.logo.png"),
            thumb_url: String::from("gameland.testnet.thumb.png"),
            description: String::from("A platform for mini-games"),
        }
    }

    pub fn get_shop(&self, shop_id: AccountId) -> HumanReadableRegForm {
        let form = self.shops.get(&shop_id).expect("Error: no this shop!");
        HumanReadableRegForm {
            shop_id: form.shop_id,
            owner_id: form.owner_id,
            flag: form.flag,
            refs: form.refs,
            height: form.height.into(),
            ts: form.ts.into(),
            status: form.status,
        }
    }

    pub fn list_shops(&self, from_index: u64, limit: u64) -> Vec<HumanReadableRegForm> {
        let keys = self.shops.keys_as_vector();
        (from_index..std::cmp::min(from_index + limit, keys.len()))
            .map(|index| self.get_shop(keys.get(index).unwrap()))
            .collect()
    }

    pub fn get_register(&self, shop_id: AccountId) -> HumanReadableRegForm {
        let form = self.registers.get(&shop_id).expect("Error: no this shop!");
        HumanReadableRegForm {
            shop_id: form.shop_id,
            owner_id: form.owner_id,
            flag: form.flag,
            refs: form.refs,
            height: form.height.into(),
            ts: form.ts.into(),
            status: form.status,
        }
    }

    pub fn list_registers(&self, from_index: u64, limit: u64) -> Vec<HumanReadableRegForm> {
        let keys = self.registers.keys_as_vector();
        (from_index..std::cmp::min(from_index + limit, keys.len()))
            .map(|index| self.get_register(keys.get(index).unwrap()))
            .collect()
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
