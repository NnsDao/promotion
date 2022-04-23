use crate::PROMOTION_STATE;

pub fn is_owner() -> Result<(), String> {
    PROMOTION_STATE.with(|promotion_service|promotion_service.borrow().is_owner())
}
