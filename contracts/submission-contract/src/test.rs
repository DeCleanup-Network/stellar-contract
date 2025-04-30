#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, Address, Env, Symbol, Vec, Map, BytesN, Panic,
};

#[contracttype]
#[derive(Clone)]
pub enum SubmissionStatus {
    Pending,
    Approved,
    Rejected,
}

#[contracttype]
#[derive(Clone)]
pub struct Submission {
    pub id: u64,
    pub submitter: Address,
    pub data_uri: BytesN<32>, // Assuming IPFS hash
    pub timestamp: u64,
    pub status: SubmissionStatus,
    pub approver: Option<Address>,
    pub processed_timestamp: Option<u64>,
    pub rewarded: bool,
}

#[contract]
pub struct SubmissionContract;

#[contractimpl]
impl SubmissionContract {
    pub fn init(env: Env, admin: Address) {
        // Set the admin address
        env.storage().set(&Symbol::short("admin"), &admin);
        // Initialize submission count
        env.storage().set(&Symbol::short("submission_count"), &0u64);
    }

    pub fn create_submission(env: Env, data_uri: BytesN<32>) -> u64 {
        let submitter = env.invoker();
        let timestamp = env.ledger().timestamp();

        // Retrieve and increment submission count
        let mut count: u64 = env.storage().get(&Symbol::short("submission_count")).unwrap_or(0);
        let submission_id = count;
        count += 1;
        env.storage().set(&Symbol::short("submission_count"), &count);

        // Create submission
        let submission = Submission {
            id: submission_id,
            submitter: submitter.clone(),
            data_uri,
            timestamp,
            status: SubmissionStatus::Pending,
            approver: None,
            processed_timestamp: None,
            rewarded: false,
        };

        // Store submission
        let key = Symbol::short(format!("submission_{}", submission_id).as_str());
        env.storage().set(&key, &submission);

        // Emit event
        env.events().publish(
            (Symbol::short("SubmissionCreated"), submitter),
            submission_id,
        );

        submission_id
    }

    pub fn approve_submission(env: Env, submission_id: u64) {
        let caller = env.invoker();
        let admin: Address = env.storage().get(&Symbol::short("admin")).expect("Admin not set");
        if caller != admin {
            panic!("Only admin can approve submissions");
        }

        let key = Symbol::short(format!("submission_{}", submission_id).as_str());
        let mut submission: Submission = env
            .storage()
            .get(&key)
            .expect("Submission not found");

        if submission.status == SubmissionStatus::Approved {
            panic!("Submission already approved");
        }

        submission.status = SubmissionStatus::Approved;
        submission.approver = Some(caller.clone());
        submission.processed_timestamp = Some(env.ledger().timestamp());
        submission.rewarded = true;

        env.storage().set(&key, &submission);

        // Emit event
        env.events().publish(
            (Symbol::short("SubmissionApproved"), caller),
            submission_id,
        );
    }

    pub fn reject_submission(env: Env, submission_id: u64) {
        let caller = env.invoker();
        let admin: Address = env.storage().get(&Symbol::short("admin")).expect("Admin not set");
        if caller != admin {
            panic!("Only admin can reject submissions");
        }

        let key = Symbol::short(format!("submission_{}", submission_id).as_str());
        let mut submission: Submission = env
            .storage()
            .get(&key)
            .expect("Submission not found");

        if submission.status == SubmissionStatus::Rejected {
            panic!("Submission already rejected");
        }

        submission.status = SubmissionStatus::Rejected;
        submission.approver = Some(caller.clone());
        submission.processed_timestamp = Some(env.ledger().timestamp());

        env.storage().set(&key, &submission);

        // Emit event
        env.events().publish(
            (Symbol::short("SubmissionRejected"), caller),
            submission_id,
        );
    }

    pub fn get_submission(env: Env, submission_id: u64) -> Submission {
        let key = Symbol::short(format!("submission_{}", submission_id).as_str());
        env.storage()
            .get(&key)
            .expect("Submission not found")
    }

    pub fn get_submission_count(env: Env) -> u64 {
        env.storage().get(&Symbol::short("submission_count")).unwrap_or(0)
    }
}
