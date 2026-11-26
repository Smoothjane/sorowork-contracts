# Security Policy

## Reporting Vulnerabilities

If you discover a security vulnerability in the SoroWork contracts, please report it responsibly.

**Do not** open a public GitHub issue for security vulnerabilities.

Email security concerns to the maintainers with:
- Description of the vulnerability
- Steps to reproduce
- Potential impact assessment

## Security Considerations

### Authorization
- All state-changing functions require `require_auth()` on the relevant party
- The ContributionRegistry only accepts writes from the initialized escrow contract
- Treasury distributions are admin-only

### State Machine Guards
- Bounty state transitions are strictly enforced (Open → Assigned → Submitted → Approved)
- Double-approval and double-refund are prevented by status checks
- Re-initialization of treasury and registry is blocked

### Fund Safety
- Escrow funds are held by the contract, not any individual
- Refunds are only possible before work is submitted
- Token transfers use the standard Soroban token interface

## Audit Status

These contracts have not yet undergone a formal security audit. Use on testnet only.
