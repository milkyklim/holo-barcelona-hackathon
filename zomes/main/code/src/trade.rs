use std::convert::TryFrom;
use hdk::{
    utils,
    error::{ZomeApiResult, ZomeApiError},
    holochain_persistence_api::{
        cas::content::{AddressableContent, Address},
    },
    holochain_json_api::{
        error::JsonError, json::JsonString,
    },
    holochain_core_types::{
        entry::Entry,
        link::LinkMatch,
    }
};

use crate::trade_action::Action;
use crate::TradeState;

#[derive(Serialize, Deserialize, Debug, DefaultJson,Clone)]
pub struct Trade {
    pub seller: Address,
    pub buyer: Address,
    pub created_at: u32,
}


/*=====================================
=            DHT Functions            =
=====================================*/

/// Traverse the linked list rooted at a trade to find all the actions
pub fn get_actions(trade_address: &Address) -> ZomeApiResult<Vec<Action>> {
    match hdk::get_links(trade_address, LinkMatch::Any, LinkMatch::Any)?.addresses().into_iter().next() {
        Some(first_action) => {
            let mut action_addresses = vec![first_action];
            let mut more = true;
            while more {
                more = match hdk::get_links(action_addresses.last().unwrap(), LinkMatch::Any, LinkMatch::Any)?.addresses().into_iter().next() {
                    Some(address) => {
                        action_addresses.push(address.clone());
                        true
                    },
                    None => {
                        false
                    },
                }
            }
            let actions: Vec<Action> = action_addresses.iter().map(|address| {
                let action_entry = hdk::get_entry(address).unwrap().unwrap();
                if let Entry::App(_, action_struct) = action_entry {
                    Action::try_from(action_struct).expect("Entry at address is type other than Action")
                } else {
                    panic!("Not an app entry!")
                }
            }).collect();
            Ok(actions)
        },
        None => {
            Ok(Vec::new())
        }
    }
}


pub fn get_state(trade_address: &Address) -> ZomeApiResult<TradeState> {
    let actions = get_actions(trade_address)?;
    let trade = get_trade(trade_address)?;
    let new_state = actions.iter().fold(TradeState::initial(), |state, new_action| state.evolve(trade.clone(), new_action));
    Ok(new_state)
}

pub fn get_trade(trade_address: &Address) -> ZomeApiResult<Trade> {
    utils::get_as_type(trade_address.to_owned())
}



/*=============================================
=            Local chain functions            =
=============================================*/

pub fn get_state_local_chain(local_chain: Vec<Entry>, trade_address: &Address) -> ZomeApiResult<TradeState> {
    let actions = get_actions_local_chain(local_chain.clone(), trade_address)?;
    let trade = get_trade_local_chain(local_chain, trade_address)?;
    let new_state = actions.iter().fold(TradeState::initial(), move |state, new_action| state.evolve(trade.clone(), new_action));
    Ok(new_state)
}

pub fn get_actions_local_chain(local_chain: Vec<Entry>, trade_address: &Address) -> ZomeApiResult<Vec<Action>> {
    Ok(local_chain
        .iter()
        .filter_map(|entry| {
            if let Entry::App(entry_type, entry_data) = entry {
                if entry_type.to_string() == "action" {
                    Some(Action::try_from(entry_data.clone()).unwrap())
                } else {
                    None
                }
            } else {
                None
            }
        })
        .filter(|trade_action| {
            trade_action.trade == trade_address.to_owned()
        })
        .rev()
        .collect())
}


pub fn get_trade_local_chain(local_chain: Vec<Entry>, trade_address: &Address) -> ZomeApiResult<Trade> {
    local_chain
        .iter()
        .filter(|entry| {
            entry.address() == trade_address.to_owned()
        })
        .filter_map(|entry| {
            if let Entry::App(_, entry_data) = entry {
                Some(Trade::try_from(entry_data.clone()).unwrap())
            } else {
                None
            }
        })
        .next()
        .ok_or(ZomeApiError::HashNotFound)
}


/*=====  End of Local chain functions  ======*/
