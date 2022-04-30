use super::ext;

use ic_cdk::export::candid::Principal;
use ic_cdk::api::call::CallResult;

pub enum CanisterEnmu {
    Ndp,
    Pod,
    Starfish,
}

pub fn get_canister_id(canister: CanisterEnmu) -> String {
    match canister {
        CanisterEnmu::Ndp => String::from("vgqnj-miaaa-aaaal-qaapa-cai"),
        CanisterEnmu::Pod => String::from("ah2fs-fqaaa-aaaak-aalya-cai"),
        CanisterEnmu::Starfish => String::from("vcpye-qyaaa-aaaak-qafjq-cai"),
    }
}

pub struct CanisterExtClient{
    id: String,
}

impl CanisterExtClient {
    pub fn new(canister_id: String) -> Self {
        CanisterExtClient{
            id: canister_id,
        }
    }

    pub async fn transfer(&self, arg: ext::TransferRequest) -> CallResult<(ext::TransferResponse,)> {
        ext::transfer(Principal::from_text(self.id.as_str()).unwrap(), arg).await
    }

    pub async fn balance(&self, arg: ext::BalanceRequest) -> CallResult<(ext::BalanceResponse,)> {
        ext::balance(Principal::from_text(self.id.as_str()).unwrap(), arg).await
    }

}