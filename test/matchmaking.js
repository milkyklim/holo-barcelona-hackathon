const {
    results,
    lastResult,
    addressLength,
    tabLength,
    createTradeProposal,
    getTradeProposals,
    acceptTradeProposal,
} = require('./utils')

const checkTradeProposalAccepted = async (t, agent, agentName, itemsNumber, discardedAddress) => {
    // check that agent sees only itemsNumber - 1 trades now
    totalTradeProposals = await getTradeProposals(agent)
    t.equal(totalTradeProposals.Ok.length, itemsNumber - 1, `${agentName} can see ${itemsNumber - 1} proposal(s)`)
    t.equal(totalTradeProposals.Ok[0].address, discardedAddress, `${agentName} sees the correct discarded option`)
}

module.exports = (scenario) => {
    scenario('Alice can create trade proposal', async (s, t, {
        alice,
        bob
    }) => {
        await createTradeProposal(alice, 'Socks', 'Old and smelly')
        t.equal(lastResult().Ok.length, addressLength, 'Alice created proposal successfully')
        console.log(JSON.stringify(lastResult(), null, tabLength))
    })

    scenario(
        'Alice creates multiple trade proposals and Bob gets the correct number of them',
        async (s, t, {
            alice,
            bob
        }) => {
            // create 5 items for proposals
            const items = {
                'Socks': 'In the original design',
                'Apple': 'Eatable',
                'Chemex': 'Best way to make coffee',
                'Book': 'Here I am',
                'Cava': 'Signed by Arthur Brock',
            }

            const itemsNumber = Object.keys(items).length;

            for (const [item, desc] of Object.entries(items)) {
                await createTradeProposal(alice, item, desc)
                t.equal(lastResult().Ok.length, addressLength, `Alice created ${item} proposal successfully`)
            }

            // let Bob get the total number of entries 
            await getTradeProposals(bob)
            t.equal(lastResult().Ok.length, itemsNumber, `Bob can see all ${itemsNumber} proposals`)

            // print all results
            results.forEach((result, i) => {
                console.log(`${i}: ${JSON.stringify(result, null, tabLength)}\n`)
            })
        }
    )

    scenario('Alice creates 2 trade proposals and Bob accepts one trade proposal',
        async (s, t, {
            alice,
            bob
        }) => {
            // create 2 items for proposals
            const items = {
                'Red Wine': 'Cheap with awful taste',
                'White Wine': 'Expensive with fruity aftertaste',
            }

            const itemsNumber = Object.keys(items).length;

            for (const [item, desc] of Object.entries(items)) {
                await createTradeProposal(alice, item, desc)
                t.equal(lastResult().Ok.length, addressLength, `Alice created ${item} proposal successfully`)
            }

            await getTradeProposals(bob)
            t.equal(lastResult().Ok.length, itemsNumber, `Bob can see ${itemsNumber} proposals`)
            console.log(JSON.stringify(lastResult(), null, tabLength))

            let bobDiscardedAddress = lastResult().Ok[0].address
            let bobChoiceAddress = lastResult().Ok[1].address

            // uses default created_at
            await acceptTradeProposal(bob, bobChoiceAddress)
            // check that alice sees only itemsNumber - 1 trades now
            await checkTradeProposalAccepted(t, alice, 'Alice', itemsNumber, bobDiscardedAddress)
            // check that bob sees only itemsNumber - 1 trades now
            await checkTradeProposalAccepted(t, bob, 'Bob', itemsNumber, bobDiscardedAddress)

            results.forEach((result, i) => {
                console.log(`${i}: ${JSON.stringify(result, null, tabLength)}\n`)
            })
        }
    )

    scenario('Alice creates a trade proposal and tries to accept it herself', async (s, t, {
        alice,
        bob
    }) => {
        // TODO:
    })
}