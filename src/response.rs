use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Response<T>(HashMap<String, T>);

#[derive(Serialize, Deserialize)]
struct TradePayResponse {
    response: PayResponse, //// `json:"alipay_trade_pay_response,omitempty"`
    sign: Option<String>,  // type//// `json:"sign"`
    alipay_cert_sn: Option<String>, //// `json:"alipay_cert_sn,omitempty"`
}
// type//
#[derive(Serialize, Deserialize)]
struct PayResponse {
    code: Option<String>,                  //// `json:"code,omitempty"`
    msg: Option<String>,                   //// `json:"msg,omitempty"`
    sub_code: Option<String>,              //// `json:"sub_code,omitempty"`
    sub_msg: Option<String>,               //// `json:"sub_msg,omitempty"`
    trade_no: Option<String>,              //// `json:"trade_no,omitempty"`
    out_trade_no: Option<String>,          //// `json:"out_trade_no,omitempty"`
    buyer_logon_id: Option<String>,        //// `json:"buyer_logon_id,omitempty"`
    settle_amount: Option<String>,         //// `json:"settle_amount,omitempty"`
    pay_currency: Option<String>,          //// `json:"pay_currency,omitempty"`
    pay_amount: Option<String>,            //// `json:"pay_amount,omitempty"`
    settle_trans_rate: Option<String>,     //// `json:"settle_trans_rate,omitempty"`
    trans_pay_rate: Option<String>,        //// `json:"trans_pay_rate,omitempty"`
    total_amount: Option<String>,          //// `json:"total_amount,omitempty"`
    trans_currency: Option<String>,        //// `json:"trans_currency,omitempty"`
    settle_currency: Option<String>,       //// `json:"settle_currency,omitempty"`
    receipt_amount: Option<String>,        //// `json:"receipt_amount,omitempty"`
    buyer_pay_amount: Option<String>,      //// `json:"buyer_pay_amount,omitempty"`
    point_amount: Option<String>,          //// `json:"point_amount,omitempty"`
    invoice_amount: Option<String>,        //// `json:"invoice_amount,omitempty"`
    gmt_payment: Option<String>,           //// `json:"gmt_payment,omitempty"`
    fund_bill_list: Vec<FundBill>,         //// `json:"fund_bill_list"`
    card_balance: Option<String>,          // // `json:"card_balance,omitempty"`
    store_name: Option<String>,            //// `json:"store_name,omitempty"`
    buyer_user_id: Option<String>,         //// `json:"buyer_user_id,omitempty"`
    discount_goods_detail: Option<String>, //// `json:"discount_goods_detail,omitempty"`
    voucher_detail_list: Vec<VoucherDetail>,
    advance_amount: Option<String>, //// `json:"advance_amount,omitempty"`
    auth_trade_pay_mode: Option<String>, //// `json:"auth_trade_pay_mode,omitempty"`
    charge_amount: Option<String>,  //// `json:"charge_amount,omitempty"`
    charge_flags: Option<String>,   //// `json:"charge_flags,omitempty"`
    settlement_id: Option<String>,  //// `json:"settlement_id,omitempty"`
    business_params: Option<String>, //// `json:"business_params,omitempty"`
    buyer_user_type: Option<String>, //// `json:"buyer_user_type,omitempty"`
    mdiscount_amount: Option<String>, //// `json:"mdiscount_amount,omitempty"`
    discount_amount: Option<String>, //// `json:"discount_amount,omitempty"`
    buyer_user_name: Option<String>, //// `json:"buyer_user_name,omitempty"`
}
#[derive(Serialize, Deserialize)]
struct FundBill {
    fund_channel: Option<String>, //// `json:"fund_channel,omitempty"`
    bank_code: Option<String>,    //// `json:"bank_code,omitempty"`
    amount: Option<String>,       //// `json:"amount,omitempty"`
    real_amount: Option<String>,  //// `json:"real_amount,omitempty"`
}
#[derive(Serialize, Deserialize)]
struct VoucherDetail {
    id: Option<String>,                           //// `json:"id,omitempty"`
    name: Option<String>,                         //// `json:"name,omitempty"`
    r#type: Option<String>,                       //// `json:"type,omitempty"`
    amount: Option<String>,                       //// `json:"amount,omitempty"`
    merchant_contribute: Option<String>,          //// `json:"merchant_contribute,omitempty"`
    other_contribute: Option<String>,             //// `json:"other_contribute,omitempty"`
    memo: Option<String>,                         //// `json:"memo,omitempty"`
    template_id: Option<String>,                  //// `json:"template_id,omitempty"`
    purchase_buyer_contribute: Option<String>,    //// `json:"purchase_buyer_contribute,omitempty"`
    purchase_merchant_contribute: Option<String>, //// `json:"purchase_merchant_contribute,omitempty"`
    purchase_ant_contribute: Option<String>,      //// `json:"purchase_ant_contribute,omitempty"`
}

