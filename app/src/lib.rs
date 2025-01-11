// Import necessary modules and crates
use vara_sails::prelude::*;

// Define the smart contract
#[contract]
pub struct DeSciFunding {
    // Mapping of project IDs to funding data
    projects: Map<u64, Project>,
    // Total number of projects
    project_count: u64,
}

// Structure to represent a research project
#[derive(Serialize, Deserialize, Clone)]
pub struct Project {
    id: u64,
    name: String,
    description: String,
    funding_goal: u128,
    funds_raised: u128,
    contributors: Vec<Contributor>,
    owner: Address,
}

// Structure to represent a contributor
#[derive(Serialize, Deserialize, Clone)]
pub struct Contributor {
    address: Address,
    amount: u128,
}

impl DeSciFunding {
    #[init]
    pub fn new() -> Self {
        Self {
            projects: Map::new(),
            project_count: 0,
        }
    }

    // Function to create a new research project
    pub fn create_project(&mut self, name: String, description: String, funding_goal: u128) {
        let id = self.project_count + 1;
        let project = Project {
            id,
            name,
            description,
            funding_goal,
            funds_raised: 0,
            contributors: Vec::new(),
            owner: msg::sender(),
        };
        self.projects.insert(id, project);
        self.project_count = id;
    }

    // Function to fund a research project
    pub fn fund_project(&mut self, project_id: u64, amount: u128) {
        let sender = msg::sender();
        let mut project = self.projects.get(&project_id).expect("Project not found");

        project.funds_raised += amount;
        project.contributors.push(Contributor {
            address: sender,
            amount,
        });

        self.projects.insert(project_id, project);
    }

    // Function to view a project
    pub fn get_project(&self, project_id: u64) -> Project {
        self.projects.get(&project_id).expect("Project not found")
    }

    // Function to distribute rewards (if any)
    pub fn distribute_rewards(&mut self, project_id: u64, reward_token: Address) {
        let project = self.projects.get(&project_id).expect("Project not found");
        assert!(project.funding_goal <= project.funds_raised, "Funding goal not met");

        for contributor in project.contributors.iter() {
            let reward_amount = (contributor.amount as u128 * 1000) / project.funding_goal;
            // Mock function for token transfer
            msg::send(contributor.address, reward_token, reward_amount);
        }
    }
}
