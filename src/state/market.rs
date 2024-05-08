use borsh::{BorshDeserialize, BorshSerialize};
use shank::ShankAccount;
use solana_program::pubkey::Pubkey;

/// Market is the parent account that stores a tradable asset and keeps track of
/// the bids placed on the specific market via a counter.
#[repr(C)]
#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, ShankAccount)]
pub struct Market {
    /// Account discriminator
    pub discriminator: String,
    /// The market account PDA.
    pub bump: u8,
    /// The accounts authority.
    pub authority: Pubkey,
    /// The unique market ID.
    pub id: u64,
    /// The title string for a specific market.
    pub title: String,
    /// Counter keeps track of the number of bids placed on this market.
    pub counter: u16,
    /// The market account key. Useful since `getMultipleAccountsInfo` does not
    /// return a `keyedAccountInfo`.
    pub key: Pubkey,
}

impl Market {
    pub const DISCRIMINATOR: &'static str = "market";

    pub fn get_account_size(title: &String, discriminator: &String) -> usize {
        return (4 + discriminator.len()) + 1 + 32 + 8 + (4 + title.len()) + 2 + 32;
    }
}
