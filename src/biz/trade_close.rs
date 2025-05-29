#![allow(unused)]
use std::hash::BuildHasher;
use jsonmap::{JsonMap, JsonV};
use super::{BizContenter, BizObject, V};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct TradeCloseBiz(BizObject);

impl BizContenter for TradeCloseBiz

{
    fn method(&self) -> String {
        "alipay.trade.close".to_string()
    }
    // 设置可选字段方法
    fn set(&mut self, key: &str, value: V) {
        self.0.insert(key.to_string(), value.clone());
    }

}
// 以下是设置必选字段方法
impl TradeCloseBiz

{
    pub fn new() -> Self {
        Self(BizObject::new())
    }

    pub fn set_trade_no(&mut self, value: V) {
        self.set("trade_no", value);
    }

    pub fn set_out_trade_no(&mut self, value: V) {
        self.set("out_trade_no", value);
    }
}
