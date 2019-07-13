use hdk::holochain_persistence_api::{
    cas::content::Address,
};

use crate::Trade;
use crate::trade_action::Action;
use super::{
    TradeState,
    ActionType,
};

pub enum Role {
    Buyer,
    Seller,
}

pub fn get_current_role(trade: &Trade, user_address: &Address) -> Result<Role, String> {
    // FIXME
    match (user_address == &trade.buyer, user_address == &trade.seller) {
        (true, true) => return Err("Buyer cannot be seller".into()),
        (true, false) => Ok(Role::Buyer),
        (false, true) => Ok(Role::Seller),
        (false, false) => return Err("User is not participant of the trade!".into()),
    }
}

impl Action {
    pub fn is_valid(&self, trade: Trade, trade_state: TradeState) -> Result<(), String> {
        hdk::debug(format!("{:?}", trade_state)).unwrap();
        let _current_role = get_current_role(&trade, &self.author)?;

        // move type specific validation
        match &self.action_type {
            ActionType::Buy => {
                // TODO: Check is not bought
                is_sold(&trade_state)?;
                // from.is_in_bounds()?;
                // to.is_in_bounds()?;
                // from.is_piece_beloning_to_player(&current_role, &trade_state)?;
                // to.is_empty(&trade_state)?;
                // from.can_move_to(to, &current_role, &trade_state)?;
                hdk::debug("Validation Success!").unwrap();
                Ok(())
            }
        }
    }
}

fn is_sold(trade_state: &TradeState) -> Result<bool, String> {
    match trade_state.sold {
        false => Ok(true),
        true => Err("Item is sold".to_string()),
    }
}
