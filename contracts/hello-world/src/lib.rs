#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, Symbol, String, Address, log};

#[contracttype]
#[derive(Clone)]
pub struct Project {
    pub owner: Address,
    pub title: String,
    pub description: String,
    pub goal_amount: u64,
    pub current_amount: u64,
    pub is_funded: bool,
}

#[contracttype]
pub enum ProjectBook {
    Project(u64),
}

const PROJECT_COUNT: Symbol = symbol_short!("P_COUNT");

#[contract]
pub struct CrowdFundContract;

#[contractimpl]
impl CrowdFundContract {
    pub fn create_project(env: Env, owner: Address, title: String, description: String, goal_amount: u64) -> u64 {
        let mut count: u64 = env.storage().instance().get(&PROJECT_COUNT).unwrap_or(0);
        count += 1;

        let project = Project {
            owner,
            title,
            description,
            goal_amount,
            current_amount: 0,
            is_funded: false,
        };

        env.storage().instance().set(&ProjectBook::Project(count), &project);
        env.storage().instance().set(&PROJECT_COUNT, &count);

        log!(&env, "Project ID {} created!", count);
        count
    }

    pub fn fund_project(env: Env, project_id: u64, amount: u64) {
        let mut project: Project = env
            .storage()
            .instance()
            .get(&ProjectBook::Project(project_id))
            .expect("Project not found");

        project.current_amount += amount;

        if project.current_amount >= project.goal_amount {
            project.is_funded = true;
        }

        env.storage().instance().set(&ProjectBook::Project(project_id), &project);

        log!(&env, "Project ID {} funded with {}!", project_id, amount);
    }

    pub fn view_project(env: Env, project_id: u64) -> Project {
        env.storage().instance().get(&ProjectBook::Project(project_id)).expect("Project not found")
    }
}