use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, msg, pubkey::Pubkey,
};
pub mod instruction;
use instruction::MovieInstruction;

entrypoint!(process_instruction);

pub fn add_movie_review(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    title: String,
    rating: u8,
    description: String,
) -> ProgramResult {
    msg!("Adding movie review...");
    msg!("Title: {}", title);
    msg!("Rating: {}", rating);
    msg!("Description: {}", description);

    Ok(())
}

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = MovieInstruction::unpack(instruction_data);

    match instruction {
        Ok(MovieInstruction::AddMovieReview {
            title,
            rating,
            description,
        }) => {
            add_movie_review(program_id, accounts, title, rating, description);
        }
        _ => {}
    }

    Ok(())
}
