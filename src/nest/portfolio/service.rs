
use std::collections::BTreeMap;

use super::{
    domain::{
        BuildProfile, PersonalInfo, WalletAddCommand, WalletId, AddressProfile, WalletProfile, WalletEditCommand
    }, 
    error::PersonalError
};

#[derive(Debug, Default)]
pub struct PersonalService{
    pub person_info: PersonalInfo,
    pub wallets: BTreeMap<u64, WalletProfile>,
    pub address_books: BTreeMap<u64, AddressProfile>,
}

impl PersonalService {
    pub fn add_wallet(&mut self, cmd: WalletAddCommand, id: WalletId, updated_at: u64) -> Result<u64, PersonalError> {
        match self.wallets.get(&id) {
            Some(_) => Err(PersonalError::WalletAlreadyExist),
            None => {
                self.wallets.insert(
                    id,
                    cmd.build_profile(
                        id,
                        updated_at
                    )
                );                
                Ok(id)
            }
        }   
    }  

    pub fn edit_wallet(&mut self, cmd: WalletEditCommand, updated_at: u64) -> Option<bool> {
        self.wallets.get_mut(&cmd.id).map(|profile| {
            let id = cmd.id;
            cmd.build_profile(id, updated_at);
        }).map(|_| true)
    }

    pub fn delete_wallet(&mut self, id: WalletId) -> Option<bool> {
        self.wallets.remove(&id).map(|_| true)
    }
    
    pub fn get_wallet(&self, id: WalletId) -> Option<WalletProfile> {
        self.wallets.get(&id).cloned()
    }

    pub fn list_wallets(&self) -> Vec<WalletProfile> {
        self.wallets.values().cloned().collect()
    }
    
}