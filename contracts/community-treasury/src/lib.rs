#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, token, Address, Env, Symbol};

#[contracttype]
pub enum DataKey {
    Admin,
    Token,
}

#[contract]
pub struct CommunityTreasury;

#[contractimpl]
impl CommunityTreasury {
    /// Initialize the treasury with an admin and token address.
    pub fn initialize(env: Env, admin: Address, token: Address) {
        assert!(
            !env.storage().instance().has(&DataKey::Admin),
            "already initialized"
        );
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::Token, &token);
    }

    /// Anyone can deposit funds into the treasury.
    pub fn deposit(env: Env, from: Address, amount: i128) {
        from.require_auth();
        assert!(amount > 0, "amount must be positive");
        let token: Address = env.storage().instance().get(&DataKey::Token).expect("not initialized");
        token::Client::new(&env, &token).transfer(&from, &env.current_contract_address(), &amount);
        env.events().publish(
            (Symbol::new(&env, "deposit"),),
            (from, amount),
        );
    }

    /// Only the admin can distribute funds to a recipient.
    pub fn distribute(env: Env, caller: Address, recipient: Address, amount: i128, reason: Symbol) {
        caller.require_auth();
        let admin: Address = env.storage().instance().get(&DataKey::Admin).expect("not initialized");
        assert!(admin == caller, "only admin can distribute");
        assert!(amount > 0, "amount must be positive");

        let token: Address = env.storage().instance().get(&DataKey::Token).expect("not initialized");
        token::Client::new(&env, &token).transfer(&env.current_contract_address(), &recipient, &amount);
        env.events().publish(
            (Symbol::new(&env, "distribute"), reason),
            (recipient, amount),
        );
    }

    pub fn get_balance(env: Env) -> i128 {
        let token: Address = env.storage().instance().get(&DataKey::Token).expect("not initialized");
        token::Client::new(&env, &token).balance(&env.current_contract_address())
    }

    pub fn get_admin(env: Env) -> Address {
        env.storage().instance().get(&DataKey::Admin).expect("not initialized")
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::testutils::Address as _;
    use soroban_sdk::{token, Address, Env, Symbol};

    fn setup_token(env: &Env, admin: &Address) -> (Address, token::StellarAssetClient<'_>) {
        let token_id = env.register_stellar_asset_contract_v2(admin.clone());
        let sac = token::StellarAssetClient::new(env, &token_id.address());
        (token_id.address(), sac)
    }

    #[test]
    fn test_initialize_and_get_admin() {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register(CommunityTreasury, ());
        let client = CommunityTreasuryClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        let (token_addr, _) = setup_token(&env, &admin);
        client.initialize(&admin, &token_addr);

        assert_eq!(client.get_admin(), admin);
    }

    #[test]
    fn test_deposit_and_balance() {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register(CommunityTreasury, ());
        let client = CommunityTreasuryClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        let depositor = Address::generate(&env);
        let (token_addr, sac) = setup_token(&env, &admin);

        client.initialize(&admin, &token_addr);
        sac.mint(&depositor, &1000);
        client.deposit(&depositor, &500);

        assert_eq!(client.get_balance(), 500);
    }

    #[test]
    fn test_distribute() {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register(CommunityTreasury, ());
        let client = CommunityTreasuryClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        let depositor = Address::generate(&env);
        let recipient = Address::generate(&env);
        let (token_addr, sac) = setup_token(&env, &admin);

        client.initialize(&admin, &token_addr);
        sac.mint(&depositor, &1000);
        client.deposit(&depositor, &1000);

        let reason = Symbol::new(&env, "reward");
        client.distribute(&admin, &recipient, &300, &reason);

        assert_eq!(client.get_balance(), 700);
        let token_client = token::Client::new(&env, &token_addr);
        assert_eq!(token_client.balance(&recipient), 300);
    }

    #[test]
    #[should_panic(expected = "only admin can distribute")]
    fn test_non_admin_cannot_distribute() {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register(CommunityTreasury, ());
        let client = CommunityTreasuryClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        let attacker = Address::generate(&env);
        let recipient = Address::generate(&env);
        let (token_addr, sac) = setup_token(&env, &admin);

        client.initialize(&admin, &token_addr);
        sac.mint(&attacker, &500);
        client.deposit(&attacker, &500);

        let reason = Symbol::new(&env, "steal");
        client.distribute(&attacker, &recipient, &500, &reason);
    }

    #[test]
    #[should_panic(expected = "already initialized")]
    fn test_cannot_reinitialize() {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register(CommunityTreasury, ());
        let client = CommunityTreasuryClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        let (token_addr, _) = setup_token(&env, &admin);
        client.initialize(&admin, &token_addr);
        client.initialize(&admin, &token_addr);
    }
}
