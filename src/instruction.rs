use borsh::BorshDeserialize;
use num_enum::TryFromPrimitive;
use shank::ShankInstruction;

#[repr(u8)]
#[derive(Clone, TryFromPrimitive, ShankInstruction)]
#[rustfmt::skip]
pub enum PithInstruction {
    #[account(0, name = "signer", desc = "Signer", signer)]
    #[account(1, name = "market", desc = "Pith market account", writable)]
    #[account(2, name = "system_program", desc = "Solana System Program", writable)]
    CreateMarket = 0,

    DeleteMarket = 1,

    UpdateMarket = 2,
}

#[repr(C)]
#[derive(Clone, Debug, BorshDeserialize)]
pub struct CreateMarketArgs {
    pub bump: u8,
    pub tx_id: String,
    pub title: String,
}
