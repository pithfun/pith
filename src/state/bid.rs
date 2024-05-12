use borsh::{BorshDeserialize, BorshSerialize};
use shank::ShankAccount;
use solana_program::pubkey::Pubkey;

#[repr(C)]
#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, ShankAccount)]
pub struct Bid {
    /// Account discriminator.
    pub discriminator: String,
    /// The market key associated with this bid.
    pub market: Pubkey,
    /// The amount of the bid in lamports.
    pub amount: u64,
    /// The account that placed this bid.
    pub authority: Pubkey,
    /// The unique bid ID.
    pub id: u64,
}

impl Bid {
    pub const DISCRIMINATOR: &'static str = "bid";
    pub const SIZE: usize = (4 + Bid::DISCRIMINATOR.len()) + 32 + 8 + 32 + 8;
}
