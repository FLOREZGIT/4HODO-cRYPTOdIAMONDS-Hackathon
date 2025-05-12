// Royalty Fee Distribution Contract
pub struct RoyaltyFee {
    pub total_sales: f64,
    pub fee_percentage: f64,
}

impl RoyaltyFee {
    pub fn calculate_royalty(&self) -> f64 {
        self.total_sales * (self.fee_percentage / 100.0)
    }

    pub fn distribute_royalty(&self) -> String {
        let royalty_amount = self.calculate_royalty();
        format!("Distributed {} USD as royalty fee from total sales of {} USD", 
                royalty_amount, self.total_sales)
    }
}
