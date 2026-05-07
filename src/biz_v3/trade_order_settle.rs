use super::{BizContenterV3, BizObjectV3, V3};
use jsonmap::JsonMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct RoyaltyParameterV3 {
    pub royalty_type: String,
    pub trans_in_type: String,
    pub trans_in: String,
    pub amount: String,
    pub desc: String,
}

impl RoyaltyParameterV3 {
    pub fn init_with(trans_in: String, amount: String, desc: String) -> Self {
        Self {
            royalty_type: "transfer".to_string(),
            trans_in_type: "userId".to_string(),
            trans_in,
            amount,
            desc,
        }
    }
    pub fn into_map(self) -> V3 {
        let mut map = JsonMap::new();
        map.insert("royalty_type".to_string(), self.royalty_type.into());
        map.insert("trans_in_type".to_string(), self.trans_in_type.into());
        map.insert("trans_in".to_string(), self.trans_in.into());
        map.insert("amount".to_string(), self.amount.into());
        map.insert("desc".to_string(), self.desc.into());
        V3::Object(map)
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct TradeOrderSettleV3Biz(BizObjectV3);

impl BizContenterV3 for TradeOrderSettleV3Biz {
    fn path(&self) -> String {
        "/v3/alipay/trade/order/settle".to_string()
    }
    fn set(&mut self, key: &str, value: V3) {
        self.0.insert(key.to_string(), value.clone());
    }
}

impl TradeOrderSettleV3Biz {
    pub fn new() -> Self {
        Self(BizObjectV3::new())
    }

    pub fn set_out_trade_no(&mut self, value: V3) {
        self.set("out_trade_no", value);
    }

    pub fn set_trade_no(&mut self, value: V3) {
        self.set("trade_no", value);
    }

    pub fn set_royalty_parameters(&mut self, value: Vec<RoyaltyParameterV3>) {
        self.set(
            "royalty_parameters",
            V3::Array(value.into_iter().map(|v| v.into_map()).collect()),
        );
    }
}
