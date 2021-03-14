use crate::*;

pub(crate) fn assert_one_yocto() {
    assert_eq!(
        env::attached_deposit(),
        1,
        "Requires attached deposit of exactly 1 yoctoNEAR"
    )
}

pub(crate) fn assert_self() {
    assert_eq!(
        env::predecessor_account_id(),
        env::current_account_id(),
        "Method is private"
    );
}

impl Contract {
    pub(crate) fn assert_owner(&self) {
        assert_eq!(
            env::predecessor_account_id(),
            self.owner_id,
            "Can only be called by the owner"
        );
    }

    pub(crate) fn internal_get_shop(&self, shop_id: &AccountId) -> RegForm {
        self.shops.get(shop_id).expect("This shop does NOT exist in running list.")
    }

    pub(crate) fn internal_get_register(&self, shop_id: &AccountId) -> RegForm {
        self.registers.get(shop_id).expect("This shop does NOT exist in reg/unreg list.")
    }

}
