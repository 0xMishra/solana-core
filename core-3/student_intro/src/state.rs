use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    program_pack::{IsInitialized, Sealed},
    pubkey::Pubkey,
};

#[derive(BorshSerialize, BorshDeserialize)]
pub struct StudentIntroAccountState {
    pub discriminator: String,
    pub is_initialized: bool,
    pub student_key: Pubkey,
    pub name: String,
    pub msg: String,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct IntroReplyCounter {
    pub discriminator: String,
    pub is_initialized: bool,
    pub counter: u64,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct IntroReply {
    pub discriminator: String,
    pub is_initialized: bool,
    pub intro: Pubkey,
    pub replier: Pubkey,
    pub reply: String,
    pub count: u64,
}

impl StudentIntroAccountState {
    pub const DISCRIMINATOR: &'static str = "intro";
    pub fn get_account_size(name: String, msg: String) -> usize {
        return (4 + Self::DISCRIMINATOR.len()) + 1 + 32 + (4 + name.len()) + (4 + msg.len());
    }
}

impl IntroReply {
    pub const DISCRIMINATOR: &'static str = "reply";
    pub fn get_account_size(reply: String) -> usize {
        return (4 + Self::DISCRIMINATOR.len()) + 1 + 32 + 32 + (4 + reply.len()) + 8;
    }
}

impl IntroReplyCounter {
    pub const DISCRIMINATOR: &'static str = "counter";
    pub const SIZE: usize = (4 + Self::DISCRIMINATOR.len()) + 1 + 8;
}

impl Sealed for StudentIntroAccountState {}
impl Sealed for IntroReplyCounter {}

impl IsInitialized for StudentIntroAccountState {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl IsInitialized for IntroReplyCounter {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl IsInitialized for IntroReply {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}
