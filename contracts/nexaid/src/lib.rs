#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env};

#[contracttype]
pub enum DataKey {
    Admin,
    Token,
    Expiry,
    Merchant(Address),
    Balance(Address),
}

#[contract]
pub struct NexAid;

#[contractimpl]
impl NexAid {
    // Guarda admin, token y vencimiento. Solo puede llamarse una vez.
    pub fn initialize(env: Env, admin: Address, token: Address, expiry: u64) {
        if env.storage().instance().has(&DataKey::Admin) {
            panic!("already initialized");
        }
        admin.require_auth();
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::Token, &token);
        env.storage().instance().set(&DataKey::Expiry, &expiry);
    }

    // El donante envía tokens al contrato. Firma: donor.
    pub fn donate(env: Env, donor: Address, amount: i128) {
        todo!()
    }

    // Autoriza un comercio. Firma: admin.
    pub fn add_merchant(env: Env, merchant: Address) {
        let admin: Address = env
            .storage()
            .instance()
            .get(&DataKey::Admin)
            .expect("not initialized");
        admin.require_auth();
        env.storage()
            .persistent()
            .set(&DataKey::Merchant(merchant), &true);
    }

    // Consulta si un comercio está autorizado.
    pub fn is_merchant(env: Env, merchant: Address) -> bool {
        env.storage()
            .persistent()
            .get(&DataKey::Merchant(merchant))
            .unwrap_or(false)
    }

    // Asigna ayuda a un beneficiario. Firma: admin.
    pub fn assign_aid(env: Env, beneficiary: Address, amount: i128) {
        todo!()
    }

    // El beneficiario paga a un comercio autorizado. Firma: beneficiary.
    pub fn pay(env: Env, beneficiary: Address, merchant: Address, amount: i128) {
        todo!()
    }

    // Si venció, el saldo vuelve al fondo. Puede llamarla cualquiera.
    pub fn reclaim_expired(env: Env, beneficiary: Address) {
        todo!()
    }
}

mod test;