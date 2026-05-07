use super::{BizContenterV3, BizObjectV3, V3};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct TradeOrderSettleQueryV3Biz(BizObjectV3);

impl BizContenterV3 for TradeOrderSettleQueryV3Biz {
    fn path(&self) -> String {
        "/v3/alipay/trade/order/settle/query".to_string()
    }
    fn set(&mut self, key: &str, value: V3) {
        self.0.insert(key.to_string(), value.clone());
    }
}

impl TradeOrderSettleQueryV3Biz {
    pub fn new() -> Self {
        Self(BizObjectV3::new())
    }

    pub fn set_settle_no(&mut self, value: V3) {
        self.set("settle_no", value);
    }

    pub fn set_out_request_no(&mut self, value: V3) {
        self.set("out_request_no", value);
    }

    pub fn set_trade_no(&mut self, value: V3) {
        self.set("trade_no", value);
    }
}
