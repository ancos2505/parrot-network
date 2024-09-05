## Transactions validation

These algorithms are designed to maintain the directed acyclic nature of the DAG, ensuring all transactions can trace their validation path back to the genesis transaction.

There are established algorithms and approaches that ensure all transactions in a DAG-based DLT trace back to the genesis transaction while maintaining the acyclic structure. These methods are crucial to maintaining the integrity and security of the network.


### Proven Algorithms and Approaches


#### Tip Selection Algorithms

1. **Random Walk Monte Carlo (RWMC):** Used in **IOTA's Tangle**, this algorithm helps select which previous transactions (tips) to validate when adding a new transaction. The idea is to randomly walk through the DAG, favoring tips that are well-connected, thus ensuring the structure always traces back to the genesis transaction.

2. **Markov Chain Monte Carlo (MCMC):** An advanced version of RWMC that uses a probability model to guide the selection of tips in a way that balances between newly added and well-established transactions.


#### Depth-First Search (DFS) and Breadth-First Search (BFS)

- These graph traversal algorithms can be used to verify that every transaction in the DAG eventually traces back to the genesis transaction.

- During validation, each node can perform a DFS or BFS from its position to check if it can reach the genesis transaction, ensuring the transaction history is continuous and acyclic.


#### Virtual Voting and Consensus Mechanisms

- **Avalanche Consensus:** Uses a DAG structure where nodes query each other to decide on transaction validity through repeated sampling and voting, leading to a consensus that all transactions are correctly linked back to the genesis.

- **Tangle (IOTA's DAG Model):** In IOTA, every new transaction must validate two previous transactions. This "tip" validation process ensures the DAG grows in a way that all transactions indirectly validate and connect back to the genesis.

#### Graph Pruning and Checkpointing

- **Checkpointing:** Introduce checkpoints at intervals that reference all known transactions up to that point. These checkpoints act like additional "mini-genesis" transactions that help reinforce the structure and traceability back to the original genesis.

- **Pruning Rules:** Define rules that periodically remove old, validated transactions from the active DAG but maintain a reference chain that always links back to the genesis.

#### Implementation Considerations
- **Security:** Tip selection algorithms need to be resistant to attacks like double-spending by making it hard for malicious actors to force the selection of certain transactions.

- **Performance:** Algorithms like MCMC and RWMC are optimized to balance security and performance, ensuring that the DAG remains acyclic without overly complex operations.

- **Scalability:** DAG-based systems are often designed to handle high transaction volumes, so the algorithms used must support scalability without compromising the acyclic structure.

