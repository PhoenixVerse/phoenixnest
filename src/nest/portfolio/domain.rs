use candid::{CandidType, Deserialize};

pub type  Timestamp = u64;

pub trait BuildProfile<K, V> {
    fn build_profile(self, id: K, updated_at: Timestamp) -> V;
}

/// 个人信息
#[derive(Debug, Clone, CandidType, Deserialize, Default)]
pub struct PersonalInfo {
    pub name: String,
    pub email: Vec<String>,
    pub photos: Vec<u64>,
}

pub type WalletId = u64;

/// 钱包或数字钱包
#[derive(Debug, Clone, CandidType, Deserialize, Default)]
pub struct WalletProfile {
    pub id: WalletId,
    pub name: String,
    pub account: String,    //  or wallet address
    pub updated_at: Timestamp,
}

#[derive(Debug, Clone, CandidType, Deserialize, Default)]
pub struct WalletAddCommand {
    name: String,
    account: String,
}

impl BuildProfile<u64, WalletProfile> for WalletAddCommand {
    fn build_profile(self, id: WalletId, updated_at: Timestamp) -> WalletProfile {
        WalletProfile { id, name: self.name, account: self.account, updated_at }
    }
}

#[derive(Debug, Clone, CandidType, Deserialize, Default)]
pub struct WalletEditCommand {
    pub id: WalletId,
    pub name: String,
    pub account: String,
    pub created_at: Timestamp
}

impl BuildProfile<u64, WalletProfile> for WalletEditCommand {
    fn build_profile(self, id: WalletId, updated_at: Timestamp) -> WalletProfile {
        assert_eq!(id, self.id);
        WalletProfile { id, name: self.name, account: self.account, updated_at }
    }
}

pub type AddressId = u64;

/// 通讯录
#[derive(Debug, Clone, CandidType, Deserialize, Default)]
pub struct AddressProfile {
    pub id: AddressId,
    pub name: String,
    pub tel: Vec<String>,
    pub address: Vec<AddressValue>,
    pub created_at: Timestamp,
}

#[derive(Debug, Clone, CandidType, Deserialize, Default)]
pub struct AddressValue {
    tag: String,
    address: String,
}

