module.exports = (scenario) => {

    // constants for all tests
    const addressLength = 46;
    const tabLength = 4;

    scenario('Alice can create trade proposal', async (s, t, {
        alice,
        bob
    }) => {
        const address = await alice.callSync('main', 'create_trade_proposal', {
            name_of_item: 'Socks',
            description: 'Old and smelly'
        })

        console.log(JSON.stringify(address, null, tabLength))
        t.equal(address.Ok.length, addressLength, 'Alice created proposal successfully')
    });

    scenario(
        'Alice creates multiple trade proposals and Bob gets the correct number of them',
        async (s, t, {
            alice,
            bob
        }) => {

            // console.log(JSON.stringify(alice, null, tabLength))
            // console.log(JSON.stringify(bob, null, tabLength))

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
                let address = await alice.callSync('main', 'create_trade_proposal', {
                    name_of_item: item,
                    description: desc
                })

                console.log(JSON.stringify(address, null, tabLength))
                t.equal(address.Ok.length, addressLength, `Alice created ${item} proposal successfully`)
            }

            // let Bob get the total number of entries 
            const totalTradeProposals = await bob.callSync('main', 'get_trade_proposals', {
                /* empty */
            })
            t.equal(totalTradeProposals.Ok.length, itemsNumber, `Bob can see all ${itemsNumber} proposals`)
            console.log(totalTradeProposals.Ok.length == itemsNumber)
        }
    );

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
                let address = await alice.callSync('main', 'create_trade_proposal', {
                    name_of_item: item,
                    description: desc
                })

                console.log(JSON.stringify(address, null, tabLength))
                t.equal(address.Ok.length, addressLength, `Alice created ${item} proposal successfully`)
            }

            let totalTradeProposals = await bob.callSync('main', 'get_trade_proposals', {
                /* empty */ })
            console.log(JSON.stringify(totalTradeProposals, null, tabLength))

            t.equal(totalTradeProposals.Ok.length, itemsNumber, `Bob can see ${itemsNumber} proposals`)

            let bobDiscardedAddress = totalTradeProposals.Ok[0].address
            let bobChoiceAddress = totalTradeProposals.Ok[1].address
            let address = await bob.callSync('main', 'accept_trade_proposal', {
                trade_proposal_address: bobChoiceAddress,
                created_at: Math.round(Date.now() / 1000)
            })

            // TODO: refactor and add code to utils.js

            // check that alice sees only itemsNumber - 1 trades now
            totalTradeProposals = await alice.callSync('main', 'get_trade_proposals', {
                /* empty */ })
            t.equal(totalTradeProposals.Ok.length, itemsNumber - 1, `Alice can see ${itemsNumber - 1} proposal(s)`)
            t.equal(totalTradeProposals.Ok[0].address, bobDiscardedAddress, `Alice sees the correct discarded option`)

            // check that bob sees only itemsNumber - 1 trades now
            totalTradeProposals = await bob.callSync('main', 'get_trade_proposals', {
                /* empty */ })
            t.equal(totalTradeProposals.Ok.length, itemsNumber - 1, `Bob can see ${itemsNumber - 1} proposal(s)`)
            t.equal(totalTradeProposals.Ok[0].address, bobDiscardedAddress, `Bob sees the correct discarded option`)

        }
    )
}