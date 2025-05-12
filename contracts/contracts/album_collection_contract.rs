// Album Collection Validation Contract
pub struct CryptoDiamondAlbum {
    pub album_id: String,
    pub required_diamonds: Vec<String>,
}

impl CryptoDiamondAlbum {
    pub fn validate_collection(&self, owned_diamonds: Vec<String>) -> bool {
        self.required_diamonds.iter().all(|diamond| owned_diamonds.contains(diamond))
    }

    pub fn check_status(&self, owned_diamonds: Vec<String>) -> String {
        if self.validate_collection(owned_diamonds) {
            format!("Album {} successfully completed!", self.album_id)
        } else {
            format!("Album {} is missing required CryptoDiamonds.", self.album_id)
        }
    }
}
