# Gas Estimation

Estimated gas costs for common contract operations on Stellar Testnet.

| Operation | Contract | Estimated Gas |
|---|---|---|
| create_bounty | BountyEscrow | ~200,000 |
| assign_contributor | BountyEscrow | ~100,000 |
| submit_work | BountyEscrow | ~120,000 |
| approve_work | BountyEscrow | ~250,000 |
| refund_bounty | BountyEscrow | ~200,000 |
| deposit | CommunityTreasury | ~150,000 |
| distribute | CommunityTreasury | ~200,000 |
| record_contribution | ContributionRegistry | ~130,000 |
| get_all_contributions (100 entries) | ContributionRegistry | ~500,000 |

These are approximate values. Actual gas depends on current network conditions and storage state.

## Tips
- Approve + record happens in one transaction (cross-contract call)
- Large contribution histories (>50 entries) may exceed budget on `get_all_contributions`
- Use `get_contribution` with specific indices for cheaper reads
