use candid::{CandidType, Deserialize};


#[derive(Debug, Clone, CandidType, Deserialize)]
pub enum PersonalError {
    WalletAlreadyExist,
    
}