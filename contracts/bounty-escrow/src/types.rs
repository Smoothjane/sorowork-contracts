use soroban_sdk::{contracttype, Address, Bytes, Symbol};

/// Categories of community work tasks.
#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub enum TaskCategory {
    Cleaning,
    RoadRepair,
    Landscaping,
    Painting,
    Plumbing,
    Electrical,
    Construction,
    Maintenance,
    Volunteering,
    Other,
}

/// Location information for a community work task.
#[contracttype]
#[derive(Clone)]
pub struct TaskLocation {
    pub area_code: Symbol,
    pub description: Bytes,
}

/// Summary statistics for the escrow contract.
#[contracttype]
#[derive(Clone)]
pub struct EscrowStats {
    pub total_tasks: u64,
    pub total_locked: i128,
    pub total_paid: i128,
}
