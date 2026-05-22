#![cfg(test)]
use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env, Symbol};

#[test]
fn test_voting_flow() {
    let env = Env::default();
    let contract_id = env.register_contract(None, ImmutableVoting);
    let client = ImmutableVotingClient::new(&env, &contract_id);

    // Setup Admin
    let admin = Address::generate(&env);
    client.initialize(&admin);

    // Setup Candidates
    client.add_candidate(&Symbol::short("Alice"));
    client.add_candidate(&Symbol::short("Bob"));

    // Voters
    let voter1 = Address::generate(&env);
    let voter2 = Address::generate(&env);

    // Casting votes
    client.vote(&voter1, &0); // Voter 1 votes for Alice
    client.vote(&voter2, &1); // Voter 2 votes for Bob

    // Check results
    let results = client.get_results();
    assert_eq!(results.get(0).unwrap().vote_count, 1);
    assert_eq!(results.get(1).unwrap().vote_count, 1);
}

#[test]
#[should_panic(expected = "Voter has already cast their vote!")]
fn test_double_voting_prevention() {
    let env = Env::default();
    let contract_id = env.register_contract(None, ImmutableVoting);
    let client = ImmutableVotingClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    client.initialize(&admin);
    client.add_candidate(&Symbol::short("Alice"));

    let voter = Address::generate(&env);
    
    // Attempt to vote twice
    client.vote(&voter, &0);
    client.vote(&voter, &0); // This should trigger panic
}