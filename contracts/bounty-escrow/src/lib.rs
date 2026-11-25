#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype, token, Address, Bytes, Env, Symbol,
};

#[contracttype]
#[derive(Clone, PartialEq, Debug)]
pub enum BountyStatus {
    Open,
    Assigned,
    Submitted,
    Approved,
    Cancelled,
}

#[contracttype]
#[derive(Clone)]
pub struct BountyInfo {
    pub creator: Address,
    pub amount: i128,
    pub token: Address,
    pub contributor: Option<Address>,
    pub status: BountyStatus,
    pub evidence_hash: Option<Bytes>,
}

#[contracttype]
pub enum DataKey {
    BountyInfo(u64),
    TaskCounter,
}

fn get_task_counter(env: &Env) -> u64 {
    env.storage().instance().get(&DataKey::TaskCounter).unwrap_or(0u64)
}

fn bump_task_counter(env: &Env) -> u64 {
    let count = get_task_counter(env) + 1;
    env.storage().instance().set(&DataKey::TaskCounter, &count);
    count
}

fn get_bounty_info(env: &Env, task_id: u64) -> BountyInfo {
    env.storage()
        .persistent()
        .get(&DataKey::BountyInfo(task_id))
        .expect("bounty not found")
}

fn save_bounty_info(env: &Env, task_id: u64, info: &BountyInfo) {
    env.storage()
        .persistent()
        .set(&DataKey::BountyInfo(task_id), info);
}

#[contract]
pub struct BountyEscrow;

#[contractimpl]
impl BountyEscrow {
    /// Create a new bounty. Transfers `amount` of `token` from `creator` to the contract.
    /// Returns the new task_id.
    pub fn create_bounty(env: Env, creator: Address, token: Address, amount: i128) -> u64 {
        creator.require_auth();
        assert!(amount > 0, "amount must be positive");

        let token_client = token::Client::new(&env, &token);
        token_client.transfer(&creator, &env.current_contract_address(), &amount);

        let task_id = bump_task_counter(&env);
        let info = BountyInfo {
            creator,
            amount,
            token,
            contributor: None,
            status: BountyStatus::Open,
            evidence_hash: None,
        };
        save_bounty_info(&env, task_id, &info);
        env.events().publish(
            (Symbol::new(&env, "create_bounty"), task_id),
            info.amount,
        );
        task_id
    }

    /// Assign a contributor to the bounty. Only the creator can call this.
    pub fn assign_contributor(env: Env, task_id: u64, contributor: Address, caller: Address) {
        caller.require_auth();
        let mut info = get_bounty_info(&env, task_id);
        assert!(info.creator == caller, "only creator can assign");
        assert!(info.status == BountyStatus::Open, "bounty must be Open");

        info.contributor = Some(contributor.clone());
        info.status = BountyStatus::Assigned;
        save_bounty_info(&env, task_id, &info);
        env.events().publish(
            (Symbol::new(&env, "assign_contributor"), task_id),
            contributor,
        );
    }

    /// Submit work for a bounty. Only the assigned contributor can call this.
    pub fn submit_work(env: Env, task_id: u64, contributor: Address, evidence_hash: Bytes) {
        contributor.require_auth();
        let mut info = get_bounty_info(&env, task_id);
        assert!(info.status == BountyStatus::Assigned, "bounty must be Assigned");
        let assigned = info.contributor.clone().expect("no contributor assigned");
        assert!(assigned == contributor, "only assigned contributor can submit");

        info.evidence_hash = Some(evidence_hash);
        info.status = BountyStatus::Submitted;
        save_bounty_info(&env, task_id, &info);
        env.events().publish(
            (Symbol::new(&env, "submit_work"), task_id),
            (),
        );
    }

    /// Approve submitted work. Only the creator can call this.
    /// Transfers the bounty amount to the contributor.
    pub fn approve_work(env: Env, task_id: u64, caller: Address) {
        caller.require_auth();
        let mut info = get_bounty_info(&env, task_id);
        assert!(info.creator == caller, "only creator can approve");
        assert!(info.status == BountyStatus::Submitted, "bounty must be Submitted");

        let contributor = info.contributor.clone().expect("no contributor");
        let token_client = token::Client::new(&env, &info.token);
        token_client.transfer(&env.current_contract_address(), &contributor, &info.amount);

        info.status = BountyStatus::Approved;
        save_bounty_info(&env, task_id, &info);
        env.events().publish(
            (Symbol::new(&env, "approve_work"), task_id),
            info.amount,
        );
    }

    /// Refund the bounty to the creator. Only valid when status is Open or Assigned.
    pub fn refund_bounty(env: Env, task_id: u64, caller: Address) {
        caller.require_auth();
        let mut info = get_bounty_info(&env, task_id);
        assert!(info.creator == caller, "only creator can refund");
        assert!(
            info.status == BountyStatus::Open || info.status == BountyStatus::Assigned,
            "can only refund Open or Assigned bounties"
        );

        let token_client = token::Client::new(&env, &info.token);
        token_client.transfer(&env.current_contract_address(), &info.creator, &info.amount);

        info.status = BountyStatus::Cancelled;
        save_bounty_info(&env, task_id, &info);
        env.events().publish(
            (Symbol::new(&env, "refund_bounty"), task_id),
            info.amount,
        );
    }

