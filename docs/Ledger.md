## DAG Ledger Structure:

- Transaction 1 (A -> B) is recorded and validated independently.
- Transaction 2 (B -> C) is another independent record that may confirm or
   validate a prior transaction.
- Transaction 3 (C -> A) also gets recorded, and it confirms the previous
   transactions.

### Example:
Transaction 1: Wallet A -> Wallet B
Transaction 2: Wallet B -> Wallet C
Transaction 3: Wallet C -> Wallet A

 In a DAG ledger, each transaction is a node, and the edges represent
validations or approvals. For example, **Transaction 3** (C -> A) may
validate **Transaction 1** (A -> B) and **Transaction 2** (B -> C), but not
in a cyclic manner—each transaction points to earlier transactions without
creating a loop back to itself.

How It Remains Acyclic:

**Sequential Validation:** Each transaction validates previous transactions
without forming a closed loop.

**Forward Movement:** In the ledger, the sequence moves forward in time and
dependencies—no transaction points back to itself or creates a closed cycle
in the validation structure.

 This is why, even though tokens can circulate among wallets in any pattern,
the underlying transaction confirmations and dependencies in the ledger form
a DAG without cycles. The key is to differentiate between **the flow of
assets** and the **structural dependencies of the transactions themselves**.





Reference: https://en.wikipedia.org/wiki/Directed_acyclic_graph


// * Node:  Each transaction is a node. While tokens can circulate among
// *       accounts in any pattern (including returning to the origin), the
// *       transaction confirmations in a DAG structure always move forward
// *       without creating cycles.
//
// * Edges:  Edges represent validations or approvals. Transaction dependencies
// *        do not create cycles, meaning a new transaction cannot be dependent
// *        on itself, either directly or indirectly.
//
// * Graphs:  The graph represents how transactions are connected and validated
// *         rather than direct token exchanges between accounts.