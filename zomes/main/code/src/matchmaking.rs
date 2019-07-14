use hdk::{
    AGENT_ADDRESS,
    error::ZomeApiResult,
    holochain_persistence_api::{
        cas::content::{Address},
    },
    holochain_json_api::{
        error::JsonError, json::{JsonString, default_to_json},
    },
    holochain_core_types::{
        entry::Entry,
        link::LinkMatch,
    }
};

use serde::Serialize;
use std::fmt::Debug;

use crate::trade::Trade;

#[derive(Serialize, Deserialize, Debug, DefaultJson,Clone)]
pub struct TradeProposal {
    pub seller: Address,
    pub name_of_item: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetResponse<T> {
    pub entry: T,
    pub address: Address
}

impl<T: Into<JsonString> + Debug + Serialize> From<GetResponse<T>> for JsonString {
    fn from(u: GetResponse<T>) -> JsonString {
        default_to_json(u)
    }
} 

pub fn handle_create_trade_proposal(name_of_item: String, description: String) -> ZomeApiResult<Address> {
    let trade_proposal_data = TradeProposal {
        seller: AGENT_ADDRESS.to_string().into(),
        name_of_item,
        description,
    };

    let entry = Entry::App("trade_proposal".into(), trade_proposal_data.into());
    let trade_proposal_address = hdk::commit_entry(&entry)?;

    let anchor_entry = Entry::App(
        "anchor".into(),
        "trade_proposals".into(),
    );
    let anchor_address = hdk::commit_entry(&anchor_entry)?;

    hdk::link_entries(
        &anchor_address,
        &trade_proposal_address,
        "has_trade_proposal",
        "",
    )?;
    
    Ok(trade_proposal_address)
}

pub fn handle_accept_trade_proposal(trade_proposal_address: Address, created_at: u32) -> ZomeApiResult<Address> {
    let trade_proposal: TradeProposal = hdk::utils::get_as_type(trade_proposal_address.clone())?;

    let trade_data = Trade {
        seller: trade_proposal.seller,
        buyer: AGENT_ADDRESS.to_string().into(),
        created_at,
    };

    let trade_entry = Entry::App(
        "trade".into(),
        trade_data.into()
    );

    let trade_address = hdk::commit_entry(&trade_entry)?;

    hdk::link_entries(
        &trade_proposal_address,
        &trade_address,
        "from_trade_proposal",
        ""
    )?;
    hdk::debug("accept trade proposal success!").unwrap();

    Ok(trade_address)
}

pub fn handle_check_responses(trade_proposal_address: Address) -> ZomeApiResult<Vec<Trade>> {
    hdk::utils::get_links_and_load_type(&trade_proposal_address, LinkMatch::Exactly("from_trade_proposal".into()), LinkMatch::Any)
}

/* 
 * Utils
 */
