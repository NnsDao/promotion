use ic_cdk_macros::inspect_message;
use ic_cdk::api::call::accept_message;
use crate::owner::is_owner;

#[inspect_message]
fn inspect_message() {
    if is_owner().is_ok() {
        accept_message()
    }
}