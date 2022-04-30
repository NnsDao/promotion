//! This is the contract for IC Market promotion
mod canisters;
/// Promotion related methods
pub mod promotion;
mod types;
mod init;
// mod inspect_message;
mod owner;
mod conditions;
mod ledger;
mod tools;

use ic_cdk::api::time;
use ic_cdk::export::candid::Principal;
use ic_cdk_macros::*;
use ic_ledger_types::AccountIdentifier;
use promotion::{PromotionService, BuyRecord, ExchangeRecord};
use std::cell::RefCell;
use std::vec::Vec;
use owner::is_owner;

use canisters::{canister::*, ext};

const FREE :u64 = 0;

thread_local! {
    static PROMOTION_STATE: RefCell<PromotionService> = RefCell::default();
}

#[query]
#[candid::candid_method(query)]
fn get_owner() -> Option<Principal> {
    PROMOTION_STATE.with(|promotion_service| promotion_service.borrow().get_owner())
}

/// Get all promotions
#[query]
#[candid::candid_method(query)]
pub fn get_promotion_list() -> Vec<promotion::PromotionItem> {
    PROMOTION_STATE.with(|promotion_service| promotion_service.borrow().get_promotion_list())
}

/// Get Specifies promotion details
#[query]
#[candid::candid_method(query)]
pub fn get_promotion(id: u32) -> Option<promotion::Promotion> {
    PROMOTION_STATE.with(|promotion_service| promotion_service.borrow().get_promotion(id))
}

// add_promotion 'add_promotion(record {nft=vec {record {token="w7o4l-hakor-uwiaa-aaaaa-cqac6-aaqca-aaath-a"; is_saled=false}; record {token="mjkop-xikor-uwiaa-aaaaa-cqac6-aaqca-aaalm-q"; is_saled=false}}; canister_id="ah2fs-fqaaa-aaaak-aalya-cai"; end_time=1750772326000000000; start_time=1650772326000000000; conditions=opt record {canister_id="vgqnj-miaaa-aaaal-qaapa-cai"; limit=100000000000; canister_type=1}; price=100000000})'
#[update(guard = "is_owner")]
#[candid::candid_method]
fn add_promotion(arg: promotion::Promotion) -> () {
    PROMOTION_STATE.with(|promotion_service| {
        promotion_service.borrow_mut().add_promotion(arg);
    })
}

///update_promotion '(1, record {nft=vec {record {token="1"; can_buy=true; price=1:nat}}; status=true; conditions=opt record {canister_id="vgqnj-miaaa-aaaal-qaapa-cai"; limit=12:nat; canister_type=1:nat32}; canister_id="xxx"; end_time=1650281514000000000:nat64; start_time=1650195114000000000:nat64})'
#[update(guard = "is_owner")]
#[candid::candid_method]
fn update_promotion(id: u32, arg: promotion::Promotion) -> () {
    PROMOTION_STATE.with(|promotion_service| promotion_service.borrow_mut().update_promotion(id, arg))
}

#[update(guard = "is_owner")]
#[candid::candid_method(update)]
async fn capital_transfer(user: Principal, price: u64) -> Result<u64, String> {
    ledger::capital_transfer(user, price).await
}

/// Call before purchasing NFT
/// Pass in the Promition ID and return the transfer address and authorization code
/// After obtaining the transfer address and authorization code, conduct ICP transfer and ICP amount required for promotion. The authorization code will be put into the MEMO during the transfer for subsequent verification
#[update]
#[candid::candid_method]
pub async fn lock(id: u32) -> Result<(String ,u64), String> {
    if let Some(promotion_info) = PROMOTION_STATE.with(|promotion_service| promotion_service.borrow().get_promotion(id)) {
        let now = time();
        let address = AccountIdentifier::new(&ic_cdk::api::id(), &ic_ledger_types::DEFAULT_SUBACCOUNT).to_string();
        if promotion_info.start_time > now {
            return Err("promition not start".to_owned());
        }

        if now > promotion_info.end_time {
            return Err("promition is ended".to_owned());
        }
        let user = ic_cdk::caller();
        let approve_num = tools::subnet_raw_rand().await?;
        match promotion_info.conditions {
            Some(condition) => {
                if conditions::check_condition(user, condition).await {
                    match PROMOTION_STATE.with(|promotion_service| promotion_service.borrow_mut().set_approve(id, user, approve_num)) {
                        Ok(_) => return Ok((address, approve_num)),
                        Err(err) => return Err(err),
                    }
                } else {
                    return Err("Ineligible".to_owned());
                }
            }
            None => {
                match PROMOTION_STATE.with(|promotion_service| promotion_service.borrow_mut().set_approve(id, user, approve_num)) {
                    Ok(_) => return Ok((address, approve_num)),
                    Err(err) => return Err(err),
                }
            }
        }
    } else {
        return Err("promotion is not exist".to_owned());
    };
}

