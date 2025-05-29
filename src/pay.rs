//! 支付模块，包括各种支付接口函数。
#![allow(unused)]
// use anyhow::Result;
use std::collections::HashMap;
use std::hash::BuildHasher;

// use std::io::{Error, ErrorKind};

use std::borrow::{Borrow, BorrowMut};
use std::sync::Arc;

use crate::error::AliPaySDKError::AliPayError;
use crate::error::{AliPayResult, AliPaySDKError};
use crate::sign::{builder, Signer};
use gostd::builtin::byte;
use gostd::net::http;
use gostd::net::url;
use serde::{Deserialize, Serialize};

use crate::biz::{self, BizContenter, BizObject, TradeAppPayBiz, TradeCancelBiz, TradeCloseBiz, TradeCreateBiz, TradeFastpayRefundQueryBiz, TradeOrderSettleBiz, TradeOrderSettleQueryBiz, TradePagePayBiz, TradePageRefundBiz, TradePayBiz, TradePrecreateBiz, TradeQueryBiz, TradeRefundBiz, TradeRoyaltyRelationBindBiz, TradeRoyaltyRelationUnBindBiz, TradeWapPayBiz};
use crate::request::{Request, Requester};
use crate::response::{self, TradeCancelResponse, TradeCloseResponse, TradeCreateResponse, TradeFastpayRefundQueryResponse, TradeOrderSettleQueryResponse, TradeOrderSettleResponse, TradePageRefundResponse, TradePayResponse, TradePrecreateResponse, TradeQueryResponse, TradeRefundResponse, TradeRoyaltyRelationBindResponse, TradeRoyaltyRelationUnBindResponse};
use crate::util::{self, build_form, json_get};
pub trait Payer {
    fn trade_create(&self, biz_content: &TradeCreateBiz) -> AliPayResult<TradeCreateResponse>;

    fn trade_pay(&self, biz_content: &TradePayBiz) -> AliPayResult<TradePayResponse>;

    fn trade_precreate(
        &self,
        biz_content: &TradePrecreateBiz,
    ) -> AliPayResult<TradePrecreateResponse>;

    fn trade_app_pay(&self, biz_content: &TradeAppPayBiz) -> AliPayResult<String>;

    fn trade_wap_pay(&self, biz_content: &TradeWapPayBiz) -> AliPayResult<String>;

    fn trade_page_pay(&self, biz_content: &TradePagePayBiz) -> AliPayResult<String>;

    fn trade_query(&self, biz_content: &TradeQueryBiz) -> AliPayResult<TradeQueryResponse>;

    fn trade_cancel(&self, biz_content: &TradeCancelBiz) -> AliPayResult<TradeCancelResponse>;

    fn trade_refund(&self, biz_content: &TradeRefundBiz) -> AliPayResult<TradeRefundResponse>;

    fn trade_page_refund(
        &self,
        biz_content: &TradePageRefundBiz,
    ) -> AliPayResult<TradePageRefundResponse>;
    fn trade_fastpay_refund_query(
        &self,
        biz_content: &TradeFastpayRefundQueryBiz,
    ) -> AliPayResult<TradeFastpayRefundQueryResponse>;
    
    fn trade_order_settle(&self, biz_content: &TradeOrderSettleBiz) -> AliPayResult<TradeOrderSettleResponse>;

    fn trade_order_settle_query(&self, biz_content: &TradeOrderSettleQueryBiz) -> AliPayResult<TradeOrderSettleQueryResponse>;
    
    fn trade_royalty_relation_bind(&self, biz_content: &TradeRoyaltyRelationBindBiz) -> AliPayResult<TradeRoyaltyRelationBindResponse>;
    
    fn trade_royalty_relation_unbind(&self, biz_content: &TradeRoyaltyRelationUnBindBiz) -> AliPayResult<TradeRoyaltyRelationUnBindResponse>;
    
    fn trade_close(&self, biz_content: &TradeCloseBiz) -> AliPayResult<TradeCloseResponse>;
    fn async_verify_sign(&self, raw_body: &[u8]) -> AliPayResult<bool>;
}

