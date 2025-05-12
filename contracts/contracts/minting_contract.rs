// Minting Contract - CryptoDiamonds
pub struct CryptoDiamondMinting {
    pub diamond_id: String,
    pub stage: u8,
    pub price: f64,
}

impl CryptoDiamondMinting {
    pub fn new(diamond_id: String, stage: u8) -> Self {
        let price = match stage {
            1 => 0.32,
            3 => 0.96,
            5 => 1.60,
            _ => 2.00,
        };
        Self { diamond_id, stage, price }
    }

    pub fn mint_diamond(&self) -> String {
        format!("Minted CryptoDiamond {} at Stage {} for {} USD", 
                self.diamond_id, self.stage, self.price)
    }
}
