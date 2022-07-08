#![allow(unused)]
use super::{BizContenter, BizObject};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct TradeWapPayBiz(BizObject);

impl BizContenter for TradeWapPayBiz {
    fn method(&self) -> String {
        "alipay.trade.wap.pay".to_string()
    }
    // 设置可选字段方法
    fn set(&mut self, key: &str, value: &str) {
        self.0.insert(key.to_string(), value.to_string());
    }
}
// 以下是设置必选字段方法
impl TradeWapPayBiz {
    pub fn new() -> Self {
        Self(BizObject::new())
    }

    pub fn set_subject(&mut self, value: &str) {
        self.set("subject", value);
    }

    pub fn set_outtrade_no(&mut self, value: &str) {
        self.set("out_trade_no", value);
    }

    pub fn set_total_amount(&mut self, value: &str) {
        self.set("total_amount", value);
    }

    pub fn set_quit_url(&mut self, value: &str) {
        self.set("quit_url", value);
    }

    pub fn set_product_code(&mut self, value: &str) {
        self.set("product_code", value);
    }

    pub fn set_return_url(&mut self, value: &str) {
        self.set("return_url", value);
    }
}
