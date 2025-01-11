use vara_sails::prelude::*;

// Helper function to validate project details
pub fn validate_project(name: &str, description: &str, funding_goal: u128) -> Result<(), String> {
    if name.is_empty() || description.is_empty() {
        return Err("Project name or description cannot be empty.".to_string());
    }
    if funding_goal == 0 {
        return Err("Funding goal must be greater than zero.".to_string());
    }
    Ok(())
}

// Helper function to calculate rewards for contributors
pub fn calculate_reward(amount_contributed: u128, total_goal: u128, reward_multiplier: u128) -> u128 {
    (amount_contributed * reward_multiplier) / total_goal
}
