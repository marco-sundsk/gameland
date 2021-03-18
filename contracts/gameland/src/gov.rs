use crate::*;
use near_sdk::{ext_contract, Gas};

const TOKEN_CONTRACT: &str = "playtoken.testnet";
const GAS_FOR_BASIC: Gas = 10_000_000_000_000;
const NO_DEPOSIT: Balance = 0;

#[ext_contract(ext_play_token)]
trait Token {
    fn register_shop(&mut self, shop_id: AccountId, shop_owner_id: AccountId);
    fn unregister_shop(&mut self, shop_id: AccountId);
}


#[near_bindgen]
impl Contract {
    pub fn register_shop(&mut self, reg_form: HumanReadableRegForm) {
        let form = RegForm {
            flag: reg_form.flag,
            shop_id: reg_form.shop_id,
            owner_id: reg_form.owner_id,
            refs: reg_form.refs,
            height: env::block_index(),
            ts: env::block_timestamp(),
            status: reg_form.status,
        };
        self.registers.insert(&form.shop_id, &form);
    }

    pub fn resovle_register(&mut self, shop_id: AccountId, pass: bool, new_status: u8) {
        let mut form = self.internal_get_register(&shop_id);
        form.status = new_status;
        self.registers.insert(&form.shop_id, &form);
        if pass && form.flag == 1 {
            // reg passed
            self.registers.remove(&shop_id);
            self.shops.insert(&shop_id, &form);
            ext_play_token::register_shop(
                form.shop_id.clone(),
                form.owner_id,
                &String::from(TOKEN_CONTRACT),
                NO_DEPOSIT,
                env::prepaid_gas() - GAS_FOR_BASIC,
            );
        } 
        if pass && form.flag == 2 {
            self.shops.remove(&shop_id);
            ext_play_token::unregister_shop(
                form.shop_id,
                &String::from(TOKEN_CONTRACT),
                NO_DEPOSIT,
                env::prepaid_gas() - GAS_FOR_BASIC,
            );
        }
    }

    pub fn lock_shop(&mut self, shop_id: AccountId, op: u8) {
        // todo:

    }
}