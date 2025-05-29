#![allow(unused)]

use std::collections::HashMap;
use std::hash::BuildHasher;
use jsonmap::{JsonMap, JsonV};
use super::{BizContenter, BizObject, V};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct TradeOrderSettleQueryBiz(BizObject);

impl BizContenter for TradeOrderSettleQueryBiz
{
    fn method(&self) -> String {
        "_".to_string()
    }
    // 设置可选字段方法
    fn set(&mut self, key: &str, value: V) {
        self.0.insert(key.to_string(), value.clone());
    }

    fn set_array(&mut self, key: &str, value: Vec<JsonV<String>>) {
        self.0.insert(key.to_string(), JsonV::from(value));
    }
}

impl TradeOrderSettleQueryBiz {
    pub fn new() -> Self {
        Self(BizObject::new())
    }
 
    pub fn set_settle_no(&mut self, settle_no: String) {
        self.set("settle_no", settle_no.into());
    }
    pub fn set_out_request_no(&mut self, out_request_no: String) {
        self.set("out_request_no", out_request_no.into());
    }
    pub fn set_trade_no(&mut self, trade_no: String) {
        self.set("trade_no", trade_no.into());
    }

}