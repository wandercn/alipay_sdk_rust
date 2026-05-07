use super::{BizContenterV3, BizObjectV3, V3};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct TradeQueryV3Biz(BizObjectV3);

impl BizContenterV3 for TradeQueryV3Biz {
    fn path(&self) -> String {
        "/v3/alipay/trade/query".to_string()
    }
    fn set(&mut self, key: &str, value: V3) {
        self.0.insert(key.to_string(), value.clone());
    }
}

impl TradeQueryV3Biz {
    pub fn new() -> Self {
        Self(BizObjectV3::new())
    }

    pub fn set_out_trade_no(&mut self, value: V3) {
        self.set("out_trade_no", value);
    }

    pub fn set_trade_no(&mut self, value: V3) {
        self.set("trade_no", value);
    }
}
