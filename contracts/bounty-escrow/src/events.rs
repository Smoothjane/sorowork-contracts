use soroban_sdk::{Address, Env, Symbol};

pub fn emit_bounty_created(env: &Env, task_id: u64, creator: &Address, amount: i128) {
    env.events().publish(
        (Symbol::new(env, "bounty_created"), task_id),
        (creator.clone(), amount),
    );
}

pub fn emit_worker_assigned(env: &Env, task_id: u64, worker: &Address) {
    env.events().publish(
        (Symbol::new(env, "worker_assigned"), task_id),
        worker.clone(),
    );
}

pub fn emit_work_submitted(env: &Env, task_id: u64, worker: &Address) {
    env.events().publish(
        (Symbol::new(env, "work_submitted"), task_id),
        worker.clone(),
    );
}

pub fn emit_work_approved(env: &Env, task_id: u64, amount: i128) {
    env.events().publish(
        (Symbol::new(env, "work_approved"), task_id),
        amount,
    );
}

pub fn emit_bounty_refunded(env: &Env, task_id: u64, amount: i128) {
    env.events().publish(
        (Symbol::new(env, "bounty_refunded"), task_id),
        amount,
    );
}
