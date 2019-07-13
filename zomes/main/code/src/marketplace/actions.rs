use hdk::holochain_json_api::{
    error::JsonError, json::JsonString,
};

#[derive(Clone, Debug, Serialize, Deserialize, DefaultJson, PartialEq)]
pub enum ActionType {
    Buy
}

impl ActionType {
    pub fn describe() -> Vec<ActionType> {
        vec![
            ActionType::Buy,
        ]
    }
}

