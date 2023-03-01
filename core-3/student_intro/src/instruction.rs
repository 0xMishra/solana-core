use borsh::BorshDeserialize;
use solana_program::program_error::ProgramError;

pub enum StudentIntroInstruction {
    AddStudentIntro { name: String, msg: String },
    UpdateStudentIntro { name: String, msg: String },
    ReplyToIntro { reply: String },
}

impl StudentIntroInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (&variant, rest) = input
            .split_first()
            .ok_or(ProgramError::InvalidInstructionData)?;

        Ok(match variant {
            0 => {
                let payload = StudentIntroPayload::try_from_slice(rest).unwrap();
                Self::AddStudentIntro {
                    name: payload.name,
                    msg: payload.msg,
                }
            }
            1 => {
                let payload = StudentIntroPayload::try_from_slice(rest).unwrap();
                Self::UpdateStudentIntro {
                    name: payload.name,
                    msg: payload.msg,
                }
            }
            2 => {
                let payload = ReplyPayload::try_from_slice(rest).unwrap();
                Self::ReplyToIntro {
                    reply: payload.reply,
                }
            }
            _ => return Err(ProgramError::InvalidInstructionData),
        })
    }
}

#[derive(BorshDeserialize)]
struct StudentIntroPayload {
    name: String,
    msg: String,
}

#[derive(BorshDeserialize)]
struct ReplyPayload {
    reply: String,
}
