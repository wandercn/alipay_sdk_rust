use super::{BizContenterV3, BizObjectV3, V3};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct TradeCreateV3Biz(BizObjectV3);

impl BizContenterV3 for TradeCreateV3Biz {
    fn path(&self) -> String {
        "/v3/alipay/trade/create".to_string()
    }
    fn set(&mut self, key: &str, value: V3) {
        self.0.insert(key.to_string(), value.clone());
    }
}

impl TradeCreateV3Biz {
    pub fn new() -> Self {
        Self(BizObjectV3::new())
    }

    pub fn set_out_trade_no(&mut self, value: V3) {
        self.set("out_trade_no", value);
    }

    pub fn set_total_amount(&mut self, value: V3) {
        self.set("total_amount", value);
    }

    pub fn set_subject(&mut self, value: V3) {
        self.set("subject", value);
    }

    pub fn set_buyer_id(&mut self, value: V3) {
        self.set("buyer_id", value);
    }

    pub fn set_product_code(&mut self, value: V3) {
        self.set("product_code", value);
    }
}