    pub fn get_bounty(env: Env, task_id: u64) -> BountyInfo {
        get_bounty_info(&env, task_id)
    }

    pub fn get_task_count(env: Env) -> u64 {
        get_task_counter(&env)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::testutils::{Address as _, AuthorizedFunction, AuthorizedInvocation};
    use soroban_sdk::{token, vec, Address, Bytes, Env, IntoVal};

    fn create_token(env: &Env, admin: &Address) -> (Address, token::StellarAssetClient<'_>) {
        let token_id = env.register_stellar_asset_contract_v2(admin.clone());
        let sac = token::StellarAssetClient::new(env, &token_id.address());
        (token_id.address(), sac)
    }

    #[test]
    fn test_create_and_get_bounty() {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register(BountyEscrow, ());
        let client = BountyEscrowClient::new(&env, &contract_id);

        let creator = Address::generate(&env);
        let (token_addr, sac) = create_token(&env, &creator);
        sac.mint(&creator, &1000);

        let task_id = client.create_bounty(&creator, &token_addr, &500);
        assert_eq!(task_id, 1);

        let bounty = client.get_bounty(&task_id);
        assert_eq!(bounty.creator, creator);
        assert_eq!(bounty.amount, 500);
        assert_eq!(bounty.status, BountyStatus::Open);
        assert_eq!(client.get_task_count(), 1);
    }

    #[test]
    fn test_full_bounty_lifecycle() {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register(BountyEscrow, ());
        let client = BountyEscrowClient::new(&env, &contract_id);

        let creator = Address::generate(&env);
        let contributor = Address::generate(&env);
        let (token_addr, sac) = create_token(&env, &creator);
        sac.mint(&creator, &1000);

        let task_id = client.create_bounty(&creator, &token_addr, &500);
        client.assign_contributor(&task_id, &contributor, &creator);

        let bounty = client.get_bounty(&task_id);
        assert_eq!(bounty.status, BountyStatus::Assigned);

        let evidence = Bytes::from_slice(&env, b"QmHash123");
        client.submit_work(&task_id, &contributor, &evidence);
        assert_eq!(client.get_bounty(&task_id).status, BountyStatus::Submitted);

        client.approve_work(&task_id, &creator);
        assert_eq!(client.get_bounty(&task_id).status, BountyStatus::Approved);

        // Contributor should now hold the bounty amount
        let token_client = token::Client::new(&env, &token_addr);
        assert_eq!(token_client.balance(&contributor), 500);
    }

    #[test]
    fn test_refund_open_bounty() {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register(BountyEscrow, ());
        let client = BountyEscrowClient::new(&env, &contract_id);

        let creator = Address::generate(&env);
        let (token_addr, sac) = create_token(&env, &creator);
        sac.mint(&creator, &1000);

        let task_id = client.create_bounty(&creator, &token_addr, &500);
        client.refund_bounty(&task_id, &creator);

        assert_eq!(client.get_bounty(&task_id).status, BountyStatus::Cancelled);

        let token_client = token::Client::new(&env, &token_addr);
        assert_eq!(token_client.balance(&creator), 1000); // full refund
    }

    #[test]
    #[should_panic(expected = "only creator can assign")]
    fn test_non_creator_cannot_assign() {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register(BountyEscrow, ());
        let client = BountyEscrowClient::new(&env, &contract_id);

        let creator = Address::generate(&env);
        let attacker = Address::generate(&env);
        let (token_addr, sac) = create_token(&env, &creator);
        sac.mint(&creator, &1000);

        let task_id = client.create_bounty(&creator, &token_addr, &500);
        client.assign_contributor(&task_id, &attacker, &attacker);
    }

    #[test]
    #[should_panic(expected = "can only refund Open or Assigned bounties")]
    fn test_cannot_refund_submitted_bounty() {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register(BountyEscrow, ());
        let client = BountyEscrowClient::new(&env, &contract_id);

        let creator = Address::generate(&env);
        let contributor = Address::generate(&env);
        let (token_addr, sac) = create_token(&env, &creator);
        sac.mint(&creator, &1000);

        let task_id = client.create_bounty(&creator, &token_addr, &500);
        client.assign_contributor(&task_id, &contributor, &creator);
        let evidence = Bytes::from_slice(&env, b"QmHash456");
        client.submit_work(&task_id, &contributor, &evidence);
        client.refund_bounty(&task_id, &creator);
    }

    #[test]
    fn test_multiple_bounties() {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register(BountyEscrow, ());
        let client = BountyEscrowClient::new(&env, &contract_id);

        let creator = Address::generate(&env);
        let (token_addr, sac) = create_token(&env, &creator);
        sac.mint(&creator, &3000);

        let id1 = client.create_bounty(&creator, &token_addr, &100);
        let id2 = client.create_bounty(&creator, &token_addr, &200);
        let id3 = client.create_bounty(&creator, &token_addr, &300);

        assert_eq!(id1, 1);
        assert_eq!(id2, 2);
        assert_eq!(id3, 3);
        assert_eq!(client.get_task_count(), 3);
    }
}
