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
            name: String::from("Land Lord"),
            logo_url: String::from("https://github.com/"),
            thumb_url: String::from("https://github.com/"),
            description: String::from("Choose lucking house"),
        }
    }

    fn gl_pub_state(&self) -> String {
        let remain = self.house_count - self.houses.keys_as_vector().len() as u8;
        format!("Round {}: {} block remain. Current jackpot has {} GPT(s).", 
            self.current_round, remain, self.jack_pod/1000000000000000000000000)
    }

    fn gl_user_state(&self, user_id: ValidAccountId) -> String {

        let keys = self.houses.keys_as_vector();
        // let mut players = HashMap::new();
        // for index in 0..keys.len() {
        //     players.insert(
        //         self.houses.get(&keys.get(index).unwrap()).unwrap_or(String::from("")),
        //         keys.get(index).unwrap(), 
        //     ); 
        // }

        let players = (0..keys.len()).map(
            |index| (
                self.houses.get(&keys.get(index).unwrap()).unwrap_or(String::from("")),
                keys.get(index).unwrap()
                )
        ).collect::<HashMap<_,_>>();

        if players.contains_key(user_id.as_ref()) {
            format!("You have taken house #{}", players.get(user_id.as_ref()).unwrap())
        } else {
            format!("Time to choose your house", )
        }
        
    }

}
