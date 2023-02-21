use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize)]
pub struct StudentIntroAccountState {
    pub is_initialized: bool,
    pub name: String,
    pub msg: String,
}
