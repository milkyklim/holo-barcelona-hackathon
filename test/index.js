const path = require('path')
const tape = require('tape')

const { 
  Diorama, 
  tapeExecutor, 
  backwardCompatibilityMiddleware
} = require('@holochain/diorama')

process.on('unhandledRejection', error => {
  // Will print "unhandledRejection err is not defined"
  console.error('got unhandledRejection:', error);
});

const dnaPath = path.join(__dirname, "../dist/holo-barcelona-hackathon.dna.json")
const dna = Diorama.dna(dnaPath, 'holo-barcelona-hackathon')

const diorama = new Diorama({
  instances: {
    alice: dna,
    bob: dna,
  },
  bridges: [],
  debugLog: false,
  executor: tapeExecutor(require('tape')),
  middleware: backwardCompatibilityMiddleware,
})

// test the matchmaking 
require('./matchmaking')(diorama.registerScenario)

diorama.run()
