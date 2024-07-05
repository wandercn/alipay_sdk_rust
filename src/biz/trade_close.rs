#![allow(unused)]
use std::hash::BuildHasher;

use super::{BizContenter, BizObject};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct TradeCloseBiz<V>(BizObject<V>);

impl<V> BizContenter<V> for TradeCloseBiz<V>
where
    V: Serialize + Clone  ,
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
impl<V> TradeCloseBiz<V>
where
    V: Serialize + Clone  ,
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
