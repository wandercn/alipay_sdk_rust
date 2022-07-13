//! 支付模块，包括各种支付接口函数。
#![allow(unused)]
use std::collections::HashMap;
use std::io::Result;
use std::io::{Error, ErrorKind};

use std::borrow::{Borrow, BorrowMut};

use crate::sign::{builder, Signer};
use gostd::builtin::byte;
use gostd::net::http;
use gostd::net::url;

use crate::biz::{
    self, BizContenter, TradeAppPayBiz, TradeCancelBiz, TradeCloseBiz, TradeCreateBiz,
    TradeFastpayRefundQueryBiz, TradePageRefundBiz, TradePayBiz, TradePrecreateBiz, TradeQueryBiz,
    TradeRefundBiz, TradeWapPayBiz,
};
use crate::request::{Request, Requester};
use crate::response::{
    self, TradeCancelResponse, TradeCloseResponse, TradeCreateResponse,
    TradeFastpayRefundQueryResponse, TradePageRefundResponse, TradePayResponse,
    TradePrecreateResponse, TradeQueryResponse, TradeRefundResponse,
};
use crate::util::{build_form, json_get};
pub trait Payer {
    fn get_api_url(&self) -> String;
    fn get_private_key(&self) -> String;
    fn get_public_key(&self) -> String;
    fn get_app_cert_sn(&self) -> String;
    fn get_alipay_root_cert_sn(&self) -> String;
    fn get_alipay_public_key(&self) -> String;
    fn get_app_id(&self) -> String;
    fn get_format(&self) -> String;
    fn get_charset(&self) -> String;
    fn get_sign_type(&self) -> String;
    fn get_version(&self) -> String;
    fn get_return_url(&self) -> String;
    fn set_sign_type(&mut self, sign_type: &str);
    fn set_charset(&mut self, charset: &str);
    fn execute(&self, request: &mut impl Requester) -> Result<Vec<byte>>;
    fn do_alipay(&self, biz_content: &impl BizContenter) -> Result<Vec<byte>>;
    fn trade_create(&self, biz_content: &TradeCreateBiz) -> Result<TradeCreateResponse>;
    fn trade_pay(&self, biz_content: &TradePayBiz) -> Result<TradePayResponse>;
    fn trade_precreate(&self, biz_content: &TradePrecreateBiz) -> Result<TradePrecreateResponse>;
    fn trade_app_pay(&self, biz_content: &TradeAppPayBiz) -> Result<String>;
    fn trade_wap_pay(&self, biz_content: &TradeWapPayBiz) -> Result<String>;
    fn trade_page_pay(&self, biz_content: &TradePayBiz) -> Result<String>;
    fn trede_query(&self, biz_content: &TradeQueryBiz) -> Result<TradeQueryResponse>;
    fn trade_cancel(&self, biz_content: &TradeCancelBiz) -> Result<TradeCancelResponse>;
    fn trade_refund(&self, biz_content: &TradeRefundBiz) -> Result<TradeRefundResponse>;
    fn trade_page_refund(
        &self,
        biz_content: &TradePageRefundBiz,
    ) -> Result<TradePageRefundResponse>;
    fn trade_fastpay_refund_query(
        &self,
        biz_content: &TradeFastpayRefundQueryBiz,
    ) -> Result<TradeFastpayRefundQueryResponse>;
    fn trade_close(&self, biz_content: &TradeCloseBiz) -> Result<TradeCloseResponse>;
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
/// fn new_pay_client() -> Result<impl Payer> {
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
}

impl Payer for PayClient {
    fn get_api_url(&self) -> String {
        self.api_url.to_owned()
    }

    fn get_private_key(&self) -> String {
        self.private_key.to_owned()
    }

    fn get_public_key(&self) -> String {
        self.public_key.to_owned()
    }

    fn get_app_cert_sn(&self) -> String {
        self.app_cert_sn.to_owned()
    }

    fn get_alipay_root_cert_sn(&self) -> String {
        self.alipay_root_cert_sn.to_owned()
    }

    fn get_alipay_public_key(&self) -> String {
        self.alipay_public_key.to_owned()
    }

    fn get_app_id(&self) -> String {
        self.app_id.to_owned()
    }

    fn get_format(&self) -> String {
        self.format.to_owned()
    }

