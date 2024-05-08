use borsh::BorshDeserialize;
use num_enum::TryFromPrimitive;
use shank::ShankInstruction;

#[repr(u8)]
#[derive(Clone, TryFromPrimitive, ShankInstruction)]
#[rustfmt::skip]
pub enum PithInstruction {
    #[account(0, name = "signer", desc = "Signer", signer)]
    #[account(1, name = "market", desc = "Pith market account", writable)]
    #[account(2, name = "system_program", desc = "Solana System Program")]
    Market = 0,

    #[account(0, name = "signer", desc = "Signer", signer)]
    #[account(1, name = "market", desc = "Pith market account", writable)]
    #[account(2, name = "bid", desc = "Bid account", writable)]
    #[account(3, name = "system_program", desc = "Solana System Program")]
    Bid = 1,
}

#[repr(C)]
#[derive(Clone, Debug, BorshDeserialize)]
pub struct MarketArgs {
    pub bump: u8,
    pub id: u64,
    pub title: String,
}

#[repr(C)]
#[derive(Clone, Debug, BorshDeserialize)]
pub struct BidArgs {
    pub bump: u8,
    pub id: u64,
    pub amount: u64,
}
