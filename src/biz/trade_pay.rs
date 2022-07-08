#![allow(unused)]
use super::{BizContenter, BizObject};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct TradePayBiz(BizObject);

impl BizContenter for TradePayBiz {
    fn method(&self) -> String {
        "alipay.trade.pay".to_string()
    }
    // 设置可选字段方法
    fn set(&mut self, key: &str, value: &str) {
        self.0.insert(key.to_string(), value.to_string());
    }
}
// 以下是设置必选字段方法
impl TradePayBiz {
    pub fn new() -> Self {
        Self(BizObject::new())
    }

    pub fn set_outtrade_no(&mut self, value: &str) {
        self.set("out_trade_no", value);
    }

    pub fn set_scene(&mut self, value: &str) {
        self.set("scene", value);
    }

    pub fn set_auth_code(&mut self, value: &str) {
        self.set("auth_code", value);
    }

    pub fn set_subject(&mut self, value: &str) {
        self.set("subject", value);
    }
}
