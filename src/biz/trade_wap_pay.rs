#![allow(unused)]
use std::hash::BuildHasher;
use jsonmap::{JsonMap, JsonV};
use super::{BizContenter, BizObject, V};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct TradeWapPayBiz(BizObject);

impl BizContenter for TradeWapPayBiz {
    fn method(&self) -> String {
        "alipay.trade.wap.pay".to_string()
    }
    // 设置可选字段方法
    fn set(&mut self, key: &str, value: V) {
        self.0.insert(key.to_string(), value.clone());
    }

    fn set_array(&mut self, key: &str, value: Vec<JsonV<String>>) {
        todo!()
    }
}
// 以下是设置必选字段方法
impl TradeWapPayBiz {
    pub fn new() -> Self {
        Self(BizObject::new())
    }

    pub fn set_subject(&mut self, value: V) {
        self.set("subject", value);
    }

    pub fn set_outtrade_no(&mut self, value: V) {
        self.set("out_trade_no", value);
    }

    pub fn set_total_amount(&mut self, value: V) {
        self.set("total_amount", value);
    }

    pub fn set_quit_url(&mut self, value: V) {
        self.set("quit_url", value);
    }

    pub fn set_product_code(&mut self, value: V) {
        self.set("product_code", value);
    }

    pub fn set_return_url(&mut self, value: V) {
        self.set("return_url", value);
    }
}
