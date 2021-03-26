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
            version: String::from("0.1.1"),
            name: String::from("Lucky Box"),
            logo_url: String::from("https://github.com/"),
            thumb_url: String::from("https://github.com/"),
            description: String::from("Choose the lightest box to bet"),
        }
    }

    fn gl_pub_state(&self) -> String {
        format!("Round #{}, Current total jackpot is {}", 
            self.current_round, self.total_jackpot)
    }

    fn gl_user_state(&self, user_id: ValidAccountId) -> String {
        let user: AccountId = user_id.into();
        format!("Welcome {}", user)
    }

}
