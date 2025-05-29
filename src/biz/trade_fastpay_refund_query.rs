#![allow(unused)]
use std::hash::BuildHasher;
use jsonmap::{JsonMap, JsonV};
use super::{BizContenter, BizObject, V};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct TradeFastpayRefundQueryBiz(BizObject);

impl BizContenter for TradeFastpayRefundQueryBiz {
    fn method(&self) -> String {
        "alipay.trade.fastpay.refund.query".to_string()
    }
    // 设置可选字段方法
    fn set(&mut self, key: &str, value: V) {
        self.0.insert(key.to_string(), value.clone());
    }

}
// 以下是设置必选字段方法
impl TradeFastpayRefundQueryBiz {
    pub fn new() -> Self {
        Self(BizObject::new())
    }

    pub fn set_trade_no(&mut self, value: V) {
        self.set("trade_no", value);
    }

    pub fn set_out_trade_no(&mut self, value: V) {
        self.set("out_trade_no", value);
    }

    pub fn set_out_request_no(&mut self, value: V) {
        self.set("out_request_no", value);
    }
}
