// Joker Wild Cards Management Contract
pub struct JokerWildCard {
    pub card_id: String,
    pub assigned_album: String,
}

impl JokerWildCard {
    pub fn validate_card(&self) -> String {
        format!("Joker Wild Card {} assigned to album {}", self.card_id, self.assigned_album)
    }

    pub fn mint_joker_card(card_id: String, album: String) -> Self {
        Self { card_id, assigned_album: album }
    }
}
