const {
  createTradeProposal,
  getTradeProposals,
  acceptTradeProposal,
} = require('./utils');

const createTradeProposals = async (items, seller) => {
  for (const [item, desc] of Object.entries(items)) {
    await createTradeProposal(seller, item, desc);
  }
};

module.exports = scenario => {
  scenario('Alice can create trade proposals', async (s, t, { alice }) => {
    let tradeProposals = (await getTradeProposals(alice)).Ok;
    t.equal(tradeProposals.length, 0, 'There are no trade proposals available');
    let proposalAddress = await createTradeProposal(
      alice,
      'Socks',
      'Old and smelly',
    );
    t.equal(
      proposalAddress.Ok,
      'QmZFbXGzFWpBPXSh2jZMRYq41VJuQ7rt5f6dD8MFSHAeck',
      'Alice created proposal successfully',
    );

    tradeProposals = (await getTradeProposals(alice)).Ok;
    t.equal(
      tradeProposals.length,
      1,
      'Alice get correct number of trade proposals',
    );
    proposalAddress = await createTradeProposal(alice, 'Apple', 'Eatable');
    tradeProposals = (await getTradeProposals(alice)).Ok;
    t.equal(
      tradeProposals.length,
      2,
      'Alice get correct number of trade proposals',
    );
  });

  scenario(
    'Bob gets the correct number of created proposals',
    async (s, t, { alice, bob }) => {
      // create 5 items for proposals
      const items = {
        Socks: 'In the original design',
        Apple: 'Eatable',
        Chemex: 'Best way to make coffee',
        Book: 'Here I am',
        Cava: 'Signed by Arthur Brock',
      };

      const itemsNumber = Object.keys(items).length;

      await createTradeProposals(items, alice);

      // let Bob get the total number of entries
      const { Ok: tradeProposals } = await getTradeProposals(bob);
      t.equal(
        tradeProposals.length,
        itemsNumber,
        `Bob can see all ${itemsNumber} proposals`,
      );
    },
  );

  scenario(
    'Bob can accept Alice trade proposal',
    async (s, t, { alice, bob }) => {
      // create 2 items for proposals
      const items = {
        'Red Wine': 'Cheap with awful taste',
        'White Wine': 'Expensive with fruity aftertaste',
      };

      const itemsNumber = Object.keys(items).length;

      await createTradeProposals(items, alice);

      const { Ok: tradeProposals } = await getTradeProposals(bob);

      let bobDiscardedAddress = tradeProposals[0].address;
      let bobChoiceAddress = tradeProposals[1].address;

      // uses default created_at
      await acceptTradeProposal(bob, bobChoiceAddress);
      // check that alice sees only proper trade now
      const { Ok: aliceTotalTradeProposals } = await getTradeProposals(alice);
      t.equal(
        aliceTotalTradeProposals.length,
        itemsNumber - 1,
        `Alice can see ${itemsNumber - 1} proposal(s)`,
      );
      t.equal(
        aliceTotalTradeProposals[0].address,
        bobDiscardedAddress,
        `Alice sees the correct discarded option`,
      );
      // check that bob sees only proper trade now
      const { Ok: bobTotalTradeProposals } = await getTradeProposals(alice);
      t.equal(
        bobTotalTradeProposals.length,
        itemsNumber - 1,
        `Bob can see ${itemsNumber - 1} proposal(s)`,
      );
      t.equal(
        bobTotalTradeProposals[0].address,
        bobDiscardedAddress,
        `Bob sees the correct discarded option`,
      );
    },
  );

  // scenario(
  // 'Alice creates a trade proposal and tries to accept it herself',
  // async () => {
  // // TODO
  // },
  // );
};
