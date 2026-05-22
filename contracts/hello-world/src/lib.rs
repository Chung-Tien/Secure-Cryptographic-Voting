#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Symbol, Vec, Map};

// Candidate structure representing each voting option
#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct Candidate {
    pub name: Symbol,
    pub vote_count: u32,
}

// Data keys for contract storage to manage election states and data
#[contracttype]
pub enum DataKey {
    Admin,           // The administrator's address who initializes the election
    Candidates,      // A list of candidates running for the election
    VotedStatus,     // A map tracking which addresses have already cast their vote
    IsVotingOpen,    // Boolean flag to track the election status (Active/Closed)
}

#[contract]
pub struct ImmutableVoting;

#[contractimpl]
impl ImmutableVoting {
    // Initialize the election: Set admin and open the voting portal
    pub fn initialize(env: Env, admin: Address) {
        if env.storage().instance().has(&DataKey::Admin) {
            panic!("Election already initialized!");
        }
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::IsVotingOpen, &true);
        env.storage().instance().set(&DataKey::Candidates, &Vec::<Candidate>::new(&env));
    }

    // Add a candidate: Only the admin can perform this
    pub fn add_candidate(env: Env, name: Symbol) {
        let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        admin.require_auth(); // Secure: verify admin signature

        let mut candidates: Vec<Candidate> = env.storage().instance().get(&DataKey::Candidates).unwrap();
        
        let new_candidate = Candidate {
            name,
            vote_count: 0,
        };

        candidates.push_back(new_candidate);
        env.storage().instance().set(&DataKey::Candidates, &candidates);
    }

    // Vote for a candidate: Only one vote per wallet allowed
    pub fn vote(env: Env, voter: Address, candidate_index: u32) {
        voter.require_auth(); // Ensure the voter is who they claim to be

        // Check if voting portal is still open
        let is_open: bool = env.storage().instance().get(&DataKey::IsVotingOpen).unwrap_or(false);
        if !is_open { panic!("Voting is closed!"); }

        // Prevent double voting using VotedStatus map
        let mut voted_status: Map<Address, bool> = env.storage().persistent()
            .get(&DataKey::VotedStatus)
            .unwrap_or(Map::new(&env));

        if voted_status.contains_key(voter.clone()) {
            panic!("Voter has already cast their vote!");
        }

        // Update vote count
        let mut candidates: Vec<Candidate> = env.storage().instance().get(&DataKey::Candidates).unwrap();
        let mut candidate = candidates.get(candidate_index).expect("Candidate not found!");
        
        candidate.vote_count += 1;
        candidates.set(candidate_index, candidate);
        
        // Save state changes
        voted_status.set(voter.clone(), true);
        env.storage().persistent().set(&DataKey::VotedStatus, &voted_status);
        env.storage().instance().set(&DataKey::Candidates, &candidates);
    }

    // Publicly query the election results
    pub fn get_results(env: Env) -> Vec<Candidate> {
        env.storage().instance().get(&DataKey::Candidates).unwrap_or(Vec::new(&env))
    }
}