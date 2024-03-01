#[cfg(not(feature = "no-entrypoint"))]
pub mod entrypoint {
    use solana_program::{
        account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, msg, pubkey::Pubkey,
    };

    entrypoint!(init_bottler);

    pub fn init_bottler(
        _program_id: &Pubkey,
        _accounts: &[AccountInfo],
        _instruction_data: &[u8],
    ) -> ProgramResult {
        msg!("Welcome to TinyBlob!");
        Ok(())
    }
}
