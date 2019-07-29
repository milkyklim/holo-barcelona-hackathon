#![feature(vec_remove_item)]
#![feature(proc_macro_hygiene)]
#[macro_use]
extern crate hdk;
extern crate hdk_proc_macros;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate holochain_json_derive;

mod marketplace;
use marketplace::{TradeState, ActionType};

mod matchmaking;
mod trade;
mod trade_action;

use matchmaking::{TradeProposal};
use trade::{Trade};

use hdk::{
    entry_definition::ValidatingEntryType,
    error::ZomeApiResult,
    holochain_persistence_api::{
        cas::content::{AddressableContent, Address},
    },
    holochain_json_api::json::JsonString,
    holochain_core_types::{
        entry::Entry,
        dna::entry_types::Sharing,
        validation::EntryValidationData,
        link::LinkMatch,
    },
};

use hdk_proc_macros::zome;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetResponse<T> {
    pub entry: T,
    pub address: Address
}


// see https://developer.holochain.org/api/0.0.18-alpha1/hdk/ for info on using the hdk library

// This is a sample zome that defines an entry type "MyEntry" that can be committed to the
// agent's chain via the exposed function create_my_entry
#[zome]
mod main {

    #[genesis]
    fn genesis() {
        Ok(())
    }

    // This is 0.0.25-alpha1 code
    // #[init]
    // fn init() {
    //     Ok(())
    // }

    #[entry_def]
    fn trade_proposal_entry_def() -> ValidatingEntryType {
        entry!(
            name: "trade_proposal",
            description: "Represents a trade proposal",
            sharing: Sharing::Public,
            validation_package: || {
                hdk::ValidationPackageDefinition::Entry
            },
            validation: | validation_data: hdk::EntryValidationData<TradeProposal>| {
                match validation_data {
                    EntryValidationData::Create{ entry, validation_data } => {
                        let trade_proposal = TradeProposal::from(entry);
                        if validation_data.sources().contains(&trade_proposal.seller) {
                            Ok(())
                        } else {
                            Err("Cannot author a proposal from another agent".into())
                        }
                        
                    },
                    _ => {
                        Err("Cannot modify, only create and delete".into())
                    }
                }
            }
        )
    }

    #[entry_def]
    fn trade_action_entry_def() -> ValidatingEntryType {
        trade_action::definition()
    }

    #[zome_fn("hc_public")]
    fn get_state(trade_address: Address) -> ZomeApiResult<TradeState> {
        trade::get_state(&trade_address)
    }

    #[zome_fn("hc_public")]
    fn render_state(trade_address: Address) -> ZomeApiResult<String> {
        // TODO: I've added '?' in the end not sure if this is the right way to do it
        // hdk::debug(format!("trade_address: {}", trade_address))?;
        Ok(trade::get_state(&trade_address)?.render())
    }

    #[zome_fn("hc_public")]
    fn create_trade_proposal(name_of_item: String, description: String) -> ZomeApiResult<Address> {
        matchmaking::handle_create_trade_proposal(name_of_item, description)
    }

    #[zome_fn("hc_public")]
    fn accept_trade_proposal(trade_proposal_address: Address, created_at: u32) -> ZomeApiResult<Address> {
        matchmaking::handle_accept_trade_proposal(trade_proposal_address, created_at)
    }

    #[zome_fn("hc_public")]
    pub fn check_responses(trade_proposal_address: Address) -> ZomeApiResult<Vec<Trade>> {
        matchmaking::handle_check_responses(trade_proposal_address)
    }

    #[zome_fn("hc_public")]
    fn remove_trade_proposal(trade_proposal_address: Address) -> ZomeApiResult<Address> {
        hdk::remove_entry(&trade_proposal_address)
    }

    #[zome_fn("hc_public")]
    fn get_trade_proposals() -> ZomeApiResult<Vec<GetResponse<TradeProposal>>> {
        let anchor_address = Entry::App(
            "anchor".into(),
            "trade_proposals".into()
        ).address();

        let trade_proposals = hdk::utils::get_links_and_load_type(
            &anchor_address, 
            LinkMatch::Exactly("has_trade_proposal"), // the link type to match,
            LinkMatch::Any,
        )?.into_iter().filter_map(|proposal: TradeProposal| {
            let address = Entry::App("trade_proposal".into(), proposal.clone().into()).address();
            let trades = matchmaking::handle_check_responses(address.clone());
            match trades.unwrap().len() {
                0 => Some(GetResponse{entry: proposal, address}),
                _ => None,
            }
            }).collect();

        Ok(trade_proposals)
    }

    #[entry_def]
    fn trade_entry_def() -> ValidatingEntryType {
        entry!(
            name: "trade",
            description: "Represents a trade",
            sharing: Sharing::Public,
            validation_package: || {
                hdk::ValidationPackageDefinition::Entry
            },
            validation: | _validation_data: hdk::EntryValidationData<Trade>| {
                Ok(())
            },
            links: [
                from!(
		    "trade_proposal",
		    link_type: "from_trade_proposal",
		    validation_package: || {
			hdk::ValidationPackageDefinition::Entry
		    },
		    validation: | _validation_data: hdk::LinkValidationData| {
			Ok(())
		    }
		)
	    ]
        )
    }


    #[entry_def]
    pub fn anchor_def() -> ValidatingEntryType {
        entry!(
            name: "anchor",
            description: "Central known location to link from",
            sharing: Sharing::Public, 
            validation_package: || { hdk::ValidationPackageDefinition::Entry },
            validation: | _validation_data: hdk::EntryValidationData<String>| {
                Ok(())
            },
            links: [
                to!(
                    "trade_proposal", // this must match exactly the target entry type
                    link_type: "has_trade_proposal", // must use this when creating the link
                    validation_package: || {
                        hdk::ValidationPackageDefinition::Entry
                    },
                    validation: | _validation_data: hdk::LinkValidationData| {
                        Ok(())
                    }
                )
            ]
        )
    }

    // This is 0.0.25-alpha1 code
    // TODO: this should probably be added to necessary validation spots
    // #[validate_agent]
    // pub fn validate_agent(_validation_data: hdk::LinkValidationData) {
    //     Ok(())
    // }
}