    fn get_charset(&self) -> String {
        self.charset.to_owned()
    }

    fn get_sign_type(&self) -> String {
        self.sign_type.to_owned()
    }

    fn get_version(&self) -> String {
        self.version.to_owned()
    }

    fn get_return_url(&self) -> String {
        self.return_url.to_owned()
    }

    fn set_sign_type(&mut self, sign_type: &str) {
        self.sign_type = sign_type.to_owned()
    }

    fn set_charset(&mut self, charset: &str) {
        self.charset = charset.to_owned()
    }

    fn execute(&self, req: &mut impl Requester) -> Result<Vec<byte>> {
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
            request = http::Request::New(
                http::Method::Post,
                &self.api_url,
                Some(payload.as_bytes().to_vec()),
            )?;
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
            Some(body) => Ok(body),
            None => Err(Error::new(ErrorKind::Other, "body is NONE")),
        }
    }
    /// 该方法对返回结果自带同步验签
    fn do_alipay(&self, biz_content: &impl BizContenter) -> Result<Vec<byte>> {
        // 同步验签
        let sync_verigy_sign = |response: &[byte]| -> Result<bool> {
            if let Ok(result) = std::str::from_utf8(response) {
                let get_raw_source = || -> String {
                    let key = biz::get_response_key(biz_content);
                    json_get(result, &key)
                };

                let get_signture = || -> String { json_get(result, "sign") };

                let mut singer = builder()
                    .set_sign_type(self.get_sign_type().as_str())
                    .build();

                singer.set_public_key(self.get_alipay_public_key().as_str())?;
                let passed = singer.verify(&get_raw_source(), &get_signture())?;
                if !passed {
                    return Ok(false);
                }
                Ok(true)
            } else {
                Err(Error::new(ErrorKind::Other, "from_utf8 response failed!"))
            }
        };

        match biz_content.method().as_str() {
            "alipay.trade.wap.pay" | "alipay.trade.page.pay" => {
                self.create_clien_page_form(biz_content)
            }
            "alipay.trade.app.pay" => self.create_clien_sdkt_request(biz_content),
            _ => {
                let mut request = Request::new_with_config(self.borrow());
                request
                    .set_biz_content(biz_content)
                    .set_method(biz_content.method().as_str());
                let res = self.execute(&mut request)?;
                let is_pass = sync_verigy_sign(&res)?;
                if !is_pass {
                    return Err(Error::new(ErrorKind::Other, "syncVerifySign no passed!"));
                }
                Ok(res)
            }
        }
    }

    fn trade_create(&self, biz_content: &TradeCreateBiz) -> Result<TradeCreateResponse> {
        let body = self.do_alipay(biz_content)?;
        let res: TradeCreateResponse = serde_json::from_slice(&body)?;
        Ok(res)
    }

    /// <https://opendocs.alipay.com/apis/api_1/alipay.trade.pay>
    ///
    /// alipay.trade.pay(统一收单交易支付接口)
    fn trade_pay(&self, biz_content: &TradePayBiz) -> Result<TradePayResponse> {
        let body = self.do_alipay(biz_content)?;
        let res: TradePayResponse = serde_json::from_slice(&body)?;
        Ok(res)
    }

    /// <https://opendocs.alipay.com/apis/api_1/alipay.trade.precreate>
    ///
    /// alipay.trade.precreate(统一收单线下交易预创建)当面付-商家生成条码
    fn trade_precreate(&self, biz_content: &TradePrecreateBiz) -> Result<TradePrecreateResponse> {
        let body = self.do_alipay(biz_content)?;
        let res: TradePrecreateResponse = serde_json::from_slice(&body)?;
        Ok(res)
    }

    /// <https://opendocs.alipay.com/apis/api_1/alipay.trade.app.pay>
    ///
    /// alipay.trade.app.pay(app支付接口2.0)后端只生成form数据给前端调用
    fn trade_app_pay(&self, biz_content: &TradeAppPayBiz) -> Result<String> {
        let body = self.do_alipay(biz_content)?;
        let res = String::from_utf8(body).map_err(|err| Error::new(ErrorKind::Other, err))?;
        Ok(res)
    }

    /// <https://opendocs.alipay.com/apis/api_1/alipay.trade.wap.pay>
    ///
    /// alipay.trade.wap.pay(手机网站支付接口2.0)后端只生成form数据给前端调用
    fn trade_wap_pay(&self, biz_content: &TradeWapPayBiz) -> Result<String> {
        let body = self.do_alipay(biz_content)?;
        let res = String::from_utf8(body).map_err(|err| Error::new(ErrorKind::Other, err))?;
        Ok(res)
    }

    /// <https://opendocs.alipay.com/apis/api_1/alipay.trade.page.pay>
    ///
    /// alipay.trade.page.pay(统一收单下单并支付页面接口)后端只生成form数据给前端调用
    fn trade_page_pay(&self, biz_content: &TradePayBiz) -> Result<String> {
        let body = self.do_alipay(biz_content)?;
        let res = String::from_utf8(body).map_err(|err| Error::new(ErrorKind::Other, err))?;
        Ok(res)
    }

    /// <https://opendocs.alipay.com/apis/api_1/alipay.trade.query>
    ///
    /// alipay.trade.query(统一收单线下交易查询)
    fn trede_query(&self, biz_content: &TradeQueryBiz) -> Result<TradeQueryResponse> {
        let body = self.do_alipay(biz_content)?;
        let res: TradeQueryResponse = serde_json::from_slice(&body)?;
        Ok(res)
    }

    ///  <https://opendocs.alipay.com/apis/api_1/alipay.trade.cancel>
    ///
    /// alipay.trade.cancel(统一收单交易撤销接口)
    fn trade_cancel(&self, biz_content: &TradeCancelBiz) -> Result<TradeCancelResponse> {
        let body = self.do_alipay(biz_content)?;
        let res: TradeCancelResponse = serde_json::from_slice(&body)?;
        Ok(res)
    }

    /// <https://opendocs.alipay.com/apis/api_1/alipay.trade.refund>
    ///
    /// alipay.trade.refund(统一收单交易退款接口)
    fn trade_refund(&self, biz_content: &TradeRefundBiz) -> Result<TradeRefundResponse> {
        let body = self.do_alipay(biz_content)?;
        let res: TradeRefundResponse = serde_json::from_slice(&body)?;
        Ok(res)
    }

    /// <https://opendocs.alipay.com/apis/api_1/alipay.trade.page.refund>
    ///
    /// alipay.trade.page.refund(统一收单退款页面接口)
    fn trade_page_refund(
        &self,
        biz_content: &TradePageRefundBiz,
    ) -> Result<TradePageRefundResponse> {
        let body = self.do_alipay(biz_content)?;
        let res: TradePageRefundResponse = serde_json::from_slice(&body)?;
        Ok(res)
    }
    /// <https://opendocs.alipay.com/apis/api_1/alipay.trade.fastpay.refund.query>
    ///
    /// alipay.trade.fastpay.refund.query(统一收单交易退款查询)
    fn trade_fastpay_refund_query(
        &self,
        biz_content: &TradeFastpayRefundQueryBiz,
    ) -> Result<TradeFastpayRefundQueryResponse> {
        let body = self.do_alipay(biz_content)?;
        let res: TradeFastpayRefundQueryResponse = serde_json::from_slice(&body)?;
        Ok(res)
    }

    /// <https://opendocs.alipay.com/apis/api_1/alipay.trade.close>
    ///
    /// alipay.trade.close(统一收单交易关闭接口)
    fn trade_close(&self, biz_content: &TradeCloseBiz) -> Result<TradeCloseResponse> {
        let body = self.do_alipay(biz_content)?;
        let res: TradeCloseResponse = serde_json::from_slice(&body)?;
        Ok(res)
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
}

