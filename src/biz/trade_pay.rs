#![allow(unused)]
use std::hash::BuildHasher;

use super::{BizContenter, BizObject, V};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct TradePayBiz(BizObject);

impl BizContenter for TradePayBiz {
    fn method(&self) -> String {
        "alipay.trade.pay".to_string()
    }
    // 设置可选字段方法
    fn set(&mut self, key: &str, value: V) {
        self.0.insert(key.to_string(), value.clone());
    }
}
// 以下是设置必选字段方法
impl TradePayBiz {
    pub fn new() -> Self {
        Self(BizObject::new())
    }

    pub fn set_outtrade_no(&mut self, value: V) {
        self.set("out_trade_no", value);
    }

    pub fn set_scene(&mut self, value: V) {
        self.set("scene", value);
    }

    pub fn set_auth_code(&mut self, value: V) {
        self.set("auth_code", value);
    }

    pub fn set_subject(&mut self, value: V) {
        self.set("subject", value);
    }
}
