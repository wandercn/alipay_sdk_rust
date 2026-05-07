use super::{BizContenterV3, BizObjectV3, V3};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct TradeRefundV3Biz(BizObjectV3);

impl BizContenterV3 for TradeRefundV3Biz {
    fn path(&self) -> String {
        "/v3/alipay/trade/refund".to_string()
    }
    fn set(&mut self, key: &str, value: V3) {
        self.0.insert(key.to_string(), value.clone());
    }
}

impl TradeRefundV3Biz {
    pub fn new() -> Self {
        Self(BizObjectV3::new())
    }

    pub fn set_out_trade_no(&mut self, value: V3) {
        self.set("out_trade_no", value);
    }

    pub fn set_trade_no(&mut self, value: V3) {
        self.set("trade_no", value);
    }

    pub fn set_refund_amount(&mut self, value: V3) {
        self.set("refund_amount", value);
    }

    pub fn set_out_request_no(&mut self, value: V3) {
        self.set("out_request_no", value);
    }

    pub fn set_refund_reason(&mut self, value: V3) {
        self.set("refund_reason", value);
    }
}
