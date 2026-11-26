# ADR-001: Three-Contract Architecture

## Status
Accepted

## Context
SoroWork needs to handle community work payments, shared funding, and work records on Stellar/Soroban. We considered a single monolithic contract vs. separate contracts for each concern.

## Decision
We use three separate contracts:
1. **BountyEscrow** — Handles the lifecycle of individual work tasks (create, assign, submit, approve, refund)
2. **CommunityTreasury** — Manages pooled community funds with admin-controlled distribution
3. **ContributionRegistry** — Maintains an append-only record of verified work contributions

## Rationale
- **Separation of concerns**: Each contract has a single responsibility, making audits easier
- **Independent upgradability**: One contract can be upgraded without affecting others
- **Access control**: The registry only accepts writes from the escrow contract, enforcing the work-then-record flow
- **Storage efficiency**: Each contract manages its own storage, avoiding bloated single-contract state

## Consequences
- Cross-contract calls introduce complexity and potential failure points
- Three contracts must be deployed and initialized in the correct order
- Registry initialization requires the escrow contract address