//===================================================
#[derive(Serialize, Deserialize)]
struct TradeQueryResponse {
    response: QueryResponse, // // `json:"alipay_trade_query_response,omitempty"`
    sign: Option<String>,    // // `json:"sign"`
    alipay_cert_sn: Option<String>, //// `json:"alipay_cert_sn,omitempty"`
}

#[derive(Serialize, Deserialize)]
struct QueryResponse {
    code: Option<String>,                    //// `json:"code,omitempty"`
    msg: Option<String>,                     //// `json:"msg,omitempty"`
    sub_code: Option<String>,                //// `json:"sub_code,omitempty"`
    sub_msg: Option<String>,                 //// `json:"sub_msg,omitempty"`
    trade_no: Option<String>,                //// `json:"trade_no,omitempty"`
    out_trade_no: Option<String>,            //// `json:"out_trade_no,omitempty"`
    buyer_logon_id: Option<String>,          //// `json:"buyer_logon_id,omitempty"`
    trade_status: Option<String>,            //// `json:"trade_status,omitempty"`
    total_amount: Option<String>,            //// `json:"total_amount,omitempty"`
    trans_currency: Option<String>,          //// `json:"trans_currency,omitempty"`
    settle_currency: Option<String>,         //// `json:"settle_currency,omitempty"`
    settle_amount: Option<String>,           //// `json:"settle_amount,omitempty"`
    pay_currency: Option<String>,            //// `json:"pay_currency,omitempty"`
    pay_amount: Option<String>,              //// `json:"pay_amount,omitempty"`
    settle_trans_rate: Option<String>,       //// `json:"settle_trans_rate,omitempty"`
    trans_pay_rate: Option<String>,          //// `json:"trans_pay_rate,omitempty"`
    buyer_pay_amount: Option<String>,        //// `json:"buyer_pay_amount,omitempty"`
    point_amount: Option<String>,            //// `json:"point_amount,omitempty"`
    invoice_amount: Option<String>,          //// `json:"invoice_amount,omitempty"`
    send_pay_date: Option<String>,           //// `json:"send_pay_date,omitempty"`
    receipt_amount: Option<String>,          //// `json:"receipt_amount,omitempty"`
    store_id: Option<String>,                //// `json:"store_id,omitempty"`
    terminal_id: Option<String>,             //// `json:"terminal_id,omitempty"`
    fund_bill_list: FundBill,                // // `json:"fund_bill_list"`
    store_name: Option<String>,              //// `json:"store_name,omitempty"`
    buyer_user_id: Option<String>,           //// `json:"buyer_user_id,omitempty"`
    charge_amount: Option<String>,           //// `json:"charge_amount,omitempty"`
    charge_flags: Option<String>,            //// `json:"charge_flags,omitempty"`
    settlement_id: Option<String>,           //// `json:"settlement_id,omitempty"`
    trade_settle_info: Vec<TradeSettleInfo>, // `json:"trade_settle_info,omitempty"`
    auth_trade_pay_mode: Option<String>,     // `json:"auth_trade_pay_mode,omitempty"`
    buyer_user_type: Option<String>,         // `json:"buyer_user_type,omitempty"`
    mdiscount_amount: Option<String>,        // `json:"mdiscount_amount,omitempty"`
    discount_amount: Option<String>,         // `json:"discount_amount,omitempty"`
    buyer_user_name: Option<String>,         // `json:"buyer_user_name,omitempty"`
    subject: Option<String>,                 // `json:"subject,omitempty"`
    body: Option<String>,                    // `json:"body,omitempty"`
    alipay_sub_merchant_id: Option<String>,  // `json:"alipay_sub_merchant_id,omitempty"`
    ext_infos: Option<String>,               // `json:"ext_infos,omitempty"`
}

