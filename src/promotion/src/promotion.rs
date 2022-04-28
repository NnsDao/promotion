use ic_cdk::export::{
    candid::{CandidType, Deserialize},
    Principal,
};
use std::collections::HashMap;
use std::vec::Vec;

type PromotionId = u32;

/// canister_type 1: quantity 2: NRI
/// canister_id vgqnj-miaaa-aaaal-qaapa-cai : NDP
#[derive(Clone, Debug, Default, CandidType, Deserialize)]
pub struct Condition {
    pub canister_type: u32,
    pub limit: u128,
    pub canister_id: String,
}

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
pub struct Nft {
    pub token: String,
    pub is_saled: bool,
}

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
pub struct Promotion {
    pub canister_id: String,
    pub price: u64,
    pub conditions: Option<Condition>,
    pub nft: Vec<Nft>,
    pub start_time: u64,
    pub end_time: u64,
}

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
pub struct PromotionItem {
    pub id: u32,
    pub promotion: Promotion,
}

#[derive(Clone, Debug, CandidType, Deserialize, Default)]
pub struct BuyRecord {
    pub user: String,
    pub status: bool,
    pub token: String,
    pub price: u64,
    pub time: u64,
}

#[derive(Clone, Debug, CandidType, Deserialize, Default)]
pub struct ExchangeRecord {
    pub user: String,
    pub status: bool,
    pub amount: u64,
    pub time: u64,
}

#[derive(CandidType, Clone, Deserialize, Default)]
pub struct PromotionService {
    pub id: PromotionId,
    pub owner: Option<Principal>,
    pub promotion_list: HashMap<PromotionId, Promotion>,
    pub approve_list: HashMap<PromotionId, Vec<(Principal, u64)>>,
    pub buy_record: HashMap<PromotionId, Vec<BuyRecord>>,
    // pub excahnge_amount: u128,
    pub exchange_record: Vec<ExchangeRecord>,
    pub exchange_approve_list: HashMap<Principal, u64>,
}

impl PromotionService {
    pub fn add_promotion(&mut self, arg: Promotion) -> () {
        self.id = self.id + 1;
        self.promotion_list.insert(self.id, arg);
        self.approve_list.insert(self.id, Vec::new());
        self.buy_record.insert(self.id, Vec::new());
    }

    pub fn get_promotion_list(&self) -> Vec<PromotionItem> {
        self.promotion_list
            .clone()
            .into_iter()
            .map(|(id, promotion)| PromotionItem { id, promotion })
            .collect()
    }

    pub fn get_promotion(&self, id: PromotionId) -> Option<Promotion> {
        self.promotion_list.get(&id).cloned()
    }

    
    pub fn update_promotion(&mut self, id: PromotionId, arg: Promotion) -> () {
        match self.promotion_list.get(&id) {
            Some(_promotion_info) => {
                self.promotion_list.insert(id, arg);
            }
            None => {}
        }
    }

    pub fn set_approve(&mut self, id: PromotionId, user: Principal, approve_num: u64) -> Result<bool, String>{
        if id > self.id {
            return Err("no record".to_owned());
        }
        let number = self.promotion_list.get(&id).unwrap().nft.len();
        ic_cdk::println!{"{:?},{:?}", number, self.approve_list.len()};
        for record_item in self.buy_record.get(&id).unwrap().to_vec().iter() {
            if record_item.user == user.to_string() {
                return Err("Already purchased".to_owned());
            }
        }

        if self.approve_list.len() < number {
            let mut approve : Vec<(Principal, u64)>= self.approve_list.get(&id).unwrap().to_vec();
            let mut index = 999999999;
            for (i, item) in approve.iter().enumerate() {
                if item.0 == user {
                    index = i;
                }
            }
            if index != 999999999 {
                return Err("Approved already".to_owned());
            }
            approve.push((user, approve_num));
            self.approve_list.insert(id, approve);
            return Ok(true);
        } else {
            return Err("saled out".to_owned());
        }
    }

    pub fn get_approve(&self, id: PromotionId) -> Result<u64, String> {
        if id > self.id {
            return Err("unauthorized".to_owned());
        }
        let approve : Vec<(Principal, u64)>= self.approve_list.get(&id).unwrap().to_vec();
        let user = ic_cdk::caller();
        for item in approve.iter() {
            if item.0 == user {
                return Ok(item.1);
            }
        }
        Err("no approve number".to_owned())
    }

    pub fn get_token(&mut self, id: PromotionId) -> Result<String, String> {
        let approve : Vec<(Principal, u64)>= self.approve_list.get(&id).unwrap().to_vec();
        let user = ic_cdk::caller();
        for (i, item) in approve.iter().enumerate() {
            if item.0 == user {

                let mut promotion = self.promotion_list.get(&id).unwrap().clone();
                promotion.nft[i].is_saled = true;
                let token = promotion.nft[i].token.clone();
                self.promotion_list.insert(id, promotion);
                return Ok(token);
            }
        }
        Err("unauthorized".to_owned())
    }

    pub fn set_buy_recorde(&mut self, id: PromotionId, record: BuyRecord) {
        let mut buy_record = self.buy_record.get(&id).unwrap().to_vec();
        buy_record.push(record);
        self.buy_record.insert(id, buy_record);
    }

    pub fn get_buy_recorde(&self, id: PromotionId) -> Option<Vec<BuyRecord>> {
        self.buy_record.get(&id).cloned()
    }

    pub fn set_exchange_approve(&mut self, user: Principal, approve: u64) {
        self.exchange_approve_list.insert(user, approve);
    }

    pub fn get_exchange_approve(&self) -> u64 {
        self.exchange_approve_list.get(&ic_cdk::caller()).unwrap().clone()
    }

    pub fn set_exchange_record(&mut self, record: ExchangeRecord) {
        self.exchange_record.push(record);
    }

    pub fn get_exchange_record(&self) -> Vec<ExchangeRecord> {
        self.exchange_record.clone()
    }

    pub fn set_owner(&mut self, principal: Principal) -> () {
        self.owner = Some(principal);
    }

    pub fn get_owner(&self) -> Option<Principal> {
        self.owner
    }

    pub fn is_owner(&self) -> Result<(), String> {
        if self.owner.unwrap() != ic_cdk::caller() {
            return  Err("no auth".to_owned());
         }
         Ok(())
    }
}
