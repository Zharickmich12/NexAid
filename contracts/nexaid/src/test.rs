#![cfg(test)]
use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env};

fn setup() -> (Env, NexAidClient<'static>, Address, Address) {
    let env = Env::default();
    env.mock_all_auths();
    let id = env.register(NexAid, ());
    let client = NexAidClient::new(&env, &id);
    let admin = Address::generate(&env);
    let token = Address::generate(&env);
    (env, client, admin, token)
}

#[test]
fn initialize_works() {
    let (_env, client, admin, token) = setup();
    client.initialize(&admin, &token, &1000u64);
}

#[test]
#[should_panic]
fn initialize_twice_fails() {
    let (_env, client, admin, token) = setup();
    client.initialize(&admin, &token, &1000u64);
    client.initialize(&admin, &token, &1000u64);
}