#[derive(Serialize, Deserialize)]
struct TradeSettleInfo {
    trade_settle_detail_list: Vec<TradeSettleDetail>, // `json:"trade_settle_detail_list,omitempty"`
}

#[derive(Serialize, Deserialize)]
struct TradeSettleDetail {
    operation_type: Option<String>, // `json:"operation_type,omitempty"`
    operation_serial_no: Option<String>, // `json:"operation_serial_no,omitempty"`
    operation_dt: Option<String>,   // `json:"operation_dt,omitempty"`
    trans_out: Option<String>,      // `json:"trans_out,omitempty"`
    trans_in: Option<String>,       // `json:"trans_in,omitempty"`
    amount: Option<String>,         // `json:"amount,omitempty"`
}

//===================================================
#[derive(Serialize, Deserialize)]
struct TradeCreateResponse {
    response: CreateResponse, // `json:"alipay_trade_create_response,omitempty"`
    sign: Option<String>,     // `json:"sign"`
    alipay_cert_sn: Option<String>, // `json:"alipay_cert_sn,omitempty"`
}

#[derive(Serialize, Deserialize)]
struct CreateResponse {
    code: Option<String>,         // `json:"code,omitempty"`
    msg: Option<String>,          // `json:"msg,omitempty"`
    sub_code: Option<String>,     // `json:"sub_code,omitempty"`
    sub_msg: Option<String>,      // `json:"sub_msg,omitempty"`
    trade_no: Option<String>,     // `json:"trade_no,omitempty"`
    out_trade_no: Option<String>, // `json:"out_trade_no,omitempty"`
}

//===================================================
#[derive(Serialize, Deserialize)]
struct TradeCloseResponse {
    response: CloseResponse, // `json:"alipay_trade_close_response,omitempty"`
    sign: Option<String>,    // `json:"sign"`
    alipay_cert_sn: Option<String>, // `json:"alipay_cert_sn,omitempty"`
}

#[derive(Serialize, Deserialize)]
struct CloseResponse {
    code: Option<String>,         // `json:"code,omitempty"`
    msg: Option<String>,          // `json:"msg,omitempty"`
    sub_code: Option<String>,     // `json:"sub_code,omitempty"`
    sub_msg: Option<String>,      // `json:"sub_msg,omitempty"`
    trade_no: Option<String>,     // `json:"trade_no,omitempty"`
    out_trade_no: Option<String>, // `json:"out_trade_no,omitempty"`
}

//===================================================
#[derive(Serialize, Deserialize)]
struct TradeCancelResponse {
    response: CancelResponse, // `json:"alipay_trade_cancel_response,omitempty"`
    alipay_cert_sn: Option<String>, // `json:"alipay_cert_sn,omitempty"`
    sign: Option<String>,     // `json:"sign"`
}

#[derive(Serialize, Deserialize)]
struct CancelResponse {
    code: Option<String>,                 // `json:"code,omitempty"`
    msg: Option<String>,                  // `json:"msg,omitempty"`
    sub_code: Option<String>,             // `json:"sub_code,omitempty"`
    sub_msg: Option<String>,              // `json:"sub_msg,omitempty"`
    trade_no: Option<String>,             // `json:"trade_no,omitempty"`
    out_trade_no: Option<String>,         // `json:"out_trade_no,omitempty"`
    retry_flag: Option<String>,           // `json:"retry_flag,omitempty"`
    action: Option<String>,               // `json:"action,omitempty"`
    gmt_refund_pay: Option<String>,       // `json:"gmt_refund_pay,omitempty"`
    refund_settlement_id: Option<String>, // `json:"refund_settlement_id,omitempty"`
}

//===================================================
#[derive(Serialize, Deserialize)]
struct SystemOauthTokenResponse {
    response: OauthTokenInfo, // `json:"alipay_system_oauth_token_response,omitempty"`
    // ErrorResponse *ErrorResponse  // `json:"error_response,omitempty"`
    alipay_cert_sn: Option<String>, // `json:"alipay_cert_sn,omitempty"`
    sign: Option<String>,           // `json:"sign"`
}

