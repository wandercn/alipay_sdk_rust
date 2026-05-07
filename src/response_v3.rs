use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
pub struct TradePayV3Response {
    pub code: String,
    pub msg: String,
    pub sub_code: Option<String>,
    pub sub_msg: Option<String>,
    pub trade_no: Option<String>,
    pub out_trade_no: Option<String>,
    pub buyer_logon_id: Option<String>,
    pub total_amount: Option<String>,
    pub gmt_payment: Option<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
pub struct TradeCreateV3Response {
    pub code: String,
    pub msg: String,
    pub sub_code: Option<String>,
    pub sub_msg: Option<String>,
    pub trade_no: Option<String>,
    pub out_trade_no: Option<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
pub struct TradePrecreateV3Response {
    pub code: String,
    pub msg: String,
    pub sub_code: Option<String>,
    pub sub_msg: Option<String>,
    pub out_trade_no: Option<String>,
    pub qr_code: Option<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
pub struct TradeQueryV3Response {
    pub code: String,
    pub msg: String,
    pub sub_code: Option<String>,
    pub sub_msg: Option<String>,
    pub trade_no: Option<String>,
    pub out_trade_no: Option<String>,
    pub buyer_logon_id: Option<String>,
    pub trade_status: Option<String>,
    pub total_amount: Option<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
pub struct TradeRefundV3Response {
    pub code: String,
    pub msg: String,
    pub sub_code: Option<String>,
    pub sub_msg: Option<String>,
    pub trade_no: Option<String>,
    pub out_trade_no: Option<String>,
    pub buyer_logon_id: Option<String>,
    pub refund_fee: Option<String>,
    pub gmt_refund_pay: Option<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
pub struct TradeCancelV3Response {
    pub code: String,
    pub msg: String,
    pub sub_code: Option<String>,
    pub sub_msg: Option<String>,
    pub trade_no: Option<String>,
    pub out_trade_no: Option<String>,
    pub retry_flag: Option<String>,
    pub action: Option<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
pub struct TradeCloseV3Response {
    pub code: String,
    pub msg: String,
    pub sub_code: Option<String>,
    pub sub_msg: Option<String>,
    pub trade_no: Option<String>,
    pub out_trade_no: Option<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
pub struct TradeFastpayRefundQueryV3Response {
    pub code: String,
    pub msg: String,
    pub sub_code: Option<String>,
    pub sub_msg: Option<String>,
    pub trade_no: Option<String>,
    pub out_trade_no: Option<String>,
    pub out_request_no: Option<String>,
    pub refund_reason: Option<String>,
    pub total_amount: Option<String>,
    pub refund_amount: Option<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
pub struct TradeOrderSettleV3Response {
    pub code: String,
    pub msg: String,
    pub sub_code: Option<String>,
    pub sub_msg: Option<String>,
    pub trade_no: Option<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
pub struct TradeOrderSettleQueryV3Response {
    pub code: String,
    pub msg: String,
    pub sub_code: Option<String>,
    pub sub_msg: Option<String>,
    pub out_request_no: Option<String>,
    pub trade_no: Option<String>,
    pub settle_no: Option<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
pub struct TradeRoyaltyRelationBindV3Response {
    pub code: String,
    pub msg: String,
    pub sub_code: Option<String>,
    pub sub_msg: Option<String>,
    pub result_code: Option<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
pub struct TradeRoyaltyRelationUnbindV3Response {
    pub code: String,
    pub msg: String,
    pub sub_code: Option<String>,
    pub sub_msg: Option<String>,
    pub result_code: Option<String>,
}
