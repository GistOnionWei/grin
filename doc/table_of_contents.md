# Documentation structure

## Explaining libercoin

- [intro](intro.md) - Technical introduction to libercoin
- [libercoin4bitcoiners](libercoin4bitcoiners.md) - Explaining libercoin from a bitcoiner's perspective

## Understand the libercoin implementation

- [chain_sync](chain/chain_sync.md) - About how Libercoin's blockchain is synchronized
- [blocks_and_headers](chain/blocks_and_headers.md) - How Libercoin tracks blocks and headers on the chain
- [contract_ideas](contract_ideas.md) - Ideas on how to implement contracts
- [dandelion/dandelion](dandelion/dandelion.md) - About transaction propagation and cut-through. Stemming and fluffing!
- [dandelion/simulation](dandelion/simulation.md) - Dandelion simulation - aggregating transaction without lock_height Stemming and fluffing!
- [internal/pool](internal/pool.md) - Technical explanation of the transaction pool
- [merkle](merkle.md) - Technical explanation of libercoin's favorite kind of merkle trees
- [merkle_proof graph](merkle_proof/merkle_proof.png) - Example merkle proof with pruning applied
- [pruning](pruning.md) - Technical explanation of pruning
- [stratum](stratum.md) - Technical explanation of Libercoin Stratum RPC protocol
- [transaction UML](wallet/transaction/basic-transaction-wf.png) - UML of an interactive transaction (aggregating transaction without `lock_height`)

## Build and use

- [api](api/api.md) - Explaining the different APIs in Libercoin and how to use them
- [build](build.md) - Explaining how to build and run the Libercoin binaries
- [release](release_instruction.md) - Instructions of making a release
- [usage](usage.md) - Explaining how to use libercoin in Testnet3
- [wallet](wallet/usage.md) - Explains the wallet design and `libercoin wallet` sub-commands

## External (wiki)

- [FAQ](https://github.com/mimblewimble/docs/wiki/FAQ) - Frequently Asked Questions
- [Building libercoin](https://github.com/mimblewimble/docs/wiki/Building)
- [How to use libercoin](https://github.com/mimblewimble/docs/wiki/How-to-use-libercoin)
- [Hacking and contributing](https://github.com/mimblewimble/docs/wiki/Hacking-and-contributing)
