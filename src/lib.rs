pub mod consts;
pub mod instruction;
mod loaders;
pub mod processor;
pub mod state;
pub mod utils;

use solana_program::{
    account_info::AccountInfo, declare_id, entrypoint, entrypoint::ProgramResult,
    program_error::ProgramError, pubkey::Pubkey,
};

pub use consts::*;
use instruction::PithInstruction;
use processor::*;

declare_id!("pith7Ki9KigbwAtC91aapAnRBxDFE9WY9gyCntDU1Ea");

#[cfg(not(feature = "no-entrypoint"))]
entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    if program_id.ne(&crate::id()) {
        return Err(ProgramError::IncorrectProgramId);
    }

    let (tag, data) = instruction_data
        .split_first()
        .ok_or(ProgramError::InvalidInstructionData)?;

    match PithInstruction::try_from(*tag).or(Err(ProgramError::InvalidInstructionData))? {
        PithInstruction::Market => process_market(program_id, accounts, data)?,
        PithInstruction::Bid => process_bid(program_id, accounts, data)?,
    }

    Ok(())
}
