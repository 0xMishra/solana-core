use borsh::BorshDeserialize;
use solana_program::program_error::ProgramError;

pub enum StudentIntroInstruction {
    AddStudentIntro { name: String, msg: String },
}

impl StudentIntroInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (&variant, rest) = input
            .split_first()
            .ok_or(ProgramError::InvalidInstructionData)?;
        let payload = StudentIntroPayload::try_from_slice(rest).unwrap();

        Ok(match variant {
            0 => Self::AddStudentIntro {
                name: payload.name,
                msg: payload.msg,
            },
            _ => return Err(ProgramError::InvalidInstructionData),
        })
    }
}

#[derive(BorshDeserialize)]
struct StudentIntroPayload {
    name: String,
    msg: String,
}

