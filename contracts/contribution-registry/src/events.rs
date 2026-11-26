use soroban_sdk::{Address, Env, Symbol};

pub fn emit_contribution_recorded(env: &Env, contributor: &Address, task_id: u64, reward: i128) {
    env.events().publish(
        (Symbol::new(env, "contribution_recorded"), task_id),
        (contributor.clone(), reward),
    );
}

pub fn emit_milestone_reached(env: &Env, contributor: &Address, count: u32) {
    env.events().publish(
        (Symbol::new(env, "milestone_reached"),),
        (contributor.clone(), count),
    );
}