impl PayClient {
    pub fn builder<'a>() -> PayClientBuilder<'a> {
        PayClientBuilder::default()
    }

    pub fn create_clien_page_form(&self, biz: &impl BizContenter) -> Result<Vec<byte>> {
        let encode_query = Request::new_with_config(self.borrow())
            .set_biz_content(biz)
            .set_method(biz.method().as_str())
            .encode_payload()?;

        let values = url::ParseQuery(encode_query.as_str())?;
        let mut parameters: HashMap<String, String> = HashMap::new();
        for (k, v) in values {
            parameters.insert(k, v[0].to_string());
        }
        let form = build_form(&self.get_api_url(), &mut parameters)
            .as_bytes()
            .to_vec();
        Ok(form)
    }

    pub fn create_clien_sdkt_request(&self, biz: &impl BizContenter) -> Result<Vec<byte>> {
        let client_sdk_request = Request::new_with_config(self.borrow())
            .set_biz_content(biz)
            .set_method(biz.method().as_str())
            .encode_payload()?;
        let form = client_sdk_request.as_bytes().to_vec();
        Ok(form)
    }
}

impl<'a> PayClientBuilder<'a> {
    pub fn api_url(&mut self, api_url: &'a str) -> &mut Self {
        self.api_url = Some(api_url);
        self.borrow_mut()
    }

