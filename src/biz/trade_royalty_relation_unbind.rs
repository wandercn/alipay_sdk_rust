use jsonmap::{JsonMap, JsonV};
use serde::{Deserialize, Serialize};
use crate::biz::{AccountInfo, BizContenter, BizObject, V};

#[derive(Serialize, Deserialize, Default)]
pub struct TradeRoyaltyRelationUnBindBiz(BizObject);

impl BizContenter for TradeRoyaltyRelationUnBindBiz

{
    fn method(&self) -> String {
        "_".to_string()
    }
    // 设置可选字段方法
    fn set(&mut self, key: &str, value: V) {
        self.0.insert(key.to_string(), value.clone());
    }

}

impl TradeRoyaltyRelationUnBindBiz {
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