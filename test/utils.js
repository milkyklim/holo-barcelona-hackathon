/**
 * Collection of functions to abstract the Holochain calls
 * and make the testing code cleaner.
 */

// constants for all tests
const addressLength = 46;
const tabLength = 4;

let results = [] 

module.exports = {
    addressLength: addressLength,
    tabLength: tabLength,
    results: results, 
    lastResult: (back = 0) => results[results.length - 1 - back],

    createTradeProposal: async (agent, nameOfItem, description) => {
        const result = await agent.callSync('main', 'create_trade_proposal', {
            name_of_item: nameOfItem,
            description: description,
        })
        results.push(result)
        return result
    },

    getTradeProposals: async (agent) => {
        const result = await agent.callSync('main', 'get_trade_proposals', {
            /* empty */
        })
        results.push(result)
        return result
    },

    acceptTradeProposal: async (agent, tradeProposalAddress, createdAt = Math.round(Date.now() / 1000)) => {
        const result = await agent.callSync('main', 'accept_trade_proposal', {
            trade_proposal_address: tradeProposalAddress,
            created_at: createdAt,
        })
        results.push(result)
        return result
    },
}