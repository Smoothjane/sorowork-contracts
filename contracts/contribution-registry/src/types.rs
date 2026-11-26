use soroban_sdk::{contracttype, Symbol};

/// Categories of community work for contribution classification.
#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub enum WorkCategory {
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

/// Worker reputation tier based on contribution count and quality.
#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub enum ReputationTier {
    Newcomer,
    Reliable,
    Experienced,
    Expert,
    Master,
}

/// Summary of a contributor's work history.
#[contracttype]
#[derive(Clone)]
pub struct ContributorSummary {
    pub total_contributions: u32,
    pub total_earned: i128,
    pub reputation_tier: ReputationTier,
    pub primary_category: WorkCategory,
}
