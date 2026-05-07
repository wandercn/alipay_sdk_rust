use super::{AccountInfoV3, BizContenterV3, BizObjectV3, V3};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct TradeRoyaltyRelationUnbindV3Biz(BizObjectV3);

impl BizContenterV3 for TradeRoyaltyRelationUnbindV3Biz {
    fn path(&self) -> String {
        "/v3/alipay/trade/royalty/relation/unbind".to_string()
    }
    fn set(&mut self, key: &str, value: V3) {
        self.0.insert(key.to_string(), value.clone());
    }
}

impl TradeRoyaltyRelationUnbindV3Biz {
    pub fn new() -> Self {
        Self(BizObjectV3::new())
    }

    pub fn set_out_request_no(&mut self, value: V3) {
        self.set("out_request_no", value);
    }

    pub fn set_trade_no(&mut self, value: V3) {
        self.set("trade_no", value);
    }

    pub fn set_biz_type(&mut self, value: V3) {
        self.set("biz_type", value);
    }

    pub fn set_receiver_list(&mut self, receiver_list: Vec<AccountInfoV3>) {
        self.set(
            "receiver_list",
            V3::Array(receiver_list.into_iter().map(|r| r.into_map()).collect()),
        );
    }
}