/// 实现Payer接口的各种方法
///
/// # Example
///
/// ## 初始化client
///
/// 避免每次输入大量参数，建议像如下例子，单独写一个函数需要使用的时候调用函数就可以。
///
/// ```
/// use alipay_sdk_rust::biz::{self, BizContenter};
/// use alipay_sdk_rust::error::AliPayResult;
/// use alipay_sdk_rust::pay::{PayClient, Payer};
/// use alipay_sdk_rust::response::TradeCreateResponse;
/// fn new_pay_client() -> AliPayResult<impl Payer> {
///     let client = PayClient::builder()
/// .api_url("https://openapi.alipaydev.com/gateway.do")
/// .app_id("2021000117650139")
/// .alipay_root_cert_sn("687b59193f3f462dd5336e5abf83c5d8_02941eef3187dddf3d3b83462e1dfcf6")
/// .alipay_public_key("MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAoEz4Tegg1Ytfn2N+jWzDTOBbUNhcpIXz7dV3K8yIsDzinPwYTzzZhlCLYlKbfJGpSDVU0K3I6TnyGfNPEg+R7EC/3ycY1nETh8mxQ4dgrz8irKlLxSRDHVtPg4luncDz2u0hMab9QJdqF8DXD5H+r0Pdt6CSJgKJqLa0awPV3/8cTQZbhZ4ontnRdcwWWcp4/TunkEc891Aa5FmzWp4hgBYcu3BGcazS1HQA4r6wTwRkiqKsCwZeVTag4CiOeqe/vRFTxTMKF4gjRzdhTapUfcBCmXblEA3i8/7OILEyNHceRAxxxIpUjTyoRJ4/2xd0kWbw1gmkLFM0Fzee0eVgoQIDAQAB")
/// .app_cert_sn("8c68b9753e5b9e0bb7704a981936ecce")
/// .charset_utf8()
/// .format_json()
/// .private_key("MIIEogIBAAKCAQEAog0N+rHllTO+e42Bc5mpvowolWVStyurL3Ou/86uRMN8im7WG1v44h09IaZpw4k6dpYEj89d7aLd7IwnBR5Wg84Ox2LMR/Y/Pzo10hjlvJJOk+igqepSTtB/4UX0cG/9tWceHAWOFuD8uw/SSJegC91a0MmLUBUpd4wWnN1iOSi0442iNvNk79Z6xLKIs4LNJGCNddxcofvpdqq4/5ywxHo24m5zPQf6/ttGk5jQQVrF+y2ckdHKd2h7ZSOYI7nzlzbZqK0UOMDuTvRs696fPa5wSEshE0RQBcn5iCltNTPyLsL3RGUUlsLOPsyT6cFZtUAKJ+J6wEqQxM5TrvNHywIDAQABAoIBAGDTuhGccFipVVzP3ZSsMV+4sZsqsrTd8+hjkCIrZbeSsvyoY2hvmRPKcreDjtiWS4eF9e3T8wTF9yKbT8lgKkORQQVkBDnPalUmO/hwhf0Z0rfQHQfKCiorrO129iqk0AyvM698pj0HbBt9xaE4cBoGxnfQpVxReLiEzRInucP6lhE79v1BwXCwMtRVvFPPIFaLJ02JGIywN+jnkpLwJj8TAu5u3JawlRnsFJgQeTdsHs4G6E11WBeo7OZtKPiKWMcj1nPU0Dnr+6VG89Rx/cxqlMrlTJBhKsLEzcVQwcc3M3UnMOU3Of7Mj1olnUGJ90apVukDFM5OI4Mfqi9etekCgYEA9ViJqVUdzJwqTK/gmbAsRvri9+rmWhfoqBMGeUoHktOGR/hMKJ/LrVa1oBIcVVNdLbI7Ks0kySGCa6qm/4YP2DCihj0GKwnHdPQ94pd3lWvUFZimNKi5V/+sREU0dKSqK3b7F0njtpR6zn+x8KpktO7izL3o8740KpSb7xGb0BUCgYEAqRaLShbDYjhSfvIzWAbuPNpvvtDWUNip1cuJwzTvDthECV5ltkhLGVWVnStch6OeTbK+llLDVw+j/YT1KetQcZ60tw2spn8nq6UvC2IFa2h61zpa8VWeRDfhyEIzoBE8DFAyeWjqYHyHJlh0BzRA2P3ts2LwwwhRa6OHhYzQ0F8CgYAUUvpMab2nNoSWh7dOY/a3Bo+IxA/DBNoEGldd8tD/y8AC9EGy19HykQ1Irldkhhxg7bPTDt1uP/Vi3+cnob5sRVMhVarOI+g++wCpZazFVwJhq5yRHi0EaiymFymKRB3IrfmM61UOyewGcTOXYTYoeuWU2mKS1n3RzS/BtS64JQKBgB2husVAGftzfVmL3l2V0VhOu3iIJpbCcXjrE3hnJWHHmpy9sztvjeGhsvd5Kt0GWm6pXWcAmAUA069RBpnTCCTxOCBAQDppXC1jZEwtYF/DTou7SUazx2mTFXk/yMZLXueVglLuhOxlxlV8+NBuYtLkJSzjsOes5H/lh5Fq7QknAoGAJ/LzTBLPy3terUgqejSxB4pr6PMbNd8wEStHN1RmR2v9Msuto7PUT7OOjQYIJwQLnxQUDr65bB5uR35v+L/rC6XUkzJM18YWvmOhFM8OsIYc4HdDhSmeFpMXdbd6entMJEX0bWrTbS/UdEcqE30kwuNuEFQ07LopGY1gBEe1G8U=")
/// .public_key("MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAog0N+rHllTO+e42Bc5mpvowolWVStyurL3Ou/86uRMN8im7WG1v44h09IaZpw4k6dpYEj89d7aLd7IwnBR5Wg84Ox2LMR/Y/Pzo10hjlvJJOk+igqepSTtB/4UX0cG/9tWceHAWOFuD8uw/SSJegC91a0MmLUBUpd4wWnN1iOSi0442iNvNk79Z6xLKIs4LNJGCNddxcofvpdqq4/5ywxHo24m5zPQf6/ttGk5jQQVrF+y2ckdHKd2h7ZSOYI7nzlzbZqK0UOMDuTvRs696fPa5wSEshE0RQBcn5iCltNTPyLsL3RGUUlsLOPsyT6cFZtUAKJ+J6wEqQxM5TrvNHywIDAQAB")
/// .sign_type_rsa2()
/// .version_1_0()
/// .build()?;
///     Ok(client)
/// }
/// ```
#[derive(Debug, Default)]
pub struct PayClient {
    api_url: String,             // `json:"-"`                    // 接口网关地址
    private_key: String,         // `json:"-"`                    // rsa私钥单行文本字符串
    public_key: String,          // `json:"-"`                    // rsa公钥单行文本字符串
    alipay_public_key: String, // `json:"_"`                    // 证书解析出的支付宝公钥 从alipayCertPublicKey_RSA2.crt 文件中提取
    app_cert_sn: String, // `json:"app_cert_sn"`          // 应用公钥证书 SN 从appCertPublicKey_2021000117650139.crt 文件提取
    alipay_root_cert_sn: String, // `json:"alipay_root_cert_sn"`  // 支付宝根证书 SN 从alipayRootCert.crt 文件中提取
    app_id: String, // `json:"app_id"`               // 是	32	支付宝分配给开发者的应用ID	2014072300007148
    format: String, // `json:"format,omitempty"`     // 否	40	仅支持JSON	JSON
    charset: String, // `json:"charset"`              // 是	10	请求使用的编码格式，如utf-8,gbk,gb2312等	utf-8
    sign_type: String, // `json:"sign_type"`            // 是	10	商户生成签名字符串所使用的签名算法类型，目前支持RSA2和RSA，推荐使用RSA2	RSA2
    version: String,   // `json:"version"`              // 是	3	调用的接口版本，固定为：1.0	1.0
    return_url: String, // `json:"return_url,omitempty"` // 否 前台回跳地址 return_url 自动跳转回商户页面
    notify_url: String, // `json:"notify_url,omitempty"` 支付宝服务器主动通知callback商户服务器里指定的页面http/https路径
}

