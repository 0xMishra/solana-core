use borsh::BorshDeserialize;
use solana_program::program_error::ProgramError;

pub enum StudentIntroInstruction {
    AddStudent { name: String, intro: String },
}

impl StudentIntroInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (&variant, rest) = input
            .split_first()
            .ok_or(ProgramError::InvalidInstructionData)?;
        let payload = StudentIntroPayload::try_from_slice(rest).unwrap();

        Ok(match variant {
            0 => Self::AddStudent {
                name: payload.name,
                intro: payload.intro,
            },
            _ => return Err(ProgramError::InvalidInstructionData),
        })
    }
}

#[derive(BorshDeserialize)]
struct StudentIntroPayload {
    name: String,
    intro: String,
}
