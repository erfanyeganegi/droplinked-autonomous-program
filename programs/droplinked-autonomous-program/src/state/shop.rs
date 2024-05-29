use anchor_lang::prelude::*;

#[account]
pub struct Shop {
    pub owner: Pubkey,
    pub metadata: String,
    pub created_at: i64,
    pub updated_at: i64,
    pub is_active: bool,
    pub is_verified: bool,
}
