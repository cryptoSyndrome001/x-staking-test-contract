## task
Using Anchor, we'd like you to build a basic xToken staking program. A user should be able to deposit a specific token into a program-controlled vault and receive a proof of staking (PoS) token 1:1, and conversely redeem the PoS token for the locked tokens. Tests must be included.

## Implementation
1. Unit tests are done already but there is a simple bug that we made for this test. Fix it.
2. complete events(TreasuryCreated, Deposited, Claimed) and trigger it in the unit tests

coding & testing time: within 30~50 minutes

how to run local tests:

yarn install
anchor build
anchor test

## Answer
1. Token account in which its token balance is changing did not set to mut in the redeem struct, so redeem transaction did not run.
2. Completed the event struct and leave the log in the transaction, and parsed it to see the detail log data with web3.