impl Payer for PayClient {
    /// <https://opendocs.alipay.com/apis/api_1/alipay.trade.create>
    ///
    /// alipay.trade.create(统一收单交易创建接口)
    fn trade_create(&self, biz_content: &TradeCreateBiz) -> AliPayResult<TradeCreateResponse> {
        let body = self.do_alipay(biz_content)?;
        let res: TradeCreateResponse = serde_json::from_slice(&body)?;
        if res.response.code != Some("10000".to_string()) {
            log::debug!("{}", serde_json::to_string(&res)?);
            return Err(AliPayError(format!(
                "trade_create failed: {} code:{}",
                res.response.sub_msg.unwrap().as_str(),
                res.response.sub_code.unwrap().as_str()
            )));
        }
        Ok(res)
    }

    /// <https://opendocs.alipay.com/apis/api_1/alipay.trade.pay>
    ///
    /// alipay.trade.pay(统一收单交易支付接口)
    fn trade_pay(&self, biz_content: &TradePayBiz) -> AliPayResult<TradePayResponse> {
        let body = self.do_alipay(biz_content)?;
        let res: TradePayResponse = serde_json::from_slice(&body)?;
        if res.response.code != Some("10000".to_string()) {
            log::debug!("{}", serde_json::to_string(&res)?);
            return Err(AliPayError(format!(
                "trade_pay failed: {} code:{}",
                res.response.sub_msg.unwrap().as_str(),
                res.response.sub_code.unwrap().as_str()
            )));
        }
        Ok(res)
    }

