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
        self.houses.insert(&user_guess, &env::signer_account_id());
        
        // god choose here
        let random_u8: u8 = env::random_seed().iter().fold(0_u8, |acc, x| acc.wrapping_add(*x));
        let god_choosen = self.house_count as u16 * random_u8 as u16 / 0x100_u16 + 1;
        let god_choosen: u8 = god_choosen as u8;
        let lucky_guy = self.houses.get(&god_choosen).unwrap_or(String::from(""));

        let mut result = HumanReadablePlayResult {
            user: account_id.clone(),
            round: self.current_round.into(),
            user_choosen: user_guess,
            god_choosen: god_choosen as u8,
            lucky_guy: lucky_guy.clone(), 
            reward_amount: 0.into(),  // if win, need update
            jackpod_left: self.jack_pod.into(),  // if win, need update
            height: env::block_index().into(),
            ts: env::block_timestamp().into(),
        };
        
        // let's see how lucky caller is this time
        if lucky_guy != "" {  // Wow, he wins
            // figure out gross reward and update jack pod
            let gross_reward = self.jack_pod / 2;
            self.jack_pod -= gross_reward;
            result.jackpod_left = self.jack_pod.into();
            result.reward_amount = gross_reward.into();
            
            self.current_round += 1;
            self.houses.clear();
            // records this winning
            self.win_history.push(&WinnerInfo {
                round: self.current_round,
                user: lucky_guy.clone(),
                lucky_number: god_choosen,
                amount: gross_reward,
                height: env::block_index(),
                ts: env::block_timestamp(),
            });
        }
        
        result
    }

}
