//! 返回值结构体定义。
use serde_with::skip_serializing_none;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Response<T>(HashMap<String, T>);

#[skip_serializing_none]
#[derive(Serialize, Deserialize)]
pub struct TradePayResponse {
    #[serde(rename(deserialize = "alipay_trade_pay_response"))]
    pub response: PayResponse,
    pub sign: Option<String>,
    pub alipay_cert_sn: Option<String>,
}
#[skip_serializing_none]
#[derive(Serialize, Deserialize)]
pub struct PayResponse {
    pub code: Option<String>,
    pub msg: Option<String>,
    pub sub_code: Option<String>,
    pub sub_msg: Option<String>,
    pub trade_no: Option<String>,
    pub out_trade_no: Option<String>,
    pub buyer_logon_id: Option<String>,
    pub settle_amount: Option<String>,
    pub pay_currency: Option<String>,
    pub pay_amount: Option<String>,
    pub settle_trans_rate: Option<String>,
    pub trans_pay_rate: Option<String>,
    pub total_amount: Option<String>,
    pub trans_currency: Option<String>,
    pub settle_currency: Option<String>,
    pub receipt_amount: Option<String>,
    pub buyer_pay_amount: Option<String>,
    pub point_amount: Option<String>,
    pub invoice_amount: Option<String>,
    pub gmt_payment: Option<String>,
    pub fund_bill_list: Option<Vec<FundBill>>,
    pub card_balance: Option<String>,
    pub store_name: Option<String>,
    pub buyer_user_id: Option<String>,
    pub discount_goods_detail: Option<String>,
    pub voucher_detail_list: Option<Vec<VoucherDetail>>,
    pub advance_amount: Option<String>,
    pub auth_trade_pay_mode: Option<String>,
    pub charge_amount: Option<String>,
    pub charge_flags: Option<String>,
    pub settlement_id: Option<String>,
    pub business_params: Option<String>,
    pub buyer_user_type: Option<String>,
    pub mdiscount_amount: Option<String>,
    pub discount_amount: Option<String>,
    pub buyer_user_name: Option<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize)]
pub struct FundBill {
    pub fund_channel: Option<String>,
    pub bank_code: Option<String>,
    pub amount: Option<String>,
    pub real_amount: Option<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize)]
pub struct VoucherDetail {
    pub id: Option<String>,
    pub name: Option<String>,
    pub r#type: Option<String>,
    pub amount: Option<String>,
    pub merchant_contribute: Option<String>,
    pub other_contribute: Option<String>,
    pub memo: Option<String>,
    pub template_id: Option<String>,
    pub purchase_buyer_contribute: Option<String>,
    pub purchase_merchant_contribute: Option<String>,
    pub purchase_ant_contribute: Option<String>,
}

//===================================================
#[skip_serializing_none]
#[derive(Serialize, Deserialize)]
pub struct TradeQueryResponse {
    #[serde(rename(deserialize = "alipay_trade_query_response"))]
    pub response: QueryResponse,
    pub sign: Option<String>,
    pub alipay_cert_sn: Option<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize)]
pub struct QueryResponse {
    pub code: Option<String>,
    pub msg: Option<String>,
    pub sub_code: Option<String>,
    pub sub_msg: Option<String>,
    pub trade_no: Option<String>,
    pub out_trade_no: Option<String>,
    pub buyer_logon_id: Option<String>,
    pub trade_status: Option<String>,
    pub total_amount: Option<String>,
    pub trans_currency: Option<String>,
    pub settle_currency: Option<String>,
    pub settle_amount: Option<String>,
    pub pay_currency: Option<String>,
    pub pay_amount: Option<String>,
    pub settle_trans_rate: Option<String>,
    pub trans_pay_rate: Option<String>,
    pub buyer_pay_amount: Option<String>,
    pub point_amount: Option<String>,
    pub invoice_amount: Option<String>,
    pub send_pay_date: Option<String>,
    pub receipt_amount: Option<String>,
    pub store_id: Option<String>,
    pub terminal_id: Option<String>,
    pub fund_bill_list: Option<Vec<FundBill>>,
    pub store_name: Option<String>,
    pub buyer_user_id: Option<String>,
    pub charge_amount: Option<String>,
    pub charge_flags: Option<String>,
    pub settlement_id: Option<String>,
    pub trade_settle_info: Option<Vec<TradeSettleInfo>>,
    pub auth_trade_pay_mode: Option<String>,
    pub buyer_user_type: Option<String>,
    pub mdiscount_amount: Option<String>,
    pub discount_amount: Option<String>,
    pub buyer_user_name: Option<String>,
    pub subject: Option<String>,
    pub body: Option<String>,
    pub alipay_sub_merchant_id: Option<String>,
    pub ext_infos: Option<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize)]
