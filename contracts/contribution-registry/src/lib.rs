#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Bytes, Env, Vec};

#[contracttype]
#[derive(Clone)]
pub struct ContributionEntry {
    pub task_id: u64,
    pub evidence_hash: Bytes,
    pub timestamp: u64,
    pub reward_amount: i128,
    /// Quality score assigned by the approver (0-100).
    pub quality_score: u32,
}

#[contracttype]
pub enum DataKey {
    /// Authorized caller (the bounty escrow contract address)
    AuthorizedCaller,
    /// Total contribution count for a contributor
    ContribCount(Address),
    /// Individual contribution entry: (contributor, index)
    ContribEntry(Address, u32),
}

/// Errors for contribution registry operations.
#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub enum ContributionError {
    AlreadyInitialized,
    NotInitialized,
    UnauthorizedCaller,
    ContributionNotFound,
    IndexOutOfBounds,
}
#[contract]
/// Append-only record of verified community work contributions.
pub struct ContributionRegistry;

#[contractimpl]
impl ContributionRegistry {
    /// Initialize with the address of the bounty escrow contract as the authorized caller.
    pub fn initialize(env: Env, authorized_caller: Address) {
        assert!(
            !env.storage().instance().has(&DataKey::AuthorizedCaller),
            "already initialized"
        );
        env.storage()
            .instance()
            .set(&DataKey::AuthorizedCaller, &authorized_caller);
    }

    /// Record a verified contribution. Only the authorized caller (bounty escrow) can call this.
    pub fn record_contribution(
        env: Env,
        authorized_caller: Address,
        contributor: Address,
        task_id: u64,
        evidence_hash: Bytes,
        reward_amount: i128,
    ) {
        authorized_caller.require_auth();
        let expected: Address = env
            .storage()
            .instance()
            .get(&DataKey::AuthorizedCaller)
            .expect("not initialized");
        assert!(expected == authorized_caller, "unauthorized caller");

        let count: u32 = env
            .storage()
            .persistent()
            .get(&DataKey::ContribCount(contributor.clone()))
            .unwrap_or(0u32);

        let entry = ContributionEntry {
            task_id,
            evidence_hash,
            timestamp: env.ledger().timestamp(),
            reward_amount,
        };

        env.storage()
            .persistent()
            .set(&DataKey::ContribEntry(contributor.clone(), count), &entry);
        env.storage()
            .persistent()
            .set(&DataKey::ContribCount(contributor), &(count + 1));
    }

    pub fn get_contribution_count(env: Env, contributor: Address) -> u32 {
        env.storage()
            .persistent()
            .get(&DataKey::ContribCount(contributor))
            .unwrap_or(0u32)
    }

    pub fn get_contribution(env: Env, contributor: Address, index: u32) -> ContributionEntry {
        env.storage()
            .persistent()
            .get(&DataKey::ContribEntry(contributor, index))
            .expect("contribution not found")
    }

    pub fn get_all_contributions(env: Env, contributor: Address) -> Vec<ContributionEntry> {
        let count: u32 = env
            .storage()
            .persistent()
            .get(&DataKey::ContribCount(contributor.clone()))
            .unwrap_or(0u32);

        let limit = if count > 100 { 100 } else { count };
        let mut entries = Vec::new(&env);
        for i in 0..limit {
            if let Some(entry) = env
                .storage()
                .persistent()
                .get(&DataKey::ContribEntry(contributor.clone(), i))
            {
                entries.push_back(entry);
            }
        }
        entries
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::testutils::Address as _;
    use soroban_sdk::{Address, Bytes, Env};

    #[test]
    fn test_record_and_get_contribution() {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register(ContributionRegistry, ());
        let client = ContributionRegistryClient::new(&env, &contract_id);

        let authorized = Address::generate(&env);
        let contributor = Address::generate(&env);

        client.initialize(&authorized);

        let hash = Bytes::from_slice(&env, b"QmHash789");
        client.record_contribution(&authorized, &contributor, &1u64, &hash, &500);

        assert_eq!(client.get_contribution_count(&contributor), 1);
        let entry = client.get_contribution(&contributor, &0u32);
        assert_eq!(entry.task_id, 1);
        assert_eq!(entry.reward_amount, 500);
    }

    #[test]
    fn test_get_all_contributions() {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register(ContributionRegistry, ());
        let client = ContributionRegistryClient::new(&env, &contract_id);

        let authorized = Address::generate(&env);
        let contributor = Address::generate(&env);
        client.initialize(&authorized);

        for i in 0u64..5 {
            let hash = Bytes::from_slice(&env, b"QmHash");
            client.record_contribution(&authorized, &contributor, &i, &hash, &(100 * (i as i128 + 1)));
        }

        let all = client.get_all_contributions(&contributor);
        assert_eq!(all.len(), 5);
        assert_eq!(all.get(0).unwrap().task_id, 0);
        assert_eq!(all.get(4).unwrap().task_id, 4);
    }

    #[test]
    #[should_panic(expected = "unauthorized caller")]
    fn test_unauthorized_cannot_record() {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register(ContributionRegistry, ());
        let client = ContributionRegistryClient::new(&env, &contract_id);

        let authorized = Address::generate(&env);
        let attacker = Address::generate(&env);
        let contributor = Address::generate(&env);
        client.initialize(&authorized);

        let hash = Bytes::from_slice(&env, b"QmHash");
        client.record_contribution(&attacker, &contributor, &1u64, &hash, &999);
    }

    #[test]
    fn test_zero_contributions_for_new_contributor() {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register(ContributionRegistry, ());
        let client = ContributionRegistryClient::new(&env, &contract_id);

        let authorized = Address::generate(&env);
        let new_contributor = Address::generate(&env);
        client.initialize(&authorized);

        assert_eq!(client.get_contribution_count(&new_contributor), 0);
        let all = client.get_all_contributions(&new_contributor);
        assert_eq!(all.len(), 0);
    }

    #[test]
    #[should_panic(expected = "already initialized")]
    fn test_cannot_reinitialize() {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register(ContributionRegistry, ());
        let client = ContributionRegistryClient::new(&env, &contract_id);

        let authorized = Address::generate(&env);
        client.initialize(&authorized);
        client.initialize(&authorized);
    }
}
