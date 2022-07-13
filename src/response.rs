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
    response: PayResponse,
    sign: Option<String>,
    alipay_cert_sn: Option<String>,
}
#[skip_serializing_none]
#[derive(Serialize, Deserialize)]
struct PayResponse {
    code: Option<String>,
    msg: Option<String>,
    sub_code: Option<String>,
    sub_msg: Option<String>,
    trade_no: Option<String>,
    out_trade_no: Option<String>,
    buyer_logon_id: Option<String>,
    settle_amount: Option<String>,
    pay_currency: Option<String>,
    pay_amount: Option<String>,
    settle_trans_rate: Option<String>,
    trans_pay_rate: Option<String>,
    total_amount: Option<String>,
    trans_currency: Option<String>,
    settle_currency: Option<String>,
    receipt_amount: Option<String>,
    buyer_pay_amount: Option<String>,
    point_amount: Option<String>,
    invoice_amount: Option<String>,
    gmt_payment: Option<String>,
    fund_bill_list: Vec<FundBill>,
    card_balance: Option<String>,
    store_name: Option<String>,
    buyer_user_id: Option<String>,
    discount_goods_detail: Option<String>,
    voucher_detail_list: Vec<VoucherDetail>,
    advance_amount: Option<String>,
    auth_trade_pay_mode: Option<String>,
    charge_amount: Option<String>,
    charge_flags: Option<String>,
    settlement_id: Option<String>,
    business_params: Option<String>,
    buyer_user_type: Option<String>,
    mdiscount_amount: Option<String>,
    discount_amount: Option<String>,
    buyer_user_name: Option<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize)]
struct FundBill {
    fund_channel: Option<String>,
    bank_code: Option<String>,
    amount: Option<String>,
    real_amount: Option<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize)]
struct VoucherDetail {
    id: Option<String>,
    name: Option<String>,
    r#type: Option<String>,
    amount: Option<String>,
    merchant_contribute: Option<String>,
    other_contribute: Option<String>,
    memo: Option<String>,
    template_id: Option<String>,
    purchase_buyer_contribute: Option<String>,
    purchase_merchant_contribute: Option<String>,
    purchase_ant_contribute: Option<String>,
}

//===================================================
#[skip_serializing_none]
#[derive(Serialize, Deserialize)]
pub struct TradeQueryResponse {
    #[serde(rename(deserialize = "alipay_trade_query_response"))]
    response: QueryResponse,
    sign: Option<String>,
    alipay_cert_sn: Option<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize)]
struct QueryResponse {
    code: Option<String>,
    msg: Option<String>,
    sub_code: Option<String>,
    sub_msg: Option<String>,
    trade_no: Option<String>,
    out_trade_no: Option<String>,
    buyer_logon_id: Option<String>,
    trade_status: Option<String>,
    total_amount: Option<String>,
    trans_currency: Option<String>,
    settle_currency: Option<String>,
    settle_amount: Option<String>,
    pay_currency: Option<String>,
    pay_amount: Option<String>,
    settle_trans_rate: Option<String>,
    trans_pay_rate: Option<String>,
    buyer_pay_amount: Option<String>,
    point_amount: Option<String>,
    invoice_amount: Option<String>,
    send_pay_date: Option<String>,
    receipt_amount: Option<String>,
    store_id: Option<String>,
    terminal_id: Option<String>,
    fund_bill_list: FundBill,
    store_name: Option<String>,
    buyer_user_id: Option<String>,
    charge_amount: Option<String>,
    charge_flags: Option<String>,
    settlement_id: Option<String>,
    trade_settle_info: Vec<TradeSettleInfo>,
    auth_trade_pay_mode: Option<String>,
    buyer_user_type: Option<String>,
    mdiscount_amount: Option<String>,
    discount_amount: Option<String>,
    buyer_user_name: Option<String>,
    subject: Option<String>,
    body: Option<String>,
    alipay_sub_merchant_id: Option<String>,
    ext_infos: Option<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize)]
struct TradeSettleInfo {
    trade_settle_detail_list: Vec<TradeSettleDetail>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize)]
struct TradeSettleDetail {
    operation_type: Option<String>,
    operation_serial_no: Option<String>,
    operation_dt: Option<String>,
    trans_out: Option<String>,
    trans_in: Option<String>,
    amount: Option<String>,
}

//===================================================
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct TradeCreateResponse {
    #[serde(rename(deserialize = "alipay_trade_create_response"))]
    response: CreateResponse,
    sign: Option<String>,
    alipay_cert_sn: Option<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Default, Debug, Clone)]

pub struct CreateResponse {
    code: Option<String>,
    msg: Option<String>,
    sub_code: Option<String>,
    sub_msg: Option<String>,
    trade_no: Option<String>,
    out_trade_no: Option<String>,
}

//===================================================
#[skip_serializing_none]
#[derive(Serialize, Deserialize)]
pub struct TradeCloseResponse {
    #[serde(rename(deserialize = "alipay_trade_close_response"))]
    response: CloseResponse,
    sign: Option<String>,
    alipay_cert_sn: Option<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize)]
struct CloseResponse {
    code: Option<String>,
    msg: Option<String>,
    sub_code: Option<String>,
    sub_msg: Option<String>,
    trade_no: Option<String>,
    out_trade_no: Option<String>,
}