    /// <https://opendocs.alipay.com/apis/api_1/alipay.trade.precreate>
    ///
    /// alipay.trade.precreate(统一收单线下交易预创建)当面付-商家生成条码
    fn trade_precreate(
        &self,
        biz_content: &TradePrecreateBiz,
    ) -> AliPayResult<TradePrecreateResponse> {
        let body = self.do_alipay(biz_content)?;
        let res: TradePrecreateResponse = serde_json::from_slice(&body)?;
        if res.response.code != Some("10000".to_string()) {
            log::debug!("{}", serde_json::to_string(&res)?);
            return Err(AliPayError(format!(
                "trade_precreate failed: {} code:{}",
                res.response.sub_msg.unwrap().as_str(),
                res.response.sub_code.unwrap().as_str()
            )));
        }
        Ok(res)
    }

    /// <https://opendocs.alipay.com/apis/api_1/alipay.trade.app.pay>
    ///
    /// alipay.trade.app.pay(app支付接口2.0)后端只生成form数据给前端调用
    fn trade_app_pay(&self, biz_content: &TradeAppPayBiz) -> AliPayResult<String> {
        let body = self.do_alipay(biz_content)?;
        let res = String::from_utf8(body)?;
        Ok(res)
    }

    /// <https://opendocs.alipay.com/apis/api_1/alipay.trade.wap.pay>
    ///
    /// alipay.trade.wap.pay(手机网站支付接口2.0)后端只生成form数据给前端调用
    fn trade_wap_pay(&self, biz_content: &TradeWapPayBiz) -> AliPayResult<String> {
        let body = self.do_alipay(biz_content)?;
        let res = String::from_utf8(body)?;
        Ok(res)
    }

    /// <https://opendocs.alipay.com/apis/api_1/alipay.trade.page.pay>
    ///
    /// alipay.trade.page.pay(统一收单下单并支付页面接口)后端只生成form数据给前端调用
    fn trade_page_pay(&self, biz_content: &TradePagePayBiz) -> AliPayResult<String> {
        let body = self.do_alipay(biz_content)?;
        let res = String::from_utf8(body)?;
        Ok(res)
    }

    /// <https://opendocs.alipay.com/apis/api_1/alipay.trade.query>
    ///
    /// alipay.trade.query(统一收单线下交易查询)
    fn trade_query(&self, biz_content: &TradeQueryBiz) -> AliPayResult<TradeQueryResponse> {
        let body = self.do_alipay(biz_content)?;
        let res: TradeQueryResponse = serde_json::from_slice(&body)?;
        if res.response.code != Some("10000".to_string()) {
            log::debug!("{}", serde_json::to_string(&res)?);
            return Err(AliPayError(format!(
                "trade_query failed: {} code:{}",
                res.response.sub_msg.unwrap().as_str(),
                res.response.sub_code.unwrap().as_str()
            )));
        }
        Ok(res)
    }

    ///  <https://opendocs.alipay.com/apis/api_1/alipay.trade.cancel>
    ///
    /// alipay.trade.cancel(统一收单交易撤销接口)
    fn trade_cancel(&self, biz_content: &TradeCancelBiz) -> AliPayResult<TradeCancelResponse> {
        let body = self.do_alipay(biz_content)?;
        let res: TradeCancelResponse = serde_json::from_slice(&body)?;
        if res.response.code != Some("10000".to_string()) {
            log::debug!("{}", serde_json::to_string(&res)?);
            return Err(AliPayError(format!(
                "trade_cancel failed: {} code:{}",
                res.response.sub_msg.unwrap().as_str(),
                res.response.sub_code.unwrap().as_str()
            )));
        }
        Ok(res)
    }

    /// <https://opendocs.alipay.com/apis/api_1/alipay.trade.refund>
    ///
    /// alipay.trade.refund(统一收单交易退款接口)
    fn trade_refund(&self, biz_content: &TradeRefundBiz) -> AliPayResult<TradeRefundResponse> {
        let body = self.do_alipay(biz_content)?;
        let res: TradeRefundResponse = serde_json::from_slice(&body)?;
        if res.response.code != Some("10000".to_string()) {
            log::debug!("{}", serde_json::to_string(&res)?);
            return Err(AliPayError(format!(
                "trade_refund failed: {} code:{}",
                res.response.sub_msg.unwrap().as_str(),
                res.response.sub_code.unwrap().as_str()
            )));
        }
        Ok(res)
    }