#[derive(Serialize, Deserialize)]
struct OauthTokenInfo {
    access_token: Option<String>,   // `json:"access_token,omitempty"`
    alipay_user_id: Option<String>, // `json:"alipay_user_id,omitempty"`
    expires_in: i32,                // `json:"expires_in,omitempty"`
    re_expires_in: i32,             // `json:"re_expires_in,omitempty"`
    refresh_token: Option<String>,  // `json:"refresh_token,omitempty"`
    user_id: Option<String>,        // `json:"user_id,omitempty"`
}

//===================================================

#[derive(Serialize, Deserialize)]
struct UserInfoShareResponse {
    response: UserInfoShare, // `json:"alipay_user_info_share_response,omitempty"`
    alipay_cert_sn: Option<String>, // `json:"alipay_cert_sn,omitempty"`
    sign: Option<String>,    // `json:"sign"`
}

#[derive(Serialize, Deserialize)]
struct UserInfoShare {
    code: Option<String>,                 // `json:"code,omitempty"`
    msg: Option<String>,                  // `json:"msg,omitempty"`
    sub_code: Option<String>,             // `json:"sub_code,omitempty"`
    sub_msg: Option<String>,              // `json:"sub_msg,omitempty"`
    user_id: Option<String>,              // `json:"user_id,omitempty"`
    avatar: Option<String>,               // `json:"avatar,omitempty"`
    province: Option<String>,             // `json:"province,omitempty"`
    city: Option<String>,                 // `json:"city,omitempty"`
    nick_name: Option<String>,            // `json:"nick_name,omitempty"`
    is_student_certified: Option<String>, // `json:"is_student_certified,omitempty"`
    user_type: Option<String>,            // `json:"user_type,omitempty"`
    user_status: Option<String>,          // `json:"user_status,omitempty"`
    is_certified: Option<String>,         // `json:"is_certified,omitempty"`
    gender: Option<String>,               // `json:"gender,omitempty"`
}

//===================================================
#[derive(Serialize, Deserialize)]
struct TradeRefundResponse {
    response: RefundResponse, // `json:"alipay_trade_refund_response,omitempty"`
    alipay_cert_sn: Option<String>, // `json:"alipay_cert_sn,omitempty"`
    sign: Option<String>,     // `json:"sign"`
}

#[derive(Serialize, Deserialize)]
struct RefundResponse {
    code: Option<String>,                                 // `json:"code,omitempty"`
    msg: Option<String>,                                  // `json:"msg,omitempty"`
    sub_code: Option<String>,                             // `json:"sub_code,omitempty"`
    sub_msg: Option<String>,                              // `json:"sub_msg,omitempty"`
    trade_no: Option<String>,                             // `json:"trade_no,omitempty"`
    out_trade_no: Option<String>,                         // `json:"out_trade_no,omitempty"`
    buyer_logon_id: Option<String>,                       // `json:"buyer_logon_id,omitempty"`
    fund_change: Option<String>,                          // `json:"fund_change,omitempty"`
    refund_fee: Option<String>,                           // `json:"refund_fee,omitempty"`
    refund_currency: Option<String>,                      // `json:"refund_currency,omitempty"`
    gmt_refund_pay: Option<String>,                       // `json:"gmt_refund_pay,omitempty"`
    refund_detail_item_list: Vec<TradeFundBill>, // `json:"refund_detail_item_list,omitempty"`
    store_name: Option<String>,                  // `json:"store_name,omitempty"`
    buyer_user_id: Option<String>,               // `json:"buyer_user_id,omitempty"`
    refund_preset_paytool_list: Vec<RefundPresetPaytool>, // `json:"refund_preset_paytool_list,omitempty"`
    refund_settlement_id: Option<String>,                 // `json:"refund_settlement_id,omitempty"`
    present_refund_buyer_amount: Option<String>, // `json:"present_refund_buyer_amount,omitempty"`
    present_refund_discount_amount: Option<String>, // `json:"present_refund_discount_amount,omitempty"`
    present_refund_mdiscount_amount: Option<String>, // `json:"present_refund_mdiscount_amount,omitempty"`
}

