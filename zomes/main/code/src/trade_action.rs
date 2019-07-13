use hdk::{
    entry_definition::ValidatingEntryType,
    holochain_persistence_api::{
        cas::content::{Address},
    },
    holochain_json_api::{
        error::JsonError, json::JsonString,
    },
    holochain_core_types::{
        dna::entry_types::Sharing,
        validation::EntryValidationData,
        entry::Entry,
    }
};

use crate::trade::{get_state_local_chain, get_trade_local_chain};

use crate::ActionType;

#[derive(Clone, Debug, Serialize, Deserialize, DefaultJson, PartialEq)]
pub struct Action {
    pub trade: Address,
    pub author: Address,
    pub action_type: ActionType,
    pub previous_action: Address,
    pub timestamp: u32,
}

pub fn definition() -> ValidatingEntryType {
    entry!(
        name: "action",
        description: "An action by an agent in a trade",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::ChainFull
        },

        validation: | validation_data: hdk::EntryValidationData<Action>| {
            match validation_data {
                EntryValidationData::Create{entry, validation_data} => {
                    let mut local_chain = validation_data.package.source_chain_entries
                            .ok_or("Could not retrieve source chain")?;
                    hdk::debug(format!("{:?}", local_chain))?;

                    let new_action = Action::from(entry);

                    // Sometimes the validating entry is already in the chain when validation runs,
                    // To make our state reduction work correctly this must be removed
                    local_chain.remove_item(&Entry::App("action".into() , new_action.clone().into()));

                    let state = get_state_local_chain(local_chain.clone(), &new_action.trade)
                            .map_err(|_| "Could not load state during validation")?;
                    let trade = get_trade_local_chain(local_chain, &new_action.trade)
                        .map_err(|_| "Could not load trade during validation")?;
                    
                    new_action.is_valid(trade, state)
                },
                _ => {
                    Err("Cannot modify or delete a action".into())
                }
            }
        },

        links: [
        	from!(
                "trade",
                link_type: "",
                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },
                validation: | _validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            ),
        	from!(
                "action",
                link_type: "",
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
