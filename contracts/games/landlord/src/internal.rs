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
    pub(crate) fn internal_play(&mut self, account_id: &AccountId, gross_amount: u128, net_amount: u128, user_guess: u8) -> HumanReadablePlayResult {
        self.jack_pod += net_amount;
        self.houses.insert(&user_guess, account_id);
        
        // rolling dice here
        let random_u8: u8 = env::random_seed().iter().fold(0_u8, |acc, x| acc.wrapping_add(*x));
        let god_choose = self.house_count as u16 * random_u8 as u16 / 0x100_u16 + 1;

        let mut result = HumanReadablePlayResult {
            user: account_id.clone(),
            user_guess,
            dice_point: dice_point as u8,
            reward_amount: 0.into(),  // if win, need update
            jackpod_left: self.jack_pod.into(),  // if win, need update
            height: env::block_index().into(),
            ts: env::block_timestamp().into(),
        };
        
        // let's see how lucky caller is this time
        if user_guess == dice_point as u8 {  // Wow, he wins
            // figure out gross reward and update jack pod
            let gross_reward = self.jack_pod / 2;
            self.jack_pod -= gross_reward;
            result.jackpod_left = self.jack_pod.into();
            result.reward_amount = gross_reward.into();
            // records this winning
            self.win_history.push(&WinnerInfo {
                user: account_id.clone(),
                amount: gross_reward,
                height: env::block_index(),
                ts: env::block_timestamp(),
            });
        }
        
        result
    }

}
