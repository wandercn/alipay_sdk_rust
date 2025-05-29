use jsonmap::{JsonMap, JsonV};
use serde::{Deserialize, Serialize};
use crate::biz::{BizContenter, BizObject, V};


#[derive(Debug, Clone)]
pub struct AccountInfo {
    pub login_name: String,
    pub name: String,
    pub memo: String,
    pub account_open_id: String,
    pub bind_login_name: String,
    pub account_type: String,  // 避免与Rust关键字冲突
    pub account_number: String,  // 更清晰的字段名
}
impl AccountInfo {
    pub fn init_with(login_name: String, name: String, memo: String, account_open_id: String, bind_login_name: String, account_type: String, account_number: String) -> Self {
        Self {
            login_name,
            name,
            memo,
            account_open_id,
            bind_login_name,
            account_type,
            account_number,
        }
    }
    pub fn into_map(self) -> JsonV<String> {
        let mut map = JsonMap::new();
        map.insert("login_name".to_string(), self.login_name.into());
        map.insert("name".to_string(), self.name.into());
        map.insert("memo".to_string(), self.memo.into());
        map.insert("account_open_id".to_string(), self.account_open_id.into());
        map.insert("bind_login_name".to_string(), self.bind_login_name.into());
        map.insert("type".to_string(), self.account_type.into());
        map.insert("account_number".to_string(), self.account_number.into());
        JsonV::from(map)
    }
}


#[derive(Serialize, Deserialize, Default)]
pub struct TradeRoyaltyRelationBindBiz(BizObject);

impl BizContenter for TradeRoyaltyRelationBindBiz

{
    fn method(&self) -> String {
        "_".to_string()
    }
    // 设置可选字段方法
    fn set(&mut self, key: &str, value: V) {
        self.0.insert(key.to_string(), value.clone());
    }
    
}

impl TradeRoyaltyRelationBindBiz {
    pub fn new() -> Self {
        Self(BizObject::new())
    }
    pub fn set_out_request_no(&mut self, out_request_no: String) {
        self.set("out_request_no", out_request_no.into());
    }
    pub fn set_trade_no(&mut self, trade_no: String) {
        self.set("trade_no", trade_no.into());
    }
    pub fn set_biz_type(&mut self, biz_type: String) {
        self.set("biz_type", biz_type.into());
    }
    pub fn set_royalty_parameters(&mut self, receiver_list: Vec<AccountInfo>) {
        self.set("receiver_list", JsonV::Array(receiver_list.iter().map(|receiver| receiver.clone().into_map()).collect::<Vec<JsonV<String>>>()));
    }
}