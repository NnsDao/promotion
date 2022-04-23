use ic_cdk::export::candid::{CandidType, Deserialize};
use ic_cdk::api::call::CallResult;

type TokenIdentifier = String;
type AccountIdentifier = String;

pub type Memo = Vec<u8>;
pub type SubAccount = Vec<u8>;

#[derive(Clone, Debug, CandidType, Deserialize)]
pub enum User { principal(candid::Principal), address(AccountIdentifier) }

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct TransferRequest {
  pub to: User,
  pub token: TokenIdentifier,
  pub notify: bool,
  pub from: User,
  pub memo: Memo,
  pub subaccount: Option<SubAccount>,
  pub amount: u128,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub enum TransferResponse_err {
  CannotNotify(AccountIdentifier),
  InsufficientBalance,
  InvalidToken(TokenIdentifier),
  Rejected,
  Unauthorized(AccountIdentifier),
  Other(String),
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub enum TransferResponse { ok(u128), err(TransferResponse_err) }

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct BalanceRequest {
    pub token: TokenIdentifier,
    pub user: User,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub enum CommonError {
    InvalidToken(TokenIdentifier),
    Other(TokenIdentifier),
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub enum BalanceResponse {
    ok(u128),
    err(CommonError),
}

pub async fn transfer(canister_id: candid::Principal, arg: TransferRequest) -> CallResult<(TransferResponse,)> { 
    ic_cdk::call(canister_id, "transfer", (arg,)).await 
}

pub async fn balance(canister_id: candid::Principal, arg: BalanceRequest) -> CallResult<(BalanceResponse,)> {
  ic_cdk::call(canister_id, "balance", (arg,)).await
}