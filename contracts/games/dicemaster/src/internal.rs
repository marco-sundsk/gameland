use crate::*;

impl Contract {
    pub(crate) fn internal_max_reward(&self, bet_amount: Balance, bet_info: &BetInfo) -> Balance {
        if bet_info.category == 1 || bet_info.category == 2 {
            bet_amount * 2
        } else if bet_info.category == 3 {
            bet_amount * 150
        } else if bet_info.category == 4 {
            bet_amount * 24
        } else if bet_info.category == 5 {
            bet_amount * 5
        } else if bet_info.category == 6 {
            bet_amount * 8
        } else {
            env::panic(b"Internal Error: category invalid!");
        }
    }

    fn internal_judge(&self, bet_info: &BetInfo, dices: &Vec<u8>) -> u8 {
        // todo: judge win or lose, return odds
        if bet_info.category == 1 {
            self.internal_judge_1(bet_info, dices)
        } else if bet_info.category == 2 {
            self.internal_judge_2(bet_info, dices)
        } else if bet_info.category == 3 {
            self.internal_judge_3(bet_info, dices)
        } else if bet_info.category == 4 {
            self.internal_judge_4(bet_info, dices)
        } else if bet_info.category == 5 {
            self.internal_judge_5(bet_info, dices)
        } else if bet_info.category == 6 {
            self.internal_judge_6(bet_info, dices)
        } else {
            0
        }
        
    }

    /// big/small, [3, 10] vs [11 - 18], but if three dices are identical, player loses.
    fn internal_judge_1(&self, bet_info: &BetInfo, dices: &Vec<u8>) -> u8 {
        let dice1: &u8 = dices.get(0).expect("internal error: dice 0 is none");
        let dice2: &u8 = dices.get(1).expect("internal error: dice 0 is none");
        let dice3: &u8 = dices.get(2).expect("internal error: dice 0 is none");
        if dice1 == dice2 && dice1 == dice3 {
            return 0;
        }
        let dices_number = dice1 + dice2 + dice3;
        if dices_number >= 11 && bet_info.guess1 == 1 {
            // player bets big, and win
            return 1;
        }
        if dices_number <= 10 && bet_info.guess1 == 2 {
            // player bets small, and win
            return 1;
        }
        0
    }

    /// odd/even, but if three dices are identical, player loses.
    fn internal_judge_2(&self, bet_info: &BetInfo, dices: &Vec<u8>) -> u8 {
        let dice1: &u8 = dices.get(0).expect("internal error: dice 0 is none");
        let dice2: &u8 = dices.get(1).expect("internal error: dice 0 is none");
        let dice3: &u8 = dices.get(2).expect("internal error: dice 0 is none");
        if dice1 == dice2 && dice1 == dice3 {
            return 0;
        }
        let dices_number = dice1 + dice2 + dice3;
        if dices_number % 2 == 1 && bet_info.guess1 == 1 {
            // player bets odd, and win
            return 1;
        }
        if dices_number % 2 == 0 && bet_info.guess1 == 2 {
            // player bets even, and win
            return 1;
        }
        0
    }

    /// player names one number, if all three dices are equal to that number ,then win.
    fn internal_judge_3(&self, bet_info: &BetInfo, dices: &Vec<u8>) -> u8 {
        let dice1: &u8 = dices.get(0).expect("internal error: dice 0 is none");
        let dice2: &u8 = dices.get(1).expect("internal error: dice 0 is none");
        let dice3: &u8 = dices.get(2).expect("internal error: dice 0 is none");
        if dice1 == dice2 && dice1 == dice3 && *dice1 == bet_info.guess1 {
            150
        } else {
            0
        }
    }

    /// if all three dices are identical, then win.
    fn internal_judge_4(&self, _bet_info: &BetInfo, dices: &Vec<u8>) -> u8 {
        let dice1: &u8 = dices.get(0).expect("internal error: dice 0 is none");
        let dice2: &u8 = dices.get(1).expect("internal error: dice 0 is none");
        let dice3: &u8 = dices.get(2).expect("internal error: dice 0 is none");
        if dice1 == dice2 && dice1 == dice3 {
            24
        } else {
            0
        }
    }

    /// palyer names two numbers, if these nubmers appear in dices, then win. 
    fn internal_judge_5(&self, bet_info: &BetInfo, dices: &Vec<u8>) -> u8 {
        // if two numbers are different, judge like this
        if bet_info.guess1 != bet_info.guess2 {
            let exist1 = dices.iter().find(|&&x| x == bet_info.guess1 );
            let exist2 = dices.iter().find(|&&x| x == bet_info.guess2 );
            if exist1 != None && exist2 != None {
                5
            } else {
                0
            }
        } else {  // two same number, should appear at least twice in dices
            let exists = dices.iter().filter(|&&x| x == bet_info.guess1).map(|x| *x).collect::<Vec<u8>>();
            if exists.len() >= 2 {
                5
            } else {
                0
            }
        }
        
    }

    /// player names one number, if this number appears more than twice in dices, then win.
    fn internal_judge_6(&self, bet_info: &BetInfo, dices: &Vec<u8>) -> u8 {
        let exists = dices.iter().filter(|&&x| x == bet_info.guess1).map(|x| *x).collect::<Vec<u8>>();
        if exists.len() >= 2 {
            8
        } else {
            0
        }
    }

    pub(crate) fn internal_play(&mut self, account_id: &AccountId, bet_amount: u128, bet_info: &BetInfo) -> HumanReadableDiceResult {

        // rolling three dice here
        let randomness = env::random_seed();
        let rand1 = randomness[..10].iter().fold(0_u8, |acc, x| acc.wrapping_add(*x));
        let rand2 = randomness[10..20].iter().fold(0_u8, |acc, x| acc.wrapping_add(*x));
        let rand3 = randomness[20..30].iter().fold(0_u8, |acc, x| acc.wrapping_add(*x));

        let dice1 = 6_u16 * rand1 as u16 / 0x100_u16 + 1;
        let dice2 = 6_u16 * rand2 as u16 / 0x100_u16 + 1;
        let dice3 = 6_u16 * rand3 as u16 / 0x100_u16 + 1;
        let dice1 = dice1 as u8;
        let dice2 = dice2 as u8;
        let dice3 = dice3 as u8;

        let mut reward: u128 = 0;
        let odds = self.internal_judge(bet_info, &vec![dice1, dice2, dice3]);
        if odds != 0 {
            reward = bet_amount.checked_mul(odds as u128).expect("Internal Error: reward overflow!");
            reward += bet_amount;
            self.jackpot -= reward;

            self.win_history.push(&WinnerInfo {
                user: account_id.clone(),
                category: bet_info.category,
                odds: odds,
                reward,
                height: env::block_index(),
                ts: env::block_timestamp(),
            });
        }

        HumanReadableDiceResult {
            user: account_id.clone(),
            dice_point: vec![dice1, dice2, dice3],
            reward_amount: reward.into(),  
            jackpot_left: self.jackpot.into(),
            height: env::block_index().into(),
            ts: env::block_timestamp().into(),
            ret_code: 0,
            reason: String::from("Successful played."),
        }
    }

}
