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

pub trait GameMetadataProvider {
    fn gl_metadata() -> GameMetadata;
    fn gl_pub_state(&self) -> String;
    fn gl_user_state(&self, user_id: ValidAccountId) -> String;
}

#[near_bindgen]
impl GameMetadataProvider for Contract {
    fn gl_metadata() -> GameMetadata {
        GameMetadata {
            version: String::from("0.1.0"),
            name: String::from("Dice Master"),
            logo_url: String::from("https://github.com/"),
            thumb_url: String::from("https://github.com/"),
            description: String::from("Let's Rolling Dices"),
        }
    }

    fn gl_pub_state(&self) -> String {
        format!("Current jackpot is {}", self.jack_pod)
    }

    fn gl_user_state(&self, user_id: ValidAccountId) -> String {
        // todo: 
        format!("", )
    }

}
