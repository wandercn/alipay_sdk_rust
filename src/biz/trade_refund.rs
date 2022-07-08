#![allow(unused)]
use super::{BizContenter, BizObject};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct TradeRefundBiz(BizObject);

impl BizContenter for TradeRefundBiz {
    fn method(&self) -> String {
        "alipay.trade.refund".to_string()
    }
    // 设置可选字段方法
    fn set(&mut self, key: &str, value: &str) {
        self.0.insert(key.to_string(), value.to_string());
    }
}
// 以下是设置必选字段方法
impl TradeRefundBiz {
    pub fn new() -> Self {
        Self(BizObject::new())
    }

    pub fn set_out_trade_no(&mut self, value: &str) {
        self.set("out_trade_no", value);
    }

    pub fn set_trade_no(&mut self, value: &str) {
        self.set("trade_no", value);
    }

    pub fn set_refund_amount(&mut self, value: &str) {
        self.set("refund_amount", value)
    }
}
