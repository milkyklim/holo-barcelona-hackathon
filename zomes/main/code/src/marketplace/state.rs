use hdk::holochain_json_api::{
    error::JsonError, json::JsonString,
};

use crate::trade_action::Action;
use crate::trade::Trade;
use super::{
    // Actions::Piece,
    ActionType,
    validation::{get_current_role},
};

#[derive(Clone, Debug, Serialize, Deserialize, DefaultJson)]
pub struct TradeState {
    pub actions: Vec<Action>,
    // pub buyer: Address,
    // pub seller: Address,
    pub sold: bool,
}

impl TradeState {
    pub fn initial() -> Self {
        TradeState {
            actions: Vec::new(),
            sold: false,
        }
    }

    pub fn render(&self) -> String {
        "".to_string()
    }

    pub fn evolve(&self, trade: Trade, next_action: &Action) -> Self {
        let _current_role = get_current_role(&trade, &next_action.author).unwrap();
        let mut actions = self.clone().actions.clone();
        actions.push(next_action.to_owned());

        match &next_action.action_type {
            ActionType::Buy => {
                TradeState {
                    actions,
                    sold: true,
                    ..self.clone()
                }
            }
        }

    }
}
