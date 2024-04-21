use solana_program::{
    account_info::AccountInfo, borsh1::try_from_slice_unchecked, entrypoint::ProgramResult,
    program_error::ProgramError, pubkey::Pubkey, system_program,
};

use crate::{instruction::CreateMarketArgs, loaders::*, state::Market, utils::*, MARKET};
use borsh::{BorshDeserialize, BorshSerialize};

// process_init_market creates a new tradable market.
pub fn process_init_market<'a, 'info>(
    _program_id: &Pubkey,
    accounts: &'a [AccountInfo<'info>],
    data: &[u8],
) -> ProgramResult {
    // Parse args
    let args = CreateMarketArgs::try_from_slice(data)?;

    // Load account data
    let [signer, market_info, system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    load_signer(signer)?;
    load_uninitialized_pda(
        market_info,
        &[MARKET, signer.key.as_ref(), args.tx_id.as_bytes().as_ref()],
        args.bump,
        &crate::id(),
    )?;
    load_program(system_program, system_program::id())?;

    // Create the market
    create_pda(
        market_info,
        &crate::id(),
        // Calculate how much space we need.
        // 1 byte => bump
        // 1 byte => is_initialzed
        // 4 bytes + title.len() => title
        // 4 bytes + tx_id.len() => tx_id
        1 + 1 + (4 + args.tx_id.len()) + (4 + args.title.len()),
        &[
            MARKET,
            signer.key.as_ref(),
            args.tx_id.as_bytes().as_ref(),
            &[args.bump],
        ],
        system_program,
        signer,
    )?;

    let mut market_data = try_from_slice_unchecked::<Market>(&market_info.data.borrow()).unwrap();
    market_data.bump = args.bump;
    market_data.tx_id = args.tx_id;
    market_data.title = args.title;

    market_data.serialize(&mut &mut market_info.data.borrow_mut()[..])?;

    Ok(())
}

pub fn process_delete_market<'a, 'info>(
    _program_id: &Pubkey,
    _accounts: &'a [AccountInfo<'info>],
    _data: &[u8],
) -> ProgramResult {
    todo!()
}

pub fn process_update_market<'a, 'info>(
    _program_id: &Pubkey,
    _accounts: &'a [AccountInfo<'info>],
    _data: &[u8],
) -> ProgramResult {
    todo!()
}