    /// <https://opendocs.alipay.com/apis/api_1/alipay.trade.page.refund>
    ///
    /// alipay.trade.page.refund(统一收单退款页面接口)
    fn trade_page_refund(
        &self,
        biz_content: &TradePageRefundBiz,
    ) -> AliPayResult<TradePageRefundResponse> {
        let body = self.do_alipay(biz_content)?;
        let res: TradePageRefundResponse = serde_json::from_slice(&body)?;
        if res.response.code != Some("10000".to_string()) {
            log::debug!("{}", serde_json::to_string(&res)?);
            return Err(AliPayError(format!(
                "trade_page_refund failed: {} code:{}",
                res.response.sub_msg.unwrap().as_str(),
                res.response.sub_code.unwrap().as_str()
            )));
        }
        Ok(res)
    }
    /// <https://opendocs.alipay.com/apis/api_1/alipay.trade.fastpay.refund.query>
    ///
    /// alipay.trade.fastpay.refund.query(统一收单交易退款查询)
    fn trade_fastpay_refund_query(
        &self,
        biz_content: &TradeFastpayRefundQueryBiz,
    ) -> AliPayResult<TradeFastpayRefundQueryResponse> {
        let body = self.do_alipay(biz_content)?;
        let res: TradeFastpayRefundQueryResponse = serde_json::from_slice(&body)?;
        if res.response.code != Some("10000".to_string()) {
            log::debug!("{}", serde_json::to_string(&res)?);
            return Err(AliPayError(format!(
                "trade_fastpay_refund_query failed: {} code:{}",
                res.response.sub_msg.unwrap().as_str(),
                res.response.sub_code.unwrap().as_str()
            )));
        }
        Ok(res)
    }

    fn trade_order_settle(&self, biz_content: &TradeOrderSettleBiz) -> AliPayResult<TradeOrderSettleResponse> {
        let body = self.do_alipay(biz_content)?;
        let res: TradeOrderSettleResponse = serde_json::from_slice(&body)?;
        if res.response.code != Some("10000".to_string()) {
            log::debug!("{}", serde_json::to_string(&res)?);
            return Err(AliPayError(format!(
                "trade_page_refund failed: {} code:{}",
                res.response.sub_msg.unwrap().as_str(),
                res.response.sub_code.unwrap().as_str()
            )));
        }
        Ok(res)
    }

    fn trade_order_settle_query(&self, biz_content: &TradeOrderSettleQueryBiz) -> AliPayResult<TradeOrderSettleQueryResponse> {
        let body = self.do_alipay(biz_content)?;
        let res: TradeOrderSettleQueryResponse = serde_json::from_slice(&body)?;
        if res.response.code != Some("10000".to_string()) {
            log::debug!("{}", serde_json::to_string(&res)?);
            return Err(AliPayError(format!(
                "trade_page_refund failed: {} code:{}",
                res.response.msg.unwrap().as_str(),
                res.response.code.unwrap().as_str()
            )));
        }
        Ok(res)
    }

    fn trade_royalty_relation_bind(&self, biz_content: &TradeRoyaltyRelationBindBiz) -> AliPayResult<TradeRoyaltyRelationBindResponse> {
        let body = self.do_alipay(biz_content)?;
        let res: TradeRoyaltyRelationBindResponse = serde_json::from_slice(&body)?;
        if res.response.code != Some("10000".to_string()) {
            log::debug!("{}", serde_json::to_string(&res)?);
            return Err(AliPayError(format!(
                "trade_page_refund failed: {} code:{}",
                res.response.msg.unwrap().as_str(),
                res.response.code.unwrap().as_str()
            )));
        }
        Ok(res)
    }

    fn trade_royalty_relation_unbind(&self, biz_content: &TradeRoyaltyRelationUnBindBiz) -> AliPayResult<TradeRoyaltyRelationUnBindResponse> {
        let body = self.do_alipay(biz_content)?;
        let res: TradeRoyaltyRelationUnBindResponse = serde_json::from_slice(&body)?;
        if res.response.code != Some("10000".to_string()) {
            log::debug!("{}", serde_json::to_string(&res)?);
            return Err(AliPayError(format!(
                "trade_page_refund failed: {} code:{}",
                res.response.msg.unwrap().as_str(),
                res.response.code.unwrap().as_str()
            )));
        }
        Ok(res)
    }

