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
            const itemsNumber = 5;

            // create 5 items for proposals
            const items = {
                'Socks': 'In the original design', 
                'Apple': 'Eatable',
                'Chemex': 'Best way to make coffee',
                'Book': 'Here I am',
                'Cava': 'Signed by Arthur Brock',
            }

            for (const [item, desc] of Object.entries(items)) {
                let address = await alice.callSync('main', 'create_trade_proposal', {
                    name_of_item: item, 
                    description: desc
                })

                console.log(JSON.stringify(address, null, tabLength))
                t.equal(address.Ok.length, addressLength, `Alice created ${item} proposal successfully`)
            }

            // let Bob get the total number of entries 
            const totalTradeProposals = await bob.callSync('main', 'get_trade_proposals', {/* empty */})
            t.equal(totalTradeProposals.Ok.length, itemsNumber, `Bob can see all ${itemsNumber} proposals`)
            console.log(totalTradeProposals.Ok.length == itemsNumber)
        }
    )

}