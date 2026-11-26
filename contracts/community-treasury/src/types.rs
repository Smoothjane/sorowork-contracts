use soroban_sdk::{contracttype, Address, Symbol};

/// Record of a deposit into the community treasury.
#[contracttype]
#[derive(Clone)]
pub struct DepositRecord {
    pub from: Address,
    pub amount: i128,
    pub timestamp: u64,
}

/// Metadata for a community treasury.
#[contracttype]
#[derive(Clone)]
pub struct TreasuryMetadata {
    pub name: Symbol,
    pub community_id: Symbol,
}

/// Summary of treasury activity.
#[contracttype]
#[derive(Clone)]
pub struct TreasurySummary {
    pub total_deposited: i128,
    pub total_distributed: i128,
    pub current_balance: i128,
    pub deposit_count: u32,
    pub distribution_count: u32,
}
