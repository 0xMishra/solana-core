use crate::instruction::StudentIntroInstruction;
use crate::state::{IntroReply, StudentIntroAccountState};
use crate::{error::IntroError, state::IntroReplyCounter};
use borsh::BorshSerialize;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    borsh::try_from_slice_unchecked,
    entrypoint::ProgramResult,
    msg,
    program::invoke_signed,
    program_error::ProgramError,
    program_pack::IsInitialized,
    pubkey::Pubkey,
    system_instruction,
    sysvar::{rent::Rent, Sysvar},
};

use std::convert::TryInto;

pub fn update_student_intro(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    name: String,
    msg: String,
) -> ProgramResult {
    msg!("Updating student intro");
    msg!("student name: {}", name);
    msg!("{}", msg);

    // Get Account iterator
    let account_info_iter = &mut accounts.iter();

    // Get accounts
    let initializer = next_account_info(account_info_iter)?;
    let pda_account = next_account_info(account_info_iter)?;

    // signer check
    if !initializer.is_signer {
        msg!("Missing required signature");
        return Err(ProgramError::MissingRequiredSignature);
    }

    msg!("unpacking state account");
    let mut account_data =
        try_from_slice_unchecked::<StudentIntroAccountState>(&pda_account.data.borrow()).unwrap();
    msg!("borrowed account data");

    // Derive PDA and check that it matches client
    let (pda, _bump_seed) = Pubkey::find_program_address(&[initializer.key.as_ref()], program_id);

    // PDA check
    if pda != *pda_account.key {
        msg!("Invalid seeds for PDA");
        return Err(IntroError::InvalidPDA.into());
    }

    if !account_data.is_initialized {
        msg!("Account is not initialized");
        return Err(IntroError::UninitializedAccount.into());
    }

    // data validation
    if msg.len() > 50 {
        msg!("Intro cannot be longer than 50 characters");
        return Err(IntroError::InvalidIntroLength.into());
    }

    // calculate account size required
    let account_len = 1 + (4 + msg.len());

    if account_len > 1000 {
        msg!("Data length is larger than 1000 bytes");
        return Err(IntroError::InvalidDataLength.into());
    }

    // Update student intro
    account_data.msg = msg;
    account_data.serialize(&mut &mut pda_account.data.borrow_mut()[..])?;
    Ok(())
}

pub fn add_student_intro(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    name: String,
    msg: String,
) -> ProgramResult {
    msg!("Adding student intro");
    msg!("Student name:{}", name);
    msg!("{}", msg);

    // Get Account iterator
    let account_info_iter = &mut accounts.iter();

    // Get accounts
    let initializer = next_account_info(account_info_iter)?;
    let pda_account = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;
    let pda_counter = next_account_info(account_info_iter)?;

    // signer check
    if !initializer.is_signer {
        msg!("Missing required signature");
        return Err(ProgramError::MissingRequiredSignature);
    }

    let (pda, bump_seed) = Pubkey::find_program_address(&[initializer.key.as_ref()], program_id);

    // PDA check
    if pda != *pda_account.key {
        msg!("Invalid seeds for PDA");
        return Err(ProgramError::InvalidArgument);
    }

    // data validation
    if name.len() > 15 {
        msg!("Name cannot be longer than 15 characters");
        return Err(IntroError::InvalidNameLength.into());
    }

    if msg.len() > 50 {
        msg!("Intro cannot be longer than 50 characters");
        return Err(IntroError::InvalidIntroLength.into());
    }

    // calculate account size required
    let account_len: usize = 1000;

    if StudentIntroAccountState::get_account_size(name.clone(), msg.clone()) > account_len {
        msg!("Data length is larger than 1000 bytes");
        return Err(IntroError::InvalidDataLength.into());
    }

    // Calculate rent required
    let rent = Rent::get()?;
    let rent_lamports = rent.minimum_balance(account_len);

    invoke_signed(
        &system_instruction::create_account(
            initializer.key,
            pda_account.key,
            rent_lamports,
            account_len.try_into().unwrap(),
            program_id,
        ),
        &[
            initializer.clone(),
            pda_account.clone(),
            system_program.clone(),
        ],
        &[&[initializer.key.as_ref(), &[bump_seed]]],
    )?;
    msg!("PDA created: {}", pda);

    msg!("unpacking state account");
    let mut account_data =
        try_from_slice_unchecked::<StudentIntroAccountState>(&pda_account.data.borrow()).unwrap();
    msg!("borrowed account data");

    account_data.discriminator = StudentIntroAccountState::DISCRIMINATOR.to_string();
    account_data.student_key = *initializer.key;
    account_data.name = name;
    account_data.msg = msg;
    account_data.is_initialized = true;

    msg!("Creating reply counter");
    let rent = Rent::get()?;
    let counter_rent_lamports = rent.minimum_balance(IntroReplyCounter::SIZE);
    // Deriving the address and validating that the correct seeds were passed in
    let (counter, counter_bump) =
        Pubkey::find_program_address(&[pda.as_ref(), "reply".as_ref()], program_id);
    if counter != *pda_counter.key {
        msg!("Invalid seeds for PDA");
        return Err(ProgramError::InvalidArgument);
    }

    // Creating the reply counter account
    invoke_signed(
        &system_instruction::create_account(
            initializer.key,                             // Rent payer
            pda_counter.key,       // Address who we're creating the account for
            counter_rent_lamports, // Amount of rent to put into the account
            IntroReplyCounter::SIZE.try_into().unwrap(), // Size of the account
            program_id,
        ),
        &[
            // List of accounts that will be read from/written to
            initializer.clone(),
            pda_counter.clone(),
            system_program.clone(),
        ],
        // Seeds for the PDA
        // PDA account
        // The string "reply"
        &[&[pda.as_ref(), "reply".as_ref(), &[counter_bump]]],
    )?;
    msg!("reply counter created");

    // Deserialize the newly created counter account
    let mut counter_data =
        try_from_slice_unchecked::<IntroReplyCounter>(&pda_counter.data.borrow()).unwrap();

    msg!("checking if counter account is already initialized");
    if counter_data.is_initialized() {
        msg!("Account already initialized");
        return Err(ProgramError::AccountAlreadyInitialized);
    }

    counter_data.discriminator = IntroReplyCounter::DISCRIMINATOR.to_string();
    counter_data.counter = 0;
    counter_data.is_initialized = true;
    msg!("reply count: {}", counter_data.counter);
    counter_data.serialize(&mut &mut pda_counter.data.borrow_mut()[..])?;

    msg!("reply counter initialized");

    msg!("serializing account");
    account_data.serialize(&mut &mut pda_account.data.borrow_mut()[..])?;
    msg!("state account serialized");
    Ok(())
}

