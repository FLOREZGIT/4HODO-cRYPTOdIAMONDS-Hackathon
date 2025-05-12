// Burning Contract - CryptoDiamonds
pub struct CryptoDiamondBurning {
    pub stage: u8,
    pub burn_amount: u32,
}

impl CryptoDiamondBurning {
    pub fn execute_burn(&self) -> String {
        format!("Burned {} CryptoDiamonds at Stage {}", self.burn_amount, self.stage)
    }
}
