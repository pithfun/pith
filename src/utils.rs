use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program, pubkey::Pubkey, rent::Rent,
    system_instruction, sysvar::Sysvar,
};

// create_pda creates a new PDA.
pub(crate) fn create_pda<'a, 'info>(
    target_account: &'a AccountInfo<'info>,
    owner: &Pubkey,
    space: usize,
    pda_seeds: &[&[u8]],
    system_program: &'a AccountInfo<'info>,
    payer: &'a AccountInfo<'info>,
) -> ProgramResult {
    let rent = Rent::get()?;

    program::invoke_signed(
        &system_instruction::create_account(
            payer.key,
            target_account.key,
            rent.minimum_balance(space),
            space as u64,
            owner,
        ),
        &[
            payer.clone(),
            target_account.clone(),
            system_program.clone(),
        ],
        &[pda_seeds],
    )?;

    Ok(())
}