pub fn reply_to_intro(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    reply: String,
) -> ProgramResult {
    msg!("Adding Reply...");
    msg!("Reply: {}", reply);

    let account_info_iter = &mut accounts.iter();

    let replier = next_account_info(account_info_iter)?;
    let pda_intro = next_account_info(account_info_iter)?;
    let pda_counter = next_account_info(account_info_iter)?;
    let pda_reply = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;

    let mut counter_data =
        try_from_slice_unchecked::<IntroReplyCounter>(&pda_counter.data.borrow()).unwrap();

    let account_len = IntroReply::get_account_size(reply.clone());

    let rent = Rent::get()?;
    let rent_lamports = rent.minimum_balance(account_len);

    let (pda, bump_seed) = Pubkey::find_program_address(
        &[
            pda_intro.key.as_ref(),
            counter_data.counter.to_be_bytes().as_ref(),
        ],
        program_id,
    );

    if pda != *pda_reply.key {
        msg!("Invalid seeds for PDA");
        return Err(IntroError::InvalidPDA.into());
    }

    invoke_signed(
        &system_instruction::create_account(
            replier.key,
            pda_reply.key,
            rent_lamports,
            account_len.try_into().unwrap(),
            program_id,
        ),
        &[replier.clone(), pda_reply.clone(), system_program.clone()],
        &[&[
            pda_intro.key.as_ref(),
            counter_data.counter.to_be_bytes().as_ref(),
            &[bump_seed],
        ]],
    )?;

    msg!("Created Reply Account");

    let mut reply_data = try_from_slice_unchecked::<IntroReply>(&pda_reply.data.borrow()).unwrap();

    msg!("checking if reply account is already initialized");
    if reply_data.is_initialized() {
        msg!("Account already initialized");
        return Err(ProgramError::AccountAlreadyInitialized);
    }

    reply_data.discriminator = IntroReply::DISCRIMINATOR.to_string();
    reply_data.intro = *pda_intro.key;
    reply_data.replier = *replier.key;
    reply_data.reply = reply;
    reply_data.is_initialized = true;
    reply_data.serialize(&mut &mut pda_reply.data.borrow_mut()[..])?;

    msg!("Reply Count: {}", counter_data.counter);
    counter_data.counter += 1;
    counter_data.serialize(&mut &mut pda_counter.data.borrow_mut()[..])?;

    Ok(())
}

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = StudentIntroInstruction::unpack(instruction_data)?;
    match instruction {
        StudentIntroInstruction::AddStudentIntro { name, msg } => {
            add_student_intro(program_id, accounts, name, msg)
        }
        StudentIntroInstruction::UpdateStudentIntro { name, msg } => {
            update_student_intro(program_id, accounts, name, msg)
        }
        StudentIntroInstruction::ReplyToIntro { reply } => {
            reply_to_intro(program_id, accounts, reply)
        }
    }
}
