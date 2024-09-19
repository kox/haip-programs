# Keypairs

Generate keypairs for the feepayer and haip token mint

```bash
solana-keygen new --outfile keypairs/feepayer.json
solana-keygen new --outfile keypairs/haip_token_mint.json
```

# Campaign

A campaign is a NFT which contains most of the campaign description in the offchain metadata plus, some onchain relevant information. We will use Metaplex core which provides a good number of plugins to make it more powerful and cover more use cases out of the box.

* A user will be able to fill up a form and pay to create a campaign proposal (NFT). 
* The state (proposal), will be an attribute of the campaign NFT.
* Once the proposed campaign gets stacked (not possible to transfer, sell or burn), it will get ready for voting.
* Users with HAIP will be able to vote for the campaign (1 user -> 1 vote). It won't be possible to vote twice. To vote, they will be able to yes, no, and also give a commentary. The commentary will be kept in their PDA.
* It will require a number of likes for a proposal campaign to be approved.
* Each proposal campaign will have a time window to vote. Once the votes reach the limit will pass to approved. If the time windows ends and the proposal didn't get enough votes, it will be denied. 
* Onces a campaign has started, it will create a vault to send funds. 
* It will have some limits (max funds, soft funds, min deposit, max deposit) and a type of share (1:1 by default).
* Each user who will deposit funds to the vault, will get some share tokens to track their contributions.
* Once the funding periods ends, no more users will be able to fund the vault and receive shared tokens.
* If a campaign doesn't reach the soft limit, all funds will get returned and all accounds and PDAs will get closed.
* If a campaign gets funded, the supporters will require to stake their tokens. (non custodial staking)
* If a user has staked their funds, they will be able to receive rewards.