    /// <https://opendocs.alipay.com/apis/api_1/alipay.trade.close>
    ///
    /// alipay.trade.close(统一收单交易关闭接口)
    fn trade_close(&self, biz_content: &TradeCloseBiz) -> AliPayResult<TradeCloseResponse> {
        let body = self.do_alipay(biz_content)?;
        let res: TradeCloseResponse = serde_json::from_slice(&body)?;
        if res.response.code != Some("10000".to_string()) {
            log::debug!("{}", serde_json::to_string(&res)?);
            return Err(AliPayError(format!(
                "trade_close failed: {} code:{}",
                res.response.sub_msg.unwrap().as_str(),
                res.response.sub_code.unwrap().as_str(),
            )));
        }
        Ok(res)
    }
    /// 自行实现签名文档 https://opendocs.alipay.com/common/02mse7?pathHash=096e611e
    ///
    /// 支付宝异步回调信息，用支付宝公钥验证签名，确认消息是支付宝服务器发出的,必须设置过异步通知的回调url连接和notify_url参数
    fn async_verify_sign(&self, raw_body: &[u8]) -> AliPayResult<bool> {
        let (source, sign, sign_type) = util::get_async_callback_msg_source(raw_body)?;
        let mut singer = builder().set_sign_type(&sign_type).build();
        singer.set_public_key(&self.alipay_public_key())?;
        return Ok(singer.verify(&source, &sign)?);
    }
}

/// 构造器
#[derive(Debug, Default, Clone, Copy)]
pub struct PayClientBuilder<'a> {
    api_url: Option<&'a str>, // `json:"-"`                    // 接口网关地址
    private_key: Option<&'a str>, // `json:"-"`                    // rsa私钥单行文本字符串
    public_key: Option<&'a str>, // `json:"-"`                    // rsa公钥单行文本字符串
    alipay_public_key: Option<&'a str>, // `json:"_"`                    // 证书解析出的支付宝公钥
    app_cert_sn: Option<&'a str>, // `json:"app_cert_sn"`          // 应用公钥证书 SN
    alipay_root_cert_sn: Option<&'a str>, // `json:"alipay_root_cert_sn"`  // 支付宝根证书 SN
    app_id: Option<&'a str>, // `json:"app_id"`               // 是	32	支付宝分配给开发者的应用ID	2014072300007148
    format: Option<&'a str>, // `json:"format,omitempty"`     // 否	40	仅支持JSON	JSON
    charset: Option<&'a str>, // `json:"charset"`              // 是	10	请求使用的编码格式，如utf-8,gbk,gb2312等	utf-8
    sign_type: Option<&'a str>, // `json:"sign_type"`            // 是	10	商户生成签名字符串所使用的签名算法类型，目前支持RSA2和RSA，推荐使用RSA2	RSA2
    version: Option<&'a str>, // `json:"version"`              // 是	3	调用的接口版本，固定为：1.0	1.0
    return_url: Option<&'a str>, // `json:"return_url,omitempty"` // 否 前台回跳地址 return_url 自动跳转回商户页面
    notify_url: Option<&'a str>, // `json:"notify_url,omitempty"`  支付宝服务器主动通知callback商户服务器里指定的页面http/https路径
}

impl PayClient {
    pub fn builder<'a>() -> PayClientBuilder<'a> {
        PayClientBuilder::default()
    }

    pub fn api_url(&self) -> String {
        self.api_url.to_owned()
    }

    pub fn private_key(&self) -> String {
        self.private_key.to_owned()
    }

    pub fn public_key(&self) -> String {
        self.public_key.to_owned()
    }

    pub fn app_cert_sn(&self) -> String {
        self.app_cert_sn.to_owned()
    }

    pub fn alipay_root_cert_sn(&self) -> String {
        self.alipay_root_cert_sn.to_owned()
    }

    pub fn alipay_public_key(&self) -> String {
        self.alipay_public_key.to_owned()
    }

    pub fn app_id(&self) -> String {
        self.app_id.to_owned()
    }

    pub fn format(&self) -> String {
        self.format.to_owned()
    }

    pub fn charset(&self) -> String {
        self.charset.to_owned()
    }

    pub fn sign_type(&self) -> String {
        self.sign_type.to_owned()
    }

    pub fn version(&self) -> String {
        self.version.to_owned()
    }

    pub fn return_url(&self) -> String {
        self.return_url.to_owned()
    }

    pub fn notify_url(&self) -> String {
        self.notify_url.to_owned()
    }

    pub fn set_notify_url(&mut self, notify_url: &str) {
        self.notify_url = notify_url.to_owned()
    }

    pub fn set_sign_type(&mut self, sign_type: &str) {
        self.sign_type = sign_type.to_owned()
    }

    pub fn set_charset(&mut self, charset: &str) {
        self.charset = charset.to_owned()
    }