#[derive(Serialize, Deserialize)]
struct RefundPresetPaytool {
    amount: Vec<String>,              // `json:"amount,omitempty"`
    assert_type_code: Option<String>, // `json:"assert_type_code,omitempty"`
}
#[derive(Serialize, Deserialize)]
struct TradeFundBill {
    fund_channel: Option<String>, // `json:"fund_channel,omitempty"` //同步通知里是 fund_channel
    bank_code: Option<String>,    // `json:"bank_code,omitempty"`
    amount: Option<String>,       // `json:"amount,omitempty"`
    real_amount: Option<String>,  // `json:"real_amount,omitempty"`
    fund_type: Option<String>,    // `json:"fund_type,omitempty"`
}

//===================================================
#[derive(Serialize, Deserialize)]
struct TradeFastpayRefundQueryResponse {
    response: RefundQueryResponse, // `json:"alipay_trade_fastpay_refund_query_response,omitempty"`
    alipay_cert_sn: Option<String>, // `json:"alipay_cert_sn,omitempty"`
    sign: Option<String>,          // `json:"sign"`
}

#[derive(Serialize, Deserialize)]
struct RefundQueryResponse {
    code: Option<String>,                            // `json:"code,omitempty"`
    msg: Option<String>,                             // `json:"msg,omitempty"`
    sub_code: Option<String>,                        // `json:"sub_code,omitempty"`
    sub_msg: Option<String>,                         // `json:"sub_msg,omitempty"`
    trade_no: Option<String>,                        // `json:"trade_no,omitempty"`
    out_trade_no: Option<String>,                    // `json:"out_trade_no,omitempty"`
    out_request_no: Option<String>,                  // `json:"out_request_no,omitempty"`
    refund_reason: Option<String>,                   // `json:"refund_reason,omitempty"`
    total_amount: Option<String>,                    // `json:"total_amount,omitempty"`
    refund_amount: Option<String>,                   // `json:"refund_amount,omitempty"`
    refund_royaltys: Vec<RefundRoyalty>,             // `json:"refund_royaltys,omitempty"`
    gmt_refund_pay: Option<String>,                  // `json:"gmt_refund_pay,omitempty"`
    refund_detail_item_list: Vec<TradeFundBill>,     // `json:"refund_detail_item_list,omitempty"`
    send_back_fee: Option<String>,                   // `json:"send_back_fee,omitempty"`
    refund_settlement_id: Option<String>,            // `json:"refund_settlement_id,omitempty"`
    present_refund_buyer_amount: Option<String>, // `json:"present_refund_buyer_amount,omitempty"`
    present_refund_discount_amount: Option<String>, // `json:"present_refund_discount_amount,omitempty"`
    present_refund_mdiscount_amount: Option<String>, // `json:"present_refund_mdiscount_amount,omitempty"`
}

#[derive(Serialize, Deserialize)]
struct RefundRoyalty {
    refund_amount: Option<String>,   // `json:"refund_amount,omitempty"`
    royalty_type: Option<String>,    // `json:"royalty_type,omitempty"`
    result_code: Option<String>,     // `json:"result_code,omitempty"`
    trans_out: Option<String>,       // `json:"trans_out,omitempty"`
    trans_out_email: Option<String>, // `json:"trans_out_email,omitempty"`
    trans_in: Option<String>,        // `json:"trans_in,omitempty"`
    trans_in_email: Option<String>,  // `json:"trans_in_email,omitempty"`
}
//===================================================
#[derive(Serialize, Deserialize)]
struct TradeOrderSettleResponse {
    response: OrderSettleResponse, // `json:"alipay_trade_order_settle_response,omitempty"`
    alipay_cert_sn: Option<String>, // `json:"alipay_cert_sn,omitempty"`
    sign: Option<String>,          // `json:"sign"`
}

#[derive(Serialize, Deserialize)]
struct OrderSettleResponse {
    code: Option<String>,     // `json:"code,omitempty"`
    msg: Option<String>,      // `json:"msg,omitempty"`
    sub_code: Option<String>, // `json:"sub_code,omitempty"`
    sub_msg: Option<String>,  // `json:"sub_msg,omitempty"`
    trade_no: Option<String>, // `json:"trade_no,omitempty"`
}

