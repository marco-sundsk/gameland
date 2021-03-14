use crate::*;
use near_sdk::{ext_contract, Gas};

const GAS_FOR_BASIC: Gas = 5_000_000_000_000;
const NO_DEPOSIT: Balance = 0;

#[near_bindgen]
impl Contract {
    pub fn register_shop(&mut self, reg_form: HumanReadableRegForm) {
        // todo:

    }

    pub fn resovle_register(&mut self, shop_id: AccountId, op: u8) {
        // todo:

    }

    pub fn lock_shop(&mut self, shop_id: AccountId, op: u8) {
        // todo:

    }
}