pub struct TradeSettleInfo {
    pub trade_settle_detail_list: Vec<TradeSettleDetail>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize)]
pub struct TradeSettleDetail {
    pub operation_type: Option<String>,
    pub operation_serial_no: Option<String>,
    pub operation_dt: Option<String>,
    pub trans_out: Option<String>,
    pub trans_in: Option<String>,
    pub amount: Option<String>,
}

//===================================================
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct TradeCreateResponse {
    #[serde(rename(deserialize = "alipay_trade_create_response"))]
    pub response: CreateResponse,
    pub sign: Option<String>,
    pub alipay_cert_sn: Option<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Default, Debug, Clone)]

pub struct CreateResponse {
    pub code: Option<String>,
    pub msg: Option<String>,
    pub sub_code: Option<String>,
    pub sub_msg: Option<String>,
    pub trade_no: Option<String>,
    pub out_trade_no: Option<String>,
}

//===================================================
#[skip_serializing_none]
#[derive(Serialize, Deserialize)]
pub struct TradeCloseResponse {
    #[serde(rename(deserialize = "alipay_trade_close_response"))]
    pub response: CloseResponse,
    pub sign: Option<String>,
    pub alipay_cert_sn: Option<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize)]
pub struct CloseResponse {
    pub code: Option<String>,
    pub msg: Option<String>,
    pub sub_code: Option<String>,
    pub sub_msg: Option<String>,
    pub trade_no: Option<String>,
    pub out_trade_no: Option<String>,
}

//===================================================
#[skip_serializing_none]
#[derive(Serialize, Deserialize)]
pub struct TradeCancelResponse {
    #[serde(rename(deserialize = "alipay_trade_cancel_response"))]
    pub response: CancelResponse,
    pub alipay_cert_sn: Option<String>,
    pub sign: Option<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize)]
pub struct CancelResponse {
    pub code: Option<String>,
    pub msg: Option<String>,
    pub sub_code: Option<String>,
    pub sub_msg: Option<String>,
    pub trade_no: Option<String>,
    pub out_trade_no: Option<String>,
    pub retry_flag: Option<String>,
    pub action: Option<String>,
    pub gmt_refund_pay: Option<String>,
    pub refund_settlement_id: Option<String>,
}

//===================================================
#[skip_serializing_none]
#[derive(Serialize, Deserialize)]
pub struct SystemOauthTokenResponse {
    #[serde(rename(deserialize = "alipay_system_oauth_token_response"))]
    response: OauthTokenInfo,
    alipay_cert_sn: Option<String>,
    sign: Option<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize)]
struct OauthTokenInfo {
    access_token: Option<String>,
    alipay_user_id: Option<String>,
    expires_in: Option<i32>,
    re_expires_in: Option<i32>,
    refresh_token: Option<String>,
    user_id: Option<String>,
}

//===================================================

#[skip_serializing_none]
#[derive(Serialize, Deserialize)]
pub struct UserInfoShareResponse {
    #[serde(rename(deserialize = "alipay_user_info_share_response"))]
    response: UserInfoShare,
    alipay_cert_sn: Option<String>,
    sign: Option<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize)]
