#![allow(unused)]

use std::collections::HashMap;
use std::hash::BuildHasher;
use jsonmap::{JsonMap, JsonV};
use super::{BizContenter, BizObject, V};
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Clone)]
pub struct RoyaltyParameter{
    pub royalty_type: String,
    pub trans_in_type: String,
    pub trans_in: String,
    pub amount: String,
    pub desc: String,
}

impl RoyaltyParameter {
    pub fn init_with(trans_in: String, amount: String, desc: String) -> Self {
        Self {
            royalty_type: "transfer".to_string(),
            trans_in_type: "userId".to_string(),
            trans_in,
            amount,
            desc,
        }
    }
    pub fn into_map(self) -> JsonV<String> {
        let mut map = JsonMap::new();
        map.insert("royalty_type".to_string(), self.royalty_type.clone().into());
        map.insert("trans_in_type".to_string(), self.trans_in_type.clone().into());
        map.insert("trans_in".to_string(), self.trans_in.clone().into());
        map.insert("amount".to_string(), self.amount.clone().into());
        map.insert("desc".to_string(), self.desc.clone().into());
        JsonV::Object(map)
    }
}


#[derive(Serialize, Deserialize, Default)]
pub struct TradeOrderSettleBiz(BizObject);

impl BizContenter for TradeOrderSettleBiz

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

// 以下是设置必选字段方法
impl TradeOrderSettleBiz
{
    pub fn new() -> Self {
        Self(BizObject::new())
    }
    // 以下是设置必选字段方法

    pub fn set_out_trade_no(&mut self, value: V) {
        self.set("out_trade_no", value);
    }
    
    pub fn set_trade_no(&mut self, value: V) {
        self.set("trade_no", value);
    }
    
    pub fn set_royalty_parameters(&mut self, value: Vec<RoyaltyParameter>) {
        self.set_array("royalty_parameters", value.iter().map(|v| v.clone().into_map()).collect::<Vec<JsonV<String>>>());
    }
    
}


