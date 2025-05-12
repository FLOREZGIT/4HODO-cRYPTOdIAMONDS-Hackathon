// Rewards Distribution Contract
pub struct RewardPool {
    pub total_rewards_paid: f64,
    pub eligible_users: u32,
}

impl RewardPool {
    pub fn distribute_rewards(&self) -> String {
        format!("Distributed {} USD among {} eligible users", self.total_rewards_paid, self.eligible_users)
    }
}