    pub fn execute(&self, req: &mut impl Requester) -> AliPayResult<Vec<byte>> {
        let method = req.method();
        let payload = req.encode_payload()?;
        let mut request = http::Request::default();
        let mut client = http::Client::New();
        if method == "alipay.trade.precreate" {
            request = http::Request::New(
                http::Method::Get,
                &format!("{}?{}", self.api_url, payload),
                None,
            )?;
        } else {
            request = http::Request::New(http::Method::Post, &self.api_url, Some(payload.into()))?;
        }
        //  设置客户端请求的编号格式utf-8, 解决中文有乱码问题。
        request.Header.Set(
            "Content-Type",
            "application/x-www-form-urlencoded;charset=utf-8",
        );
        //  设置客户端接受返回数据的编号格式utf-8, 解决中文有乱码问题。
        request
            .Header
            .Set("Accept", "application/json;charset=utf-8");

        let res = client.Do(request.borrow_mut())?;
        match res.Body {
            Some(body) => Ok(body.into()),
            None => Err(AliPaySDKError::AliPayError("body is NONE".to_string())),
        }
    }
    /// 该方法对返回结果自带同步验签
    pub fn do_alipay(&self, biz_content: &impl BizContenter) -> AliPayResult<Vec<byte>> {
        // 同步验签
        let sync_verigy_sign = |response: &[byte]| -> AliPayResult<bool> {
            let result = std::str::from_utf8(response)?;
            let get_raw_source = || -> String {
                let key = biz::get_response_key(biz_content);
                json_get(result, &key)
            };

            let get_signture = || -> String { json_get(result, "sign") };

            let mut singer = builder().set_sign_type(self.sign_type().as_str()).build();

            singer.set_public_key(self.alipay_public_key().as_str())?;
            let passed = singer.verify(&get_raw_source(), &get_signture())?;
            if !passed {
                return Ok(false);
            }
            Ok(true)
        };
        match biz_content.method().as_str() {
            "alipay.trade.wap.pay" | "alipay.trade.page.pay" => {
                Ok(self.create_clien_page_form(biz_content)?)
            }
            "alipay.trade.app.pay" => self.create_clien_sdk_request(biz_content),
            _ => {
                let mut request = Request::new_with_config(self);
                request
                    .set_biz_content(biz_content)
                    .set_method(biz_content.method().as_str());
                let res = self.execute(&mut request)?;
                // dbg!(String::from_utf8(res.to_vec()));
                let is_pass = sync_verigy_sign(&res)?;
                if !is_pass {
                    return Err(AliPayError("syncVerifySign no passed!".to_string()));
                }
                Ok(res)
            }
        }
    }

    fn create_clien_page_form(&self, biz: &impl BizContenter) -> AliPayResult<Vec<byte>> {
        let encode_query = Request::new_with_config(self)
            .set_biz_content(biz)
            .set_method(biz.method().as_str())
            .encode_payload()?;

        let values = url::ParseQuery(encode_query.as_str())?;
        let mut parameters: HashMap<String, String> = HashMap::new();
        for (k, v) in values {
            parameters.insert(k, v[0].to_string());
        }

        let form = build_form(&self.api_url(), &mut parameters)?
            .as_bytes()
            .to_vec();
        Ok(form)
    }

    fn create_clien_sdk_request(&self, biz: &impl BizContenter) -> AliPayResult<Vec<byte>> {
        let client_sdk_request = Request::new_with_config(self)
            .set_biz_content(biz)
            .set_method(biz.method().as_str())
            .encode_payload()?;
        let form = client_sdk_request.as_bytes().to_vec();
        Ok(form)
    }
}

impl<'a> PayClientBuilder<'a> {
    /// 设置接口网关地址 ,dev沙箱环境为：https://openapi.alipaydev.com/gateway.do，prod正式环境为：https://openapi.alipay.com/gateway.do
    pub fn api_url(&mut self, api_url: &'a str) -> &mut Self {
        self.api_url = Some(api_url);
        self.borrow_mut()
    }