/// buy Nft
/// This method is called after the transfer has been made
/// Pass the promotion ID and transfer trouble to return the block height
#[update]
#[candid::candid_method(update)]
pub async fn buy(id: u32, block_height: u64) -> Result<bool, String> {
    let user_principal = ic_cdk::caller();
    let approve_num = PROMOTION_STATE.with(|promotion_service| promotion_service.borrow().get_approve(id, user_principal))?;
    let user = AccountIdentifier::new(&user_principal, &ic_ledger_types::DEFAULT_SUBACCOUNT);
    let to = AccountIdentifier::new(&ic_cdk::api::id(), &ic_ledger_types::DEFAULT_SUBACCOUNT);
    let promotion_info = PROMOTION_STATE.with(|promotion_service| promotion_service.borrow().get_promotion(id)).unwrap();
    if promotion_info.price == FREE || ledger::check_transfer(user.to_string(), to.to_string(), block_height, approve_num, promotion_info.price).await? {
        let token = PROMOTION_STATE.with(|promotion_service| promotion_service.borrow_mut().get_token(id, user_principal))?;
        let arg = ext::TransferRequest{
            to: ext::User::address(user.to_string()),
            token: token.clone(),
            notify: false,
            from: ext::User::address(to.to_string()),
            memo: Vec::new(),
            subaccount: None,
            amount: 1,
        };
        match CanisterExtClient::new(promotion_info.canister_id).transfer(arg).await.unwrap().0 {
            ext::TransferResponse::ok(_) => {
                let record = BuyRecord{
                    user: user.to_string(),
                    status: true,
                    token: token.clone(),
                    price: promotion_info.price,
                    time: time(),
                };
                PROMOTION_STATE.with(|promotion_service| promotion_service.borrow_mut().set_buy_recorde(id, record));
                return Ok(true);
            },
            ext::TransferResponse::err(_) => {
                let record = BuyRecord{
                    user: user.to_string(),
                    status: false,
                    token: token.clone(),
                    price: promotion_info.price,
                    time: time(),
                };
                PROMOTION_STATE.with(|promotion_service| promotion_service.borrow_mut().set_buy_recorde(id, record));
                return Err("Purchase failed".to_owned());
            },
        }
    }
    Err("Check for transaction failure".to_owned())
}

#[query]
#[candid::candid_method(query)]
fn get_record(id: u32) -> Option<Vec<BuyRecord>> {
    PROMOTION_STATE.with(|promotion_service| promotion_service.borrow().get_buy_recorde(id))
}

/// Ndp exchange authorization interface, similar to the LOCK method
#[update]
#[candid::candid_method(update)]
pub async fn approve() -> Result<(String ,u64), String> {
    let address = AccountIdentifier::new(&ic_cdk::api::id(), &ic_ledger_types::DEFAULT_SUBACCOUNT).to_string();
    let user = ic_cdk::caller();
    let approve_num = tools::subnet_raw_rand().await?;
    PROMOTION_STATE.with(|promotion_service| promotion_service.borrow_mut().set_exchange_approve(user, approve_num));
    Ok((address, approve_num))
}

/// Call this method after you get the authorization code get Ndp
/// Pass in the number of Icp exchanges and the block height, note the number of Icp with precision
#[update]
#[candid::candid_method(update)]
pub async fn exchange(amount: u64, block_height: u64) -> Result<u128, String> {
    let user = AccountIdentifier::new(&ic_cdk::caller(), &ic_ledger_types::DEFAULT_SUBACCOUNT);
    let from = AccountIdentifier::new(&ic_cdk::api::id(), &ic_ledger_types::DEFAULT_SUBACCOUNT);
    let approve_num = PROMOTION_STATE.with(|promotion_service| promotion_service.borrow().get_exchange_approve());
    if ledger::check_transfer(user.to_string(), from.to_string(), block_height, approve_num, amount).await? {
        // Temporarily set 200
        let ndp_amount = amount * 200;
        let ndp_arg = ext::TransferRequest{
            from: ext::User::address(from.to_string()),
            to: ext::User::address(user.to_string()),
            token: get_canister_id(CanisterEnmu::Ndp),
            notify: false,
            memo: Vec::new(),
            subaccount: None,
            amount:ndp_amount as u128,
        };
        match CanisterExtClient::new(get_canister_id(CanisterEnmu::Ndp)).transfer(ndp_arg).await.unwrap().0 {
            ext::TransferResponse::ok(height) => {
                let record = ExchangeRecord{
                    user: user.to_string(),
                    status: true,
                    amount: ndp_amount,
                    time: time(),
                };
                PROMOTION_STATE.with(|promotion_service| promotion_service.borrow_mut().set_exchange_record(record));
                return Ok(height);
            }
            ext::TransferResponse::err(_) => {
                let record = ExchangeRecord{
                    user: user.to_string(),
                    status: false,
                    amount: ndp_amount,
                    time: time(),
                };
                PROMOTION_STATE.with(|promotion_service| promotion_service.borrow_mut().set_exchange_record(record));
                return Err("Purchase failed".to_owned());
            }
        }
    }
    Err("Check for transaction failure".to_owned())
}

#[query]
#[candid::candid_method(query)]
fn get_ndp_record() -> Vec<ExchangeRecord> {
    PROMOTION_STATE.with(|promotion_service| promotion_service.borrow().get_exchange_record())
}


#[pre_upgrade]
fn pre_upgrade() {
    let stable_state = PROMOTION_STATE.with(|s| s.take());
    ic_cdk::storage::stable_save((stable_state,)).expect("failed to save stable state");
}

#[post_upgrade]
fn post_upgrade() {
    let (stable_state,) = ic_cdk::storage::stable_restore().expect("failed to restore stable state");
    PROMOTION_STATE.with(|s| {
        s.replace(stable_state);
    });
}

candid::export_service!();

#[query(name = "__get_candid_interface_tmp_hack")]
fn export_candid() -> String {
    __export_service()
}
