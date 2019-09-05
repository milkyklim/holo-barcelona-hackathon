/**
 * Collection of functions to abstract the Holochain calls
 * and make the testing code cleaner.
 */

module.exports = {
    createTradeProposal: async (agent, nameOfItem, description) => {
        const result = await agent.callSync('main', 'create_trade_proposal', {
            name_of_item: nameOfItem,
            description: description,
        })
        return result
    },

    getTradeProposals: async (agent) => {
        const result = await agent.callSync('main', 'get_trade_proposals', {
            /* empty */
        })
        return result
    },

    acceptTradeProposal: async (agent, tradeProposalAddress, createdAt = Math.round(Date.now() / 1000)) => {
        const result = await agent.callSync('main', 'accept_trade_proposal', {
            trade_proposal_address: tradeProposalAddress,
            created_at: createdAt,
        })
        return result
    },
}