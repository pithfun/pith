use borsh::{BorshDeserialize, BorshSerialize};
use shank::ShankAccount;

// Market is an account that tracks the current state of the market.
#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, ShankAccount)]
pub struct Market {
    // The proof PDAs bump.
    pub bump: u8,
    // The transaction ID used to keep track of the client state.
    pub tx_id: String,
    // A none-unique string used to identify a market.
    pub title: String,
}
