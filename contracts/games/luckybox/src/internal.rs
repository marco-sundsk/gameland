use crate::*;

// pub(crate) fn assert_self() {
//     assert_eq!(
//         env::predecessor_account_id(),
//         env::current_account_id(),
//         "Method is private"
//     );
// }

impl Contract {
    /// found the box with minimum total_amount
    pub(crate) fn internal_get_winbox(&self) -> Option<BoxInfo> {
        let min_id = self.boxes.values().fold(0_u8, |id, box_info| {
            if id == 0 || box_info.total_amount < self.boxes.get(&id).unwrap().total_amount {
                box_info.box_id
            } else {
                id
            }
        });
        self.boxes.get(&min_id)
    }

    /// see if current round is ended, and return the winners info as a HashMap
    pub(crate) fn internal_settle(&mut self) -> (bool, HashMap<AccountId, Balance>) {
        if self.current_round_start_height > 0 {
            // current round has started, need settle
            // with a 0 - 255 randomness
            let random_u8: u8 = env::random_seed().iter().fold(0_u8, |acc, x| acc.wrapping_add(*x));
            let cur_height = env::block_index();
            let end_height = self.current_round_start_height + self.per_round_last as u64 + random_u8 as u64;
            if cur_height >= end_height {
                // record win history
                let total_reward = self.total_jackpot / 2;
                let lucky_box = self.boxes.get(&(self.current_min_box_id)).unwrap();
                let winners = lucky_box.investors.keys().map(
                    |key| {
                        let winner = key.clone();
                        let winner_share = lucky_box.investors.get(key).unwrap();
                        let winner_reward = total_reward * winner_share / lucky_box.total_amount;
                        (winner, winner_reward)
                    }
                ).collect::<HashMap<AccountId, Balance>>();
                let win_his = WinnerInfo {
                    round: self.current_round,
                    total_reward,
                    winners: winners.clone(),
                    height: env::block_index(),
                    ts: env::block_timestamp(),  
                };
                self.win_history.push(&win_his);

                // reset status
                self.total_jackpot -= total_reward;
                self.current_round += 1;
                self.current_round_start_height = 0;
                self.boxes.clear();
                self.current_min_box_id = 1;
                // return value
                return (true, winners);
            }
        }
        (false, HashMap::new())
    }

    pub(crate) fn internal_play(&mut self, account_id: &AccountId, gross_amount: u128, net_amount: u128, box_id: u8) -> HumanReadablePlayResult {
        
        self.total_jackpot += net_amount;
        let mut result = HumanReadablePlayResult {
            user: account_id.clone(),
            round: self.current_round.into(),
            box_id,
            box_amount: gross_amount.into(),  
            total_jackpot: self.total_jackpot.into(), 
            height: env::block_index().into(),
            ts: env::block_timestamp().into(),
        };

        if let Some(mut box_info) = self.boxes.get(&box_id) {
            box_info.height = env::block_index();
            box_info.ts = env::block_timestamp();
            box_info.total_amount += gross_amount;
            let amount = box_info.investors.get_mut(account_id).unwrap_or(&mut 0_u128).clone();
            result.box_amount = (amount + gross_amount).into();
            box_info.investors.insert(account_id.clone(), amount + gross_amount);
            // save back to storage
            self.boxes.insert(&box_id, &box_info);
        } else {
            let mut investors: HashMap<AccountId, Balance> = HashMap::new();
            investors.insert(account_id.clone(), gross_amount);
            let new_box = BoxInfo {
                box_id,
                total_amount: gross_amount,
                investors,
                height: env::block_index(),
                ts: env::block_timestamp(),
            };
            // save back to storage
            self.boxes.insert(&box_id, &new_box);
        }

        // see if it triggers a round start
        if self.current_round_start_height == 0 {
            self.current_round_start_height = env::block_index();
            self.current_min_box_id = box_id;
        } else {
            self.current_min_box_id = self.internal_get_winbox().unwrap().box_id;
        }

        result
    }

}
