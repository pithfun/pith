use borsh::{BorshDeserialize, BorshSerialize};
use shank::ShankAccount;

// Market is an account that tracks the current state of the market.
#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, ShankAccount)]
pub struct Market {
    // The proof PDAs bump.
    pub bump: u8,
    // Transaction ID used to keep track of client state.
    pub id: u64,
    // A none-unique string used to identify a market.
    pub title: String,
}
