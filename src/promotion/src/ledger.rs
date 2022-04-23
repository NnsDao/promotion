use ic_nns_constants::LEDGER_CANISTER_ID;
use ledger_canister::{
    Block, BlockArg, BlockRes, Operation, Memo
};
use dfn_protobuf::protobuf;
use dfn_core::{api::call_with_cleanup};
use ic_types::CanisterId;
use ic_cdk::export::candid::Principal;

use ic_ledger_types::{TransferArgs,BlockIndex, AccountIdentifier, Memo as Memo_1, Tokens, DEFAULT_SUBACCOUNT, MAINNET_LEDGER_CANISTER_ID, DEFAULT_FEE};

pub async fn get_block(height: u64) -> Result<Result<Block, CanisterId>, String> {
    let BlockRes(res) =
        call_with_cleanup(LEDGER_CANISTER_ID, "block_pb", protobuf, BlockArg(height))
            .await
            .map_err(|e| format!("Failed to fetch block {}", e.1))?;
    match res.ok_or("Block not found")? {
        Ok(raw_block) => {
            let block = raw_block.decode().unwrap();
            Ok(Ok(block))
        }
        Err(canister_id) => Ok(Err(canister_id)),
    }
}

pub async fn check_transfer(user: String, receive: String, block_height: u64, memo: u64, price: u64) -> Result<bool, String> {
    match get_block(block_height).await {
        Ok(result_1) => {
            match result_1 {
                Ok(block) => {
                    if let Operation::Transfer{from,to,amount, ..} = block.transaction.operation {
                        if to.to_hex() == receive && from.to_hex() == user && amount.get_e8s() == price && block.transaction.memo == Memo(memo) {
                            return Ok(true);
                        }
                    }
                    return Err("Transaction discipline query failed".to_owned());
                }
                _ => {return Err("The request failed, retry".to_owned());}
            }
        }
        _ => {return Err("The request failed, retry".to_owned());}
    }
}

pub async fn capital_transfer(user: Principal, price: u64) ->  Result<BlockIndex, String> {
    let arg = TransferArgs{
        memo: Memo_1(0),
        amount: Tokens::from_e8s(price),
        fee: DEFAULT_FEE,
        from_subaccount: None,
        to: AccountIdentifier::new(&user, &DEFAULT_SUBACCOUNT),
        created_at_time: None,
    };
    ic_ledger_types::transfer(MAINNET_LEDGER_CANISTER_ID, arg).await
    .map_err(|e| format!("failed to call ledger: {:?}", e))?
    .map_err(|e| format!("ledger transfer error {:?}", e))
}