//===================================================
#[derive(Serialize, Deserialize)]
struct TradePrecreateResponse {
    response: PrecreateResponse, // `json:"alipay_trade_precreate_response,omitempty"`
    // NullResponse *ErrorResponse     // `json:"null_response,omitempty"`
    alipay_cert_sn: Option<String>, // `json:"alipay_cert_sn,omitempty"`
    sign: Option<String>,           // `json:"sign"`
}

#[derive(Serialize, Deserialize)]
struct PrecreateResponse {
    code: Option<String>,         // `json:"code,omitempty"`
    msg: Option<String>,          // `json:"msg,omitempty"`
    sub_code: Option<String>,     // `json:"sub_code,omitempty"`
    sub_msg: Option<String>,      // `json:"sub_msg,omitempty"`
    out_trade_no: Option<String>, // `json:"out_trade_no,omitempty"`
    qrcode: Option<String>,       // `json:"qr_code,omitempty"`
}

//===================================================
#[derive(Serialize, Deserialize)]
struct TradePageRefundResponse {
    response: PageRefundResponse, // `json:"alipay_trade_page_refund_response,omitempty"`
    alipay_cert_sn: Option<String>, // `json:"alipay_cert_sn,omitempty"`
    sign: Option<String>,         // `json:"sign"`
}

#[derive(Serialize, Deserialize)]
struct PageRefundResponse {
    code: Option<String>,           // `json:"code,omitempty"`
    msg: Option<String>,            // `json:"msg,omitempty"`
    sub_code: Option<String>,       // `json:"sub_code,omitempty"`
    sub_msg: Option<String>,        // `json:"sub_msg,omitempty"`
    trade_no: Option<String>,       // `json:"trade_no,omitempty"`
    out_trade_no: Option<String>,   // `json:"out_trade_no,omitempty"`
    out_request_no: Option<String>, // `json:"out_request_no,omitempty"`
    refund_amount: Option<String>,  // `json:"refund_amount,omitempty"`
}

//===================================================
#[derive(Serialize, Deserialize)]
struct FundTransToaccountTransferResponse {
    response: TransToaccountTransferResponse, // `json:"alipay_fund_trans_toaccount_transfer_response,omitempty"`
    alipay_cert_sn: Option<String>,           // `json:"alipay_cert_sn,omitempty"`
    sign: Option<String>,                     // `json:"sign"`
}

#[derive(Serialize, Deserialize)]
struct TransToaccountTransferResponse {
    code: Option<String>,       // `json:"code,omitempty"`
    msg: Option<String>,        // `json:"msg,omitempty"`
    sub_code: Option<String>,   // `json:"sub_code,omitempty"`
    sub_msg: Option<String>,    // `json:"sub_msg,omitempty"`
    out_biz_no: Option<String>, // `json:"out_biz_no,omitempty"`
    order_id: Option<String>,   // `json:"order_id,omitempty"`
    pay_date: Option<String>,   // `json:"pay_date,omitempty"`
}

//===================================================
#[derive(Serialize, Deserialize)]
struct ZhimaCreditScoreGetResponse {
    response: ScoreGetResponse, // `json:"zhima_credit_score_get_response,omitempty"`
    alipay_cert_sn: Option<String>, // `json:"alipay_cert_sn,omitempty"`
    sign: Option<String>,       // `json:"sign"`
}

#[derive(Serialize, Deserialize)]
struct ScoreGetResponse {
    code: Option<String>,     // `json:"code,omitempty"`
    msg: Option<String>,      // `json:"msg,omitempty"`
    sub_code: Option<String>, // `json:"sub_code,omitempty"`
    sub_msg: Option<String>,  // `json:"sub_msg,omitempty"`
    biz_no: Option<String>,   // `json:"biz_no,omitempty"`
    zm_score: Option<String>, // `json:"zm_score,omitempty"`
}

//===================================================
#[derive(Serialize, Deserialize)]
struct OpenAuthTokenAppResponse {
    response: AuthTokenAppResponse, // `json:"alipay_open_auth_token_app_response,omitempty"`
    alipay_cert_sn: Option<String>, // `json:"alipay_cert_sn,omitempty"`
    sign: Option<String>,           // `json:"sign"`
}