//===================================================
#[skip_serializing_none]
#[derive(Serialize, Deserialize)]
pub struct TradeCancelResponse {
    #[serde(rename(deserialize = "alipay_trade_cancel_response"))]
    response: CancelResponse,
    alipay_cert_sn: Option<String>,
    sign: Option<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize)]
struct CancelResponse {
    code: Option<String>,
    msg: Option<String>,
    sub_code: Option<String>,
    sub_msg: Option<String>,
    trade_no: Option<String>,
    out_trade_no: Option<String>,
    retry_flag: Option<String>,
    action: Option<String>,
    gmt_refund_pay: Option<String>,
    refund_settlement_id: Option<String>,
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
    expires_in: i32,
    re_expires_in: i32,
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
    response: RefundResponse,
    alipay_cert_sn: Option<String>,
    sign: Option<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize)]
struct RefundResponse {
    code: Option<String>,
    msg: Option<String>,
    sub_code: Option<String>,
    sub_msg: Option<String>,
    trade_no: Option<String>,
    out_trade_no: Option<String>,
    buyer_logon_id: Option<String>,
    fund_change: Option<String>,
    refund_fee: Option<String>,
    refund_currency: Option<String>,
    gmt_refund_pay: Option<String>,
    refund_detail_item_list: Vec<TradeFundBill>,
    store_name: Option<String>,
    buyer_user_id: Option<String>,
    refund_preset_paytool_list: Vec<RefundPresetPaytool>,
    refund_settlement_id: Option<String>,
    present_refund_buyer_amount: Option<String>,
    present_refund_discount_amount: Option<String>,
    present_refund_mdiscount_amount: Option<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize)]
struct RefundPresetPaytool {
    amount: Vec<String>,
    assert_type_code: Option<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize)]
struct TradeFundBill {
    fund_channel: Option<String>,
    bank_code: Option<String>,
    amount: Option<String>,
    real_amount: Option<String>,
    fund_type: Option<String>,
}

//===================================================
#[skip_serializing_none]
#[derive(Serialize, Deserialize)]
pub struct TradeFastpayRefundQueryResponse {
    #[serde(rename(deserialize = "alipay_trade_fastpay_refund_query_response"))]
    response: RefundQueryResponse,
    alipay_cert_sn: Option<String>,
    sign: Option<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize)]
struct RefundQueryResponse {
    code: Option<String>,
    msg: Option<String>,
    sub_code: Option<String>,
    sub_msg: Option<String>,
    trade_no: Option<String>,
    out_trade_no: Option<String>,
    out_request_no: Option<String>,
    refund_reason: Option<String>,
    total_amount: Option<String>,
    refund_amount: Option<String>,
    refund_royaltys: Vec<RefundRoyalty>,
    gmt_refund_pay: Option<String>,
    refund_detail_item_list: Vec<TradeFundBill>,
    send_back_fee: Option<String>,
    refund_settlement_id: Option<String>,
    present_refund_buyer_amount: Option<String>,
    present_refund_discount_amount: Option<String>,
    present_refund_mdiscount_amount: Option<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize)]
struct RefundRoyalty {
    refund_amount: Option<String>,
    royalty_type: Option<String>,
    result_code: Option<String>,
    trans_out: Option<String>,
    trans_out_email: Option<String>,
    trans_in: Option<String>,
    trans_in_email: Option<String>,
}
//===================================================
#[skip_serializing_none]
#[derive(Serialize, Deserialize)]
pub struct TradeOrderSettleResponse {
    #[serde(rename(deserialize = "alipay_trade_order_settle_response"))]
    response: OrderSettleResponse,
    alipay_cert_sn: Option<String>,
    sign: Option<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize)]
struct OrderSettleResponse {
    code: Option<String>,
    msg: Option<String>,
    sub_code: Option<String>,
    sub_msg: Option<String>,
    trade_no: Option<String>,
}

//===================================================
#[skip_serializing_none]
#[derive(Serialize, Deserialize)]
pub struct TradePrecreateResponse {
    #[serde(rename(deserialize = "alipay_trade_precreate_response"))]
    response: PrecreateResponse,
    alipay_cert_sn: Option<String>,
    sign: Option<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize)]
struct PrecreateResponse {
    code: Option<String>,
    msg: Option<String>,
    sub_code: Option<String>,
    sub_msg: Option<String>,
    out_trade_no: Option<String>,
    qrcode: Option<String>,
}

//===================================================
#[skip_serializing_none]
#[derive(Serialize, Deserialize)]
pub struct TradePageRefundResponse {
    #[serde(rename(deserialize = "alipay_trade_page_refund_response"))]
    response: PageRefundResponse,
    alipay_cert_sn: Option<String>,
    sign: Option<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize)]
struct PageRefundResponse {
    code: Option<String>,
    msg: Option<String>,
    sub_code: Option<String>,
    sub_msg: Option<String>,
    trade_no: Option<String>,
    out_trade_no: Option<String>,
    out_request_no: Option<String>,
    refund_amount: Option<String>,
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
    tokens: Vec<Token>,
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
    passed: Vec<String>,
    identity_info: Option<String>,
    material_info: Option<String>,
}
