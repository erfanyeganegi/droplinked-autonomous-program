use std::mem::size_of;

use anchor_lang::prelude::*;

use crate::IsState;

#[account]
pub struct Shop {
    pub owner: Pubkey,
    pub metadata: String,
    pub created_at: i64,
    pub updated_at: i64,
    pub is_active: bool,
    pub is_verified: bool,
}

pub struct ShopSpaceParams {
    metadata: String,
}

impl IsState<ShopSpaceParams> for Shop {
    const PREFIX: &'static str = "shop";

    fn space(params: ShopSpaceParams) -> usize {
        let ShopSpaceParams { metadata } = params;

        8 /* discriminator */
      + size_of::<Pubkey>() /* owner */
      + (4 + metadata.as_bytes().len()) /* metadata */
      + size_of::<i64>() /* created_at */
      + size_of::<i64>() /* updated_at */
      + size_of::<bool>() /* is_active */
      + size_of::<bool>() /* is_verified */
    }
}
