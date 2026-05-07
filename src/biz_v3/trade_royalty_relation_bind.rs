use super::{BizContenterV3, BizObjectV3, V3};
use jsonmap::JsonMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct AccountInfoV3 {
    pub login_name: String,
    pub name: String,
    pub memo: String,
    pub account_open_id: String,
    pub bind_login_name: String,
    pub account_type: String,
    pub account_number: String,
}

impl AccountInfoV3 {
    pub fn init_with(
        login_name: String,
        name: String,
        memo: String,
        account_open_id: String,
        bind_login_name: String,
        account_type: String,
        account_number: String,
    ) -> Self {
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
    pub fn into_map(self) -> V3 {
        let mut map = JsonMap::new();
        map.insert("login_name".to_string(), self.login_name.into());
        map.insert("name".to_string(), self.name.into());
        map.insert("memo".to_string(), self.memo.into());
        map.insert("account_open_id".to_string(), self.account_open_id.into());
        map.insert("bind_login_name".to_string(), self.bind_login_name.into());
        map.insert("type".to_string(), self.account_type.into());
        map.insert("account_number".to_string(), self.account_number.into());
        V3::Object(map)
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct TradeRoyaltyRelationBindV3Biz(BizObjectV3);

impl BizContenterV3 for TradeRoyaltyRelationBindV3Biz {
    fn path(&self) -> String {
        "/v3/alipay/trade/royalty/relation/bind".to_string()
    }
    fn set(&mut self, key: &str, value: V3) {
        self.0.insert(key.to_string(), value.clone());
    }
}

impl TradeRoyaltyRelationBindV3Biz {
    pub fn new() -> Self {
        Self(BizObjectV3::new())
    }

    pub fn set_out_request_no(&mut self, value: V3) {
        self.set("out_request_no", value);
    }

    pub fn set_trade_no(&mut self, value: V3) {
        self.set("trade_no", value);
    }

    pub fn set_biz_type(&mut self, value: V3) {
        self.set("biz_type", value);
    }

    pub fn set_receiver_list(&mut self, receiver_list: Vec<AccountInfoV3>) {
        self.set(
            "receiver_list",
            V3::Array(receiver_list.into_iter().map(|r| r.into_map()).collect()),
        );
    }
}