#[derive(Serialize, Deserialize)]
struct AuthTokenAppResponse {
    code: Option<String>,              // `json:"code,omitempty"`
    msg: Option<String>,               // `json:"msg,omitempty"`
    sub_code: Option<String>,          // `json:"sub_code,omitempty"`
    sub_msg: Option<String>,           // `json:"sub_msg,omitempty"`
    user_id: Option<String>,           // `json:"user_id,omitempty"`
    auth_app_id: Option<String>,       // `json:"auth_app_id,omitempty"`
    app_auth_token: Option<String>,    // `json:"app_auth_token,omitempty"`
    app_refresh_token: Option<String>, // `json:"app_refresh_token,omitempty"`
    expires_in: Option<i32>,           // `json:"expires_in,omitempty"`
    re_expires_in: Option<i32>,        // `json:"re_expires_in,omitempty"`
    tokens: Vec<Token>,                // `json:"tokens,omitempty"`
}

#[derive(Serialize, Deserialize)]
struct Token {
    app_auth_token: Option<String>,    // `json:"app_auth_token,omitempty"`
    app_refresh_token: Option<String>, // `json:"app_refresh_token,omitempty"`
    auth_app_id: Option<String>,       // `json:"auth_app_id,omitempty"`
    expires_in: Option<i32>,           // `json:"expires_in,omitempty"`
    re_expires_in: Option<i32>,        // `json:"re_expires_in,omitempty"`
    user_id: Option<String>,           // `json:"user_id,omitempty"`
}
//===================================================
#[derive(Serialize, Deserialize)]
struct UserCertifyOpenInitResponse {
    response: AlipayUserCertifyOpenInitResponse, // `json:"alipay_user_certify_open_initialize_response,omitempty"`
    alipay_cert_sn: Option<String>,              // `json:"alipay_cert_sn,omitempty"`
    sign: Option<String>,                        // `json:"sign"`
}

#[derive(Serialize, Deserialize)]
struct AlipayUserCertifyOpenInitResponse {
    code: Option<String>,       // `json:"code,omitempty"`
    msg: Option<String>,        // `json:"msg,omitempty"`
    sub_code: Option<String>,   // `json:"sub_code,omitempty"`
    sub_msg: Option<String>,    // `json:"sub_msg,omitempty"`
    certify_id: Option<String>, // `json:"certify_id,omitempty"`
}

//===================================================
#[derive(Serialize, Deserialize)]
struct UserCertifyOpenCertifyResponse {
    response: AlipayUserCertifyOpenCertifyResponse, // `json:"alipay_user_certify_open_certify_response,omitempty"`
    alipay_cert_sn: Option<String>,                 // `json:"alipay_cert_sn,omitempty"`
    sign: Option<String>,                           // `json:"sign"`
}

#[derive(Serialize, Deserialize)]
struct AlipayUserCertifyOpenCertifyResponse {
    code: Option<String>,     // `json:"code,omitempty"`
    msg: Option<String>,      // `json:"msg,omitempty"`
    sub_code: Option<String>, // `json:"sub_code,omitempty"`
    sub_msg: Option<String>,  // `json:"sub_msg,omitempty"`
}

//===================================================
#[derive(Serialize, Deserialize)]
struct UserCertifyOpenQueryResponse {
    response: AlipayUserCertifyOpenQueryResponse, // `json:"alipay_user_certify_open_query_response,omitempty"`
    alipay_cert_sn: Option<String>,               // `json:"alipay_cert_sn,omitempty"`
    sign: Option<String>,                         // `json:"sign"`
}

#[derive(Serialize, Deserialize)]
struct AlipayUserCertifyOpenQueryResponse {
    code: Option<String>,          // `json:"code,omitempty"`
    msg: Option<String>,           // `json:"msg,omitempty"`
    sub_code: Option<String>,      // `json:"sub_code,omitempty"`
    sub_msg: Option<String>,       // `json:"sub_msg,omitempty"`
    passed: Vec<String>,           // `json:"passed,omitempty"`
    identity_info: Option<String>, // `json:"identity_info,omitempty"`
    material_info: Option<String>, // `json:"material_info,omitempty"`
}