    /// 设置应用私钥 rsa私钥单行文本字符串,来源xxx_私钥.txt文件内容(PKCS1(非java适用))
    pub fn private_key(&mut self, private_key: &'a str) -> &mut Self {
        self.private_key = Some(private_key);
        self.borrow_mut()
    }
    /// 设置应用公钥 rsa公钥单行文本字符串,来源xxx_公钥.txt文件内容(PKCS1(非java适用))
    pub fn public_key(&mut self, public_key: &'a str) -> &mut Self {
        self.public_key = Some(public_key);
        self.borrow_mut()
    }
    /// 设置支付宝证书公钥, 来源 alipayCertPublicKey_RSA2.crt解析出的公钥
    pub fn alipay_public_key(&mut self, alipay_public_key: &'a str) -> &mut Self {
        self.alipay_public_key = Some(alipay_public_key);
        self.borrow_mut()
    }
    /// 设置应用公钥证书 SN,来源appCertPublicKey_2021000117650139.crt文件解析
    pub fn app_cert_sn(&mut self, app_cert_sn: &'a str) -> &mut Self {
        self.app_cert_sn = Some(app_cert_sn);
        self.borrow_mut()
    }
    /// 设置支付宝根证书 SN，来源alipayRootCert.crt文件解析
    pub fn alipay_root_cert_sn(&mut self, alipay_root_cert_sn: &'a str) -> &mut Self {
        self.alipay_root_cert_sn = Some(alipay_root_cert_sn);
        self.borrow_mut()
    }
    /// 设置应用ID
    pub fn app_id(&mut self, app_id: &'a str) -> &mut Self {
        self.app_id = Some(app_id);
        self.borrow_mut()
    }
    /// 设置请求格式，固定值json
    pub fn format_json(&mut self) -> &mut Self {
        self.format = Some("JSON");
        self.borrow_mut()
    }
    /// 设置字符集，固定值 utf-8
    pub fn charset_utf8(&mut self) -> &mut Self {
        self.charset = Some("utf-8");
        self.borrow_mut()
    }
    /// 设置签名类型，固定值RSA2
    pub fn sign_type_rsa2(&mut self) -> &mut Self {
        self.sign_type = Some("RSA2");
        self.borrow_mut()
    }
    /// 设置接口版本号，固定值1.0
    pub fn version_1_0(&mut self) -> &mut Self {
        self.version = Some("1.0");
        self.borrow_mut()
    }
    /// 设置前台回跳地址 return_url 自动跳转回商户页面
    pub fn return_url(&mut self, return_url: &'a str) -> &mut Self {
        self.return_url = Some(return_url);
        self.borrow_mut()
    }
    /// 支付宝服务器主动通知callback商户服务器里指定的页面http/https路径
    pub fn notify_url(&mut self, notify_url: &'a str) -> &mut Self {
        self.notify_url = Some(notify_url);
        self.borrow_mut()
    }

    /// 构造PayClient
    pub fn build(self) -> AliPayResult<impl Payer> {
        let mut p = PayClient::default();
        if let Some(api_url) = self.api_url {
            p.api_url = api_url.to_owned();
        } else {
            return Err(AliPaySDKError::AliPayError(
                "api_url is required".to_string(),
            ));
        }

        if let Some(private_key) = self.private_key {
            p.private_key = private_key.to_owned();
        } else {
            return Err(AliPaySDKError::AliPayError(
                "private_key is required".to_string(),
            ));
        }

        if let Some(public_key) = self.public_key {
            p.public_key = public_key.to_owned();
        } else {
            return Err(AliPaySDKError::AliPayError(
                "public_key is required".to_string(),
            ));
        }

        if let Some(app_cert_sn) = self.app_cert_sn {
            p.app_cert_sn = app_cert_sn.to_owned();
        } else {
            return Err(AliPaySDKError::AliPayError(
                "app_cert_sn is required".to_string(),
            ));
        }

        if let Some(alipay_root_cert_sn) = self.alipay_root_cert_sn {
            p.alipay_root_cert_sn = alipay_root_cert_sn.to_owned();
        } else {
            return Err(AliPayError("alipay_root_cert_sn is required".to_string()));
        }

        if let Some(app_id) = self.app_id {
            p.app_id = app_id.to_owned();
        } else {
            return Err(AliPayError("app_id is required".to_string()));
        }

        if let Some(alipay_public_key) = self.alipay_public_key {
            p.alipay_public_key = alipay_public_key.to_owned();
        } else {
            return Err(AliPayError("alipay_public_key is required".to_string()));
        }

        if let Some(format) = self.format {
            p.format = format.to_owned();
        } else {
            p.format = "JSON".to_owned();
        }

        if let Some(charset) = self.charset {
            p.charset = charset.to_owned();
        } else {
            p.charset = "utf-8".to_owned();
        }

        if let Some(sign_type) = self.sign_type {
            p.sign_type = sign_type.to_owned();
        } else {
            p.sign_type = "RSA2".to_owned();
        }

        if let Some(version) = self.version {
            p.version = version.to_owned();
        } else {
            p.version = "1.0".to_owned()
        }

        if let Some(return_url) = self.return_url {
            p.return_url = return_url.to_owned();
        }

        if let Some(notify_url) = self.notify_url {
            p.notify_url = notify_url.to_owned()
        }

        Ok(p)
    }
}
