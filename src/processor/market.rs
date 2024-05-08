use solana_program::{
    account_info::AccountInfo, borsh1::try_from_slice_unchecked, entrypoint::ProgramResult,
    program_error::ProgramError, pubkey::Pubkey, system_program,
};

use crate::{instruction::MarketArgs, loaders::*, state::Market, utils::*, MARKET};
use borsh::{BorshDeserialize, BorshSerialize};

// process_market creates a new tradable market.
pub fn process_market<'a, 'info>(
    _program_id: &Pubkey,
    accounts: &'a [AccountInfo<'info>],
    data: &[u8],
) -> ProgramResult {
    // Parse args
    let args = MarketArgs::try_from_slice(data)?;

    // Load account data
    let [signer, market_info, system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    load_signer(signer)?;
    load_uninitialized_pda(
        market_info,
        &[MARKET, signer.key.as_ref(), args.id.to_le_bytes().as_ref()],
        args.bump,
        &crate::id(),
    )?;
    load_program(system_program, system_program::id())?;

    // Create the market
    create_pda(
        market_info,
        &crate::id(),
        Market::get_account_size(&args.title, &Market::DISCRIMINATOR.to_string()),
        &[
            MARKET,
            signer.key.as_ref(),
            args.id.to_le_bytes().as_ref(),
            &[args.bump],
        ],
        system_program,
        signer,
    )?;

    let mut market_data = try_from_slice_unchecked::<Market>(&market_info.data.borrow()).unwrap();
    market_data.discriminator = Market::DISCRIMINATOR.to_string();
    market_data.bump = args.bump;
    market_data.authority = *signer.key;
    market_data.id = args.id;
    market_data.title = args.title;
    market_data.key = *market_info.key;
    market_data.counter = 0;
    market_data.serialize(&mut &mut market_info.data.borrow_mut()[..])?;

    Ok(())
}
