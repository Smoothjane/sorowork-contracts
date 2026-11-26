#![cfg(test)]
use soroban_sdk::{token, Address, Bytes, Env};

pub fn setup_env() -> (Env, Address, Address) {
    let env = Env::default();
    env.mock_all_auths();
    let creator = Address::generate(&env);
    let worker = Address::generate(&env);
    (env, creator, worker)
}

pub fn create_test_token(env: &Env, admin: &Address) -> (Address, token::StellarAssetClient<'_>) {
    let token_id = env.register_stellar_asset_contract_v2(admin.clone());
    let sac = token::StellarAssetClient::new(env, &token_id.address());
    (token_id.address(), sac)
}

pub fn sample_evidence(env: &Env) -> Bytes {
    Bytes::from_slice(env, b"QmSampleEvidenceHash123")
}

pub fn sample_evidence_photo(env: &Env) -> Bytes {
    Bytes::from_slice(env, b"QmBeforeAfterPhotoHash456")
}
