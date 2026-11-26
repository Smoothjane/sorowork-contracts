use soroban_sdk::{Address, Env, Symbol};

pub fn emit_deposit(env: &Env, from: &Address, amount: i128) {
    env.events().publish(
        (Symbol::new(env, "treasury_deposit"),),
        (from.clone(), amount),
    );
}

pub fn emit_distribution(env: &Env, recipient: &Address, amount: i128, reason: &Symbol) {
    env.events().publish(
        (Symbol::new(env, "treasury_distribute"), reason.clone()),
        (recipient.clone(), amount),
    );
}

pub fn emit_admin_changed(env: &Env, old_admin: &Address, new_admin: &Address) {
    env.events().publish(
        (Symbol::new(env, "admin_changed"),),
        (old_admin.clone(), new_admin.clone()),
    );
}
