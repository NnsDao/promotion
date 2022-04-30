use ic_cdk::export::candid::{Principal};
use ic_ledger_types::AccountIdentifier;
use crate::promotion::Condition;
use crate::canisters::{ext, canister::*};

pub async fn check_condition(caller: Principal, condition: Condition) -> bool {
    if condition.canister_id ==  get_canister_id(CanisterEnmu::Ndp) {
        if let ext::BalanceResponse::ok(ndp_balance) = get_balance(caller).await {
            if ndp_balance >= condition.limit {
                return true;
            }
        }
        return false;
    }

    if condition.canister_id ==  get_canister_id(CanisterEnmu::Starfish) {
        return false;
    }

    return false;
}

async fn get_balance(caller: Principal) -> ext::BalanceResponse {
    let addr = AccountIdentifier::new(&caller, &ic_ledger_types::DEFAULT_SUBACCOUNT);
    let arg = ext::BalanceRequest {
        token: get_canister_id(CanisterEnmu::Ndp),
        user: ext::User::address(addr.to_string()),
    };
    CanisterExtClient::new(get_canister_id(CanisterEnmu::Ndp))
        .balance(arg)
        .await
        .unwrap()
        .0
}