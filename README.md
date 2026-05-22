# Title
Decentralized E-Voting System (Soroban-Vote)

# Description
Traditional and centralized online voting systems often suffer from a severe "trust deficit." In centralized Web2 applications, databases are managed by a single authority, making them vulnerable to internal manipulation (such as database administrators altering vote counts), external hacking, and lack of verifiable proof for voters. People are forced to blindly trust that the organizers will count their votes fairly.

This project solves this fundamental issue by introducing a Decentralized E-Voting System built on the Stellar network using Soroban SDK. By leveraging blockchain technology, the entire voting logic is executed through an open-source, immutable smart contract. Once a vote is cast, it is permanent and cannot be modified, deleted, or stuffed by anyone—including the system creators or admins. This shifts the trust from human authorities to cryptographic proof and mathematics.

# Features
- **Tamper-Proof Result Ledger:** All candidates and their current vote counts are stored directly on the blockchain instance storage, ensuring maximum data integrity against any unauthorized database overwrites.
- **Strict One-Vote-Per-Wallet Enforcement:** Utilizing a persistent blockchain `Map`, the contract registers the address of each voter instantly upon submission. If a user attempts to vote a second time, the transaction is automatically rejected at the protocol level.
- **Role-Based Candidate Management:** Only the designated election Organizer (Admin) has the cryptographic authority (`require_auth`) to initialize the election and add verified candidates to the ballot.
- **Immutable Election Clock / Gate:** The Admin can trigger a hard-close on the election portal. Once closed, the voting status changes to `false` permanently on-chain, preventing any late "ghost votes" from being slipped into the ballot box.
- **Public Auditability:** The `get_results` function is entirely public, allowing voters, independent auditors, or the media to query the blockchain ledger and independently verify the election tally in real-time.

# Contract
Contract link:

Contract's screenshot:

# Future scopes
- **Voter Privacy with Zero-Knowledge Proofs (ZKP):** Upgrading the contract to allow anonymous voting, where the system can verify a voter is eligible and hasn't voted yet without revealing *which* wallet address cast *which* specific ballot.
- **Token-Weighted Voting (Governance DAO):** Integrating custom Stellar assets (SEP-41 tokens) to allow organizations to weigh votes based on the number of governance tokens a participant holds.
- **Quadratic Voting Implementation:** A more democratic voting mechanism where voters can cast multiple votes for a single candidate, but the cost of doing so increases quadratically (1 vote = 1 credit, 2 votes = 4 credits, 3 votes = 9 credits) to protect minority opinions.

# Profile
- **Name:** Tiến Chung
- **Skills:**