struct UserInfoShare {
    code: Option<String>,
    msg: Option<String>,
    sub_code: Option<String>,
    sub_msg: Option<String>,
    user_id: Option<String>,
    avatar: Option<String>,
    province: Option<String>,
    city: Option<String>,
    nick_name: Option<String>,
    is_student_certified: Option<String>,
    user_type: Option<String>,
    user_status: Option<String>,
    is_certified: Option<String>,
    gender: Option<String>,
}

//===================================================
#[skip_serializing_none]
#[derive(Serialize, Deserialize)]
pub struct TradeRefundResponse {
    #[serde(rename(deserialize = "alipay_trade_refund_response"))]
    pub response: RefundResponse,
    pub alipay_cert_sn: Option<String>,
    pub sign: Option<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize)]
pub struct RefundResponse {
    pub code: Option<String>,
    pub msg: Option<String>,
    pub sub_code: Option<String>,
    pub sub_msg: Option<String>,
    pub trade_no: Option<String>,
    pub out_trade_no: Option<String>,
    pub buyer_logon_id: Option<String>,
    pub fund_change: Option<String>,
    pub refund_fee: Option<String>,
    pub refund_currency: Option<String>,
    pub gmt_refund_pay: Option<String>,
    pub refund_detail_item_list: Option<Vec<TradeFundBill>>,
    pub store_name: Option<String>,
    pub buyer_user_id: Option<String>,
    pub refund_preset_paytool_list: Option<Vec<RefundPresetPaytool>>,
    pub refund_settlement_id: Option<String>,
    pub present_refund_buyer_amount: Option<String>,
    pub present_refund_discount_amount: Option<String>,
    pub present_refund_mdiscount_amount: Option<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize)]
pub struct RefundPresetPaytool {
    pub amount: Option<Vec<String>>,
    pub assert_type_code: Option<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize)]
pub struct TradeFundBill {
    pub fund_channel: Option<String>,
    pub bank_code: Option<String>,
    pub amount: Option<String>,
    pub real_amount: Option<String>,
    pub fund_type: Option<String>,
}

//===================================================
#[skip_serializing_none]
#[derive(Serialize, Deserialize)]
pub struct TradeFastpayRefundQueryResponse {
    #[serde(rename(deserialize = "alipay_trade_fastpay_refund_query_response"))]
    pub response: RefundQueryResponse,
    pub alipay_cert_sn: Option<String>,
    pub sign: Option<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize)]
pub struct RefundQueryResponse {
    pub code: Option<String>,
    pub msg: Option<String>,
    pub sub_code: Option<String>,
    pub sub_msg: Option<String>,
    pub trade_no: Option<String>,
    pub out_trade_no: Option<String>,
    pub out_request_no: Option<String>,
    pub refund_reason: Option<String>,
    pub total_amount: Option<String>,
    pub refund_amount: Option<String>,
    pub refund_royaltys: Option<Vec<RefundRoyalty>>,
    pub gmt_refund_pay: Option<String>,
    pub refund_detail_item_list: Option<Vec<TradeFundBill>>,
    pub send_back_fee: Option<String>,
    pub refund_settlement_id: Option<String>,
    pub present_refund_buyer_amount: Option<String>,
    pub present_refund_discount_amount: Option<String>,
    pub present_refund_mdiscount_amount: Option<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize)]
pub struct RefundRoyalty {
    pub refund_amount: Option<String>,
    pub royalty_type: Option<String>,
    pub result_code: Option<String>,
    pub trans_out: Option<String>,
    pub trans_out_email: Option<String>,
    pub trans_in: Option<String>,
    pub trans_in_email: Option<String>,
}
//===================================================
#[skip_serializing_none]
#[derive(Serialize, Deserialize)]
pub struct TradeOrderSettleResponse {
    #[serde(rename(deserialize = "alipay_trade_order_settle_response"))]
    pub response: OrderSettleResponse,
    pub alipay_cert_sn: Option<String>,
    pub sign: Option<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize)]
