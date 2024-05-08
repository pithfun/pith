use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::AccountInfo, borsh1::try_from_slice_unchecked, entrypoint::ProgramResult, msg,
    program_error::ProgramError, pubkey::Pubkey, system_program,
};

use crate::{
    instruction::BidArgs,
    loaders::{load_program, load_signer, load_uninitialized_pda},
    state::{Bid, Market},
    utils::create_pda,
    BID,
};

/// process_bid handles the creation of a new bid on a market.
pub fn process_bid<'a, 'info>(
    _program_id: &Pubkey,
    accounts: &'a [AccountInfo<'info>],
    data: &[u8],
) -> ProgramResult {
    // Parse args
    let args = BidArgs::try_from_slice(data)?;

    // Load account data
    let [signer, market_info, bid_info, system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    let mut market_data = try_from_slice_unchecked::<Market>(&market_info.data.borrow())?;

    load_signer(signer)?;
    load_uninitialized_pda(
        bid_info,
        &[
            BID,
            market_data.id.to_le_bytes().as_ref(),
            signer.key.as_ref(),
            args.id.to_le_bytes().as_ref(),
        ],
        args.bump,
        &crate::id(),
    )?;
    load_program(system_program, system_program::id())?;

    // create bid Program Derived Address.
    create_pda(
        bid_info,
        &crate::id(),
        Bid::SIZE,
        &[
            BID,
            market_data.id.to_le_bytes().as_ref(),
            signer.key.as_ref(),
            args.id.to_le_bytes().as_ref(),
            &[args.bump],
        ],
        system_program,
        signer,
    )?;

    let mut bid_data = try_from_slice_unchecked::<Bid>(&bid_info.data.borrow()).unwrap();

    bid_data.discriminator = Bid::DISCRIMINATOR.to_string();
    bid_data.market = *market_info.key;
    bid_data.amount = args.amount;
    bid_data.authority = *signer.key;
    bid_data.serialize(&mut &mut bid_info.data.borrow_mut()[..])?;

    market_data.counter += 1;
    market_data.serialize(&mut &mut market_info.data.borrow_mut()[..])?;

    Ok(())
}
