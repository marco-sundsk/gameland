use super::*;
use near_sdk::serde::Serialize;

#[derive(Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct GameMetadata {
    version: String,
    name: String,
    logo_url: String,
    thumb_url: String,
    description: String,
}

#[derive(Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct GameStatistic {
    play_count: U128,  // counter of play action called
    winner_count: U128,  // counter for winners
    reward_sum: U128,  // total reward distributed
}

pub trait GameMetadataProvider {
    fn gl_metadata() -> GameMetadata;
    fn gl_statistic(&self) -> GameStatistic;
    fn gl_pub_state(&self) -> String;
    fn gl_user_state(&self, user_id: ValidAccountId) -> String;
}

#[near_bindgen]
impl GameMetadataProvider for Contract {
    fn gl_metadata() -> GameMetadata {
        GameMetadata {
            version: String::from("0.1.0"),
            name: String::from("Dice Master 2"),
            logo_url: String::from("dicemaster.testnet.logo.png"),
            thumb_url: String::from("dicemaster.testnet.thumb.png"),
            description: String::from("Let's Rolling Dices"),
        }
    }

    fn gl_statistic(&self) -> GameStatistic {
        GameStatistic {
            play_count: self.gl_play_count.into(),
            winner_count: self.gl_winner_count.into(),
            reward_sum: self.gl_reward_sum.into(),
        }
    }

    fn gl_pub_state(&self) -> String {
        format!("Current jackpot has {} GPT(s).", self.jackpot/1000000000000000000000000)
    }

    fn gl_user_state(&self, user_id: ValidAccountId) -> String {
        // todo: 
        format!("Welcome {}", user_id.as_ref())
    }

}
