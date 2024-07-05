#![allow(unused)]
use std::hash::BuildHasher;

use super::{BizContenter, BizObject};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct TradePrecreateBiz<V>(BizObject<V>);

impl<V> BizContenter<V> for TradePrecreateBiz<V>
where
    V: Serialize + Clone  ,
{
    fn method(&self) -> String {
        "alipay.trade.precreate".to_string()
    }
    // 设置可选字段方法
    fn set(&mut self, key: &str, value: V) {
        self.0.insert(key.to_string(), value.clone());
    }
}
// 以下是设置必选字段方法
impl<V> TradePrecreateBiz<V>
where
    V: Serialize + Clone  ,
{
    pub fn new() -> Self {
        Self(BizObject::new())
    }

    pub fn set_out_trade_no(&mut self, value: V) {
        self.set("out_trade_no", value);
    }

    pub fn set_total_amount(&mut self, value: V) {
        self.set("total_amount", value);
    }

    pub fn set_subject(&mut self, value: V) {
        self.set("subject", value);
    }
}