pub struct OrderSettleResponse {
    pub code: Option<String>,
    pub msg: Option<String>,
    pub sub_code: Option<String>,
    pub sub_msg: Option<String>,
    pub trade_no: Option<String>,
}

//===================================================
#[skip_serializing_none]
#[derive(Serialize, Deserialize)]
pub struct TradePrecreateResponse {
    #[serde(rename(deserialize = "alipay_trade_precreate_response"))]
    pub response: PrecreateResponse,
    pub alipay_cert_sn: Option<String>,
    pub sign: Option<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize)]
pub struct PrecreateResponse {
    pub code: Option<String>,
    pub msg: Option<String>,
    pub sub_code: Option<String>,
    pub sub_msg: Option<String>,
    pub out_trade_no: Option<String>,
    pub qr_code: Option<String>,
}

//===================================================
#[skip_serializing_none]
#[derive(Serialize, Deserialize)]
pub struct TradePageRefundResponse {
    #[serde(rename(deserialize = "alipay_trade_page_refund_response"))]
    pub response: PageRefundResponse,
    pub alipay_cert_sn: Option<String>,
    pub sign: Option<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize)]
pub struct PageRefundResponse {
    pub code: Option<String>,
    pub msg: Option<String>,
    pub sub_code: Option<String>,
    pub sub_msg: Option<String>,
    pub trade_no: Option<String>,
    pub out_trade_no: Option<String>,
    pub out_request_no: Option<String>,
    pub refund_amount: Option<String>,
}

//===================================================
#[skip_serializing_none]
#[derive(Serialize, Deserialize)]
pub struct FundTransToaccountTransferResponse {
    #[serde(rename(deserialize = "alipay_fund_trans_toaccount_transfer_response"))]
    response: TransToaccountTransferResponse,
    alipay_cert_sn: Option<String>,
    sign: Option<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize)]
struct TransToaccountTransferResponse {
    code: Option<String>,
    msg: Option<String>,
    sub_code: Option<String>,
    sub_msg: Option<String>,
    out_biz_no: Option<String>,
    order_id: Option<String>,
    pay_date: Option<String>,
}

//===================================================
#[skip_serializing_none]
#[derive(Serialize, Deserialize)]
pub struct ZhimaCreditScoreGetResponse {
    #[serde(rename(deserialize = "zhima_credit_score_get_response"))]
    response: ScoreGetResponse,
    alipay_cert_sn: Option<String>,
    sign: Option<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize)]
struct ScoreGetResponse {
    code: Option<String>,
    msg: Option<String>,
    sub_code: Option<String>,
    sub_msg: Option<String>,
    biz_no: Option<String>,
    zm_score: Option<String>,
}

//===================================================
#[skip_serializing_none]
#[derive(Serialize, Deserialize)]
pub struct OpenAuthTokenAppResponse {
    #[serde(rename(deserialize = "alipay_open_auth_token_app_response"))]
    response: AuthTokenAppResponse,
    alipay_cert_sn: Option<String>,
    sign: Option<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize)]
struct AuthTokenAppResponse {
    code: Option<String>,
    msg: Option<String>,
    sub_code: Option<String>,
    sub_msg: Option<String>,
    user_id: Option<String>,
    auth_app_id: Option<String>,
    app_auth_token: Option<String>,
    app_refresh_token: Option<String>,
    expires_in: Option<i32>,
    re_expires_in: Option<i32>,
    tokens: Option<Vec<Token>>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize)]
struct Token {
    app_auth_token: Option<String>,
    app_refresh_token: Option<String>,
    auth_app_id: Option<String>,
    expires_in: Option<i32>,
    re_expires_in: Option<i32>,
    user_id: Option<String>,
}
//===================================================
#[skip_serializing_none]
#[derive(Serialize, Deserialize)]
pub struct UserCertifyOpenInitResponse {
    #[serde(rename(deserialize = "alipay_user_certify_open_initialize_response"))]
    response: AlipayUserCertifyOpenInitResponse,
    alipay_cert_sn: Option<String>,
    sign: Option<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize)]
