# ADR-002: Escrow-First Payment Flow

## Status
Accepted

## Context
Workers need assurance they'll be paid for completed community work. Communities need assurance they only pay for satisfactory work.

## Decision
Payments follow an escrow-first flow:
1. Community deposits USDC into the contract before any work begins
2. Worker completes the task and submits evidence (photos, video)
3. Community reviews evidence and approves
4. Contract automatically releases USDC to the worker

## Rationale
- Workers see locked funds before starting work, building trust
- Communities maintain approval authority before funds release
- The contract holds funds — neither party can unilaterally claim them
- Refunds are possible only before work is submitted

## Consequences
- Funds are locked for the duration of the task
- Disputes require off-chain resolution (future: add arbitration)
- Communities must have USDC ready before posting tasks
