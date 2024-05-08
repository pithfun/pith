use solana_program::{
    account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey, system_program,
};

/// Errors if:
/// The account is not the signer.
pub fn load_signer<'a, 'info>(info: &AccountInfo<'info>) -> Result<(), ProgramError> {
    if !info.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }
    Ok(())
}

// load_uninitialized_account will throw an error if;
// The account owner is not the system program.
// The account data is not empty.
// Account is not writable.
pub fn load_uninitialized_account<'a, 'info>(
    info: &'a AccountInfo<'info>,
) -> Result<(), ProgramError> {
    if info.owner.ne(&system_program::id()) {
        return Err(ProgramError::InvalidAccountOwner);
    }

    if !info.data_is_empty() {
        return Err(ProgramError::AccountAlreadyInitialized);
    }

    if !info.is_writable {
        return Err(ProgramError::InvalidAccountData);
    }

    Ok(())
}

/// Errors if:
/// The keys do not match
/// The bump does not match
pub fn load_uninitialized_pda<'a, 'info>(
    info: &'a AccountInfo<'info>,
    seeds: &[&[u8]],
    bump: u8,
    program_id: &Pubkey,
) -> Result<(), ProgramError> {
    let pda = Pubkey::find_program_address(seeds, program_id);

    if info.key.ne(&pda.0) {
        return Err(ProgramError::InvalidSeeds);
    }

    if bump.ne(&pda.1) {
        return Err(ProgramError::InvalidSeeds);
    }

    load_uninitialized_account(info)?;

    Ok(())
}

// load_program loads the program account. It will throw an error if;
// Account is not the program account
// Account is not executable
pub fn load_program<'a, 'info>(
    info: &'a AccountInfo<'info>,
    key: Pubkey,
) -> Result<(), ProgramError> {
    if info.key.ne(&key) {
        return Err(ProgramError::IncorrectProgramId);
    }

    if !info.executable {
        return Err(ProgramError::InvalidAccountData);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use solana_program::{
        account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey, system_program,
    };

    use crate::loaders::{load_signer, load_uninitialized_account};

    #[test]
    fn test_signer_is_signer() {
        let key = Pubkey::new_unique();
        let owner = system_program::id();
        let mut data = [];
        let mut lamports = 5_000_000_000;

        let info = AccountInfo::new(
            &key,
            false,
            false,
            &mut lamports,
            &mut data,
            &owner,
            false,
            0,
        );

        assert!(matches!(
            load_signer(&info),
            Err(ProgramError::MissingRequiredSignature)
        ))
    }

    #[test]
    fn test_load_uninitialized_account_bad_owner() {
        let key = Pubkey::new_unique();
        let owner = Pubkey::new_unique();
        let mut data = [];
        let mut lamports = 5_000_000_000;

        let info = AccountInfo::new(
            &key,
            false,
            false,
            &mut lamports,
            &mut data,
            &owner,
            false,
            0,
        );

        assert!(matches!(
            load_uninitialized_account(&info),
            Err(ProgramError::InvalidAccountOwner)
        ))
    }

    #[test]
    fn test_load_uninitialized_account_data_not_empty() {
        let key = Pubkey::new_unique();
        let owner = system_program::id();
        let mut data = [0xAA];
        let mut lamports = 5_000_000_000;

        let info = AccountInfo::new(
            &key,
            false,
            false,
            &mut lamports,
            &mut data,
            &owner,
            false,
            0,
        );

        assert!(matches!(
            load_uninitialized_account(&info),
            Err(ProgramError::AccountAlreadyInitialized)
        ))
    }

    #[test]
    fn test_load_uninitialized_account_is_not_writable() {
        let key = Pubkey::new_unique();
        let owner = system_program::id();
        let mut data = [];
        let mut lamports = 5_000_000_000;

        let info = AccountInfo::new(
            &key,
            false,
            false,
            &mut lamports,
            &mut data,
            &owner,
            false,
            0,
        );

        assert!(matches!(
            load_uninitialized_account(&info),
            Err(ProgramError::InvalidAccountData)
        ))
    }
}