    pub fn private_key(&mut self, private_key: &'a str) -> &mut Self {
        self.private_key = Some(private_key);
        self.borrow_mut()
    }

    pub fn public_key(&mut self, public_key: &'a str) -> &mut Self {
        self.public_key = Some(public_key);
        self.borrow_mut()
    }

    pub fn alipay_public_key(&mut self, alipay_public_key: &'a str) -> &mut Self {
        self.alipay_public_key = Some(alipay_public_key);
        self.borrow_mut()
    }

    pub fn app_cert_sn(&mut self, app_cert_sn: &'a str) -> &mut Self {
        self.app_cert_sn = Some(app_cert_sn);
        self.borrow_mut()
    }

    pub fn alipay_root_cert_sn(&mut self, alipay_root_cert_sn: &'a str) -> &mut Self {
        self.alipay_root_cert_sn = Some(alipay_root_cert_sn);
        self.borrow_mut()
    }

    pub fn app_id(&mut self, app_id: &'a str) -> &mut Self {
        self.app_id = Some(app_id);
        self.borrow_mut()
    }

    pub fn format_json(&mut self) -> &mut Self {
        self.format = Some("JSON");
        self.borrow_mut()
    }

    pub fn charset_utf8(&mut self) -> &mut Self {
        self.charset = Some("utf-8");
        self.borrow_mut()
    }

    pub fn sign_type_rsa2(&mut self) -> &mut Self {
        self.sign_type = Some("RSA2");
        self.borrow_mut()
    }

    pub fn version_1_0(&mut self) -> &mut Self {
        self.version = Some("1.0");
        self.borrow_mut()
    }

    pub fn return_url(&mut self, return_url: &'a str) -> &mut Self {
        self.return_url = Some(return_url);
        self.borrow_mut()
    }

    pub fn build(self) -> Result<impl Payer> {
        let mut p = PayClient::default();
        if let Some(api_url) = self.api_url {
            p.api_url = api_url.to_owned();
        } else {
            return Err(Error::new(ErrorKind::Other, "api_url is required"));
        }

        if let Some(private_key) = self.private_key {
            p.private_key = private_key.to_owned();
        } else {
            return Err(Error::new(ErrorKind::Other, "private_key is required"));
        }

        if let Some(public_key) = self.public_key {
            p.public_key = public_key.to_owned();
        } else {
            return Err(Error::new(ErrorKind::Other, "public_key is required"));
        }

        if let Some(app_cert_sn) = self.app_cert_sn {
            p.app_cert_sn = app_cert_sn.to_owned();
        } else {
            return Err(Error::new(ErrorKind::Other, "app_cert_sn is required"));
        }

        if let Some(alipay_root_cert_sn) = self.alipay_root_cert_sn {
            p.alipay_root_cert_sn = alipay_root_cert_sn.to_owned();
        } else {
            return Err(Error::new(
                ErrorKind::Other,
                "alipay_root_cert_sn is required",
            ));
        }

        if let Some(app_id) = self.app_id {
            p.app_id = app_id.to_owned();
        } else {
            return Err(Error::new(ErrorKind::Other, "app_id is required"));
        }

        if let Some(alipay_public_key) = self.alipay_public_key {
            p.alipay_public_key = alipay_public_key.to_owned();
        } else {
            return Err(Error::new(
                ErrorKind::Other,
                "alipay_public_key is required",
            ));
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
        // else {
        //     return Err(Error::new(ErrorKind::Other, "return_url is required"));
        // }

        Ok(p)
    }
}
