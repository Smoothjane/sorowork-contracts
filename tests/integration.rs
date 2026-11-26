//! Integration tests for the full SoroWork contract suite.
//!
//! These tests exercise cross-contract interactions:
//! - BountyEscrow creating tasks and paying workers
//! - ContributionRegistry recording verified work
//! - CommunityTreasury managing pooled funds

// TODO: Integration test framework for multi-contract scenarios
// Once soroban_sdk test utilities support cross-contract testing,
// add the following test cases:
//
// 1. Full lifecycle: create → assign → submit → approve → record
// 2. Refund flow: create → assign → refund (verify funds returned)
// 3. Treasury → Escrow: deposit to treasury, distribute for task creation
// 4. Multiple workers: parallel tasks with independent lifecycles
// 5. Registry cap: verify 100-entry query limit behavior
