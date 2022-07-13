#![allow(unused)]
use std::collections::HashMap;
use std::io::Result;
use std::io::{Error, ErrorKind};

use std::borrow::{Borrow, BorrowMut};

use crate::sign::{builder, Signer};
use gostd::builtin::byte;
use gostd::net::http;
use gostd::net::url;

use crate::biz::{self, BizContenter};
use crate::request::{Request, Requester};
use crate::response::{self, TradeCreateResponse};
use crate::util::{build_form, json_get};
use crate::{api, request};
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
    fn do_alipay(&self, biz: &impl BizContenter) -> Result<Vec<byte>>;
    fn trade_create(&self, biz: biz::TradeCreateBiz) -> Result<TradeCreateResponse>;
}

#[derive(Debug, Default)]
pub struct PayClient {
    api_url: String,             // `json:"-"`                    // 接口网关地址
    private_key: String,         // `json:"-"`                    // rsa私钥单行文本字符串
    public_key: String,          // `json:"-"`                    // rsa公钥单行文本字符串
    alipay_public_key: String,   // `json:"_"`                    // 证书解析出的支付宝公钥
    app_cert_sn: String,         // `json:"app_cert_sn"`          // 应用公钥证书 SN
    alipay_root_cert_sn: String, // `json:"alipay_root_cert_sn"`  // 支付宝根证书 SN
    app_id: String, // `json:"app_id"`               // 是	32	支付宝分配给开发者的应用ID	2014072300007148
    format: String, // `json:"format,omitempty"`     // 否	40	仅支持JSON	JSON
    charset: String, // `json:"charset"`              // 是	10	请求使用的编码格式，如utf-8,gbk,gb2312等	utf-8
    sign_type: String, // `json:"sign_type"`            // 是	10	商户生成签名字符串所使用的签名算法类型，目前支持RSA2和RSA，推荐使用RSA2	RSA2
    version: String,   // `json:"version"`              // 是	3	调用的接口版本，固定为：1.0	1.0
    return_url: String, // `json:"return_url,omitempty"` // 否 前台回跳地址 return_url 自动跳转回商户页面
    is_prod: bool,      //`json:"-"`                    // 是否生产环境
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

    fn trade_create(&self, biz: biz::TradeCreateBiz) -> Result<TradeCreateResponse> {
        let mut res = TradeCreateResponse::default();
        let body = self.do_alipay(&biz)?;
        res = serde_json::from_slice(&body)?;
        Ok(res)
    }
}

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