struct AlipayUserCertifyOpenInitResponse {
    code: Option<String>,
    msg: Option<String>,
    sub_code: Option<String>,
    sub_msg: Option<String>,
    certify_id: Option<String>,
}

//===================================================
#[skip_serializing_none]
#[derive(Serialize, Deserialize)]
pub struct UserCertifyOpenCertifyResponse {
    #[serde(rename(deserialize = "alipay_user_certify_open_certify_response"))]
    response: AlipayUserCertifyOpenCertifyResponse,
    alipay_cert_sn: Option<String>,
    sign: Option<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize)]
struct AlipayUserCertifyOpenCertifyResponse {
    code: Option<String>,
    msg: Option<String>,
    sub_code: Option<String>,
    sub_msg: Option<String>,
}

//===================================================
#[skip_serializing_none]
#[derive(Serialize, Deserialize)]
pub struct UserCertifyOpenQueryResponse {
    #[serde(rename(deserialize = "alipay_user_certify_open_query_response"))]
    response: AlipayUserCertifyOpenQueryResponse,
    alipay_cert_sn: Option<String>,
    sign: Option<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize)]
struct AlipayUserCertifyOpenQueryResponse {
    code: Option<String>,
    msg: Option<String>,
    sub_code: Option<String>,
    sub_msg: Option<String>,
    passed: Option<Vec<String>>,
    identity_info: Option<String>,
    material_info: Option<String>,
}


#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct RoyaltyDetail {
    #[serde(rename = "operation_type")]
    pub operation_type: String,

    #[serde(rename = "execute_dt")]
    pub execute_datetime: String,

    #[serde(rename = "trans_out")]
    pub transfer_out: String,

    #[serde(rename = "trans_out_type")]
    pub transfer_out_type: String,

    #[serde(rename = "trans_out_open_id")]
    pub transfer_out_open_id: String,

    #[serde(rename = "trans_in")]
    pub transfer_in: String,

    #[serde(rename = "trans_in_open_id")]
    pub transfer_in_open_id: String,

    #[serde(rename = "trans_in_type")]
    pub transfer_in_type: String,

    #[serde(rename = "amount")]
    pub amount: String,

    #[serde(rename = "state")]
    pub state: String,

    #[serde(rename = "detail_id")]
    pub detail_id: String,

    #[serde(rename = "error_code")]
    pub error_code: Option<String>,

    #[serde(rename = "error_desc")]
    pub error_description: Option<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize)]
pub struct AlipayTradeOrderSettleQueryResponse {
    pub code: Option<String>,
    pub msg: Option<String>,
    pub out_request_no: Option<String>,
    pub operation_dt: Option<String>,
    pub royalty_detail_list: Vec<RoyaltyDetail>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize)]
pub struct TradeOrderSettleQueryResponse {
    #[serde(rename(deserialize = "alipay_trade_order_settle_query_response"))]
    pub response: AlipayTradeOrderSettleQueryResponse,
    pub sign: String,
}


#[skip_serializing_none]
#[derive(Serialize, Deserialize)]
pub struct AlipayTraderRoyaltyRelationBindResponse {
    pub code: Option<String>,
    pub msg: Option<String>,
    pub result_code: Option<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize)]
pub struct TradeRoyaltyRelationBindResponse {
    #[serde(rename(deserialize = "alipay_trade_royalty_relation_bind_response"))]
    pub response: AlipayTraderRoyaltyRelationBindResponse,
    pub sign: String,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize)]
pub struct AlipayTraderRoyaltyRelationUnBindResponse {
    pub code: Option<String>,
    pub msg: Option<String>,
    pub result_code: Option<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize)]
pub struct TradeRoyaltyRelationUnBindResponse {
    #[serde(rename(deserialize = "alipay_trade_royalty_relation_unbind_response"))]
    pub response: AlipayTraderRoyaltyRelationUnBindResponse,
    pub sign: String,
}