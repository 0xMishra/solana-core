use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, msg, pubkey::Pubkey,
};

pub mod instruction;
use instruction::StudentIntroInstruction;

entrypoint!(process_instruction);

pub fn add_student_intro(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    name: String,
    intro: String,
) -> ProgramResult {
    msg!("adding student intro");
    msg!("student name: {}", name);
    msg!("student intro: {}", intro);
    Ok(())
}

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = StudentIntroInstruction::unpack(instruction_data)?;

    match instruction {
        StudentIntroInstruction::AddStudent { name, intro } => {
            add_student_intro(program_id, accounts, name, intro)
        }
    }
}
