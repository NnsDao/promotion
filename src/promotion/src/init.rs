use ic_cdk_macros::init;
use crate::PROMOTION_STATE;

#[init]
fn init() {
    ic_cdk::setup();
    PROMOTION_STATE.with(|promotion_service| promotion_service.borrow_mut().set_owner(ic_cdk::caller()))
}