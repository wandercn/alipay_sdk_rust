#![allow(unused)]
use gostd::{net::url, strings};
use serde::{Deserialize, Serialize};

use crate::{
    biz::BizContenter,
    pay::{PayClient, Payer},
    sign::{builder, Signer},
    util::{get_biz_content_str, get_now_beijing_time_str, response_to_json},
};
use std::{
    borrow::{Borrow, BorrowMut},
    collections::HashMap,
    io::Result,
};

pub trait Requester {
    fn new_with_config(pay_config: &impl Payer) -> Self;
    fn set_method(&mut self, method: &str) -> &mut Self;
    fn method(&self) -> String;
    fn set_biz_content(&mut self, b: &impl BizContenter) -> &mut Self;
    fn encode_payload(&mut self) -> Result<String>;
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Request {
    // 是否必填	最大长度	描述	示例值
    private_key: String, //`json:"-"`                        // rsa私钥单行文本字符串
    public_key: String,  //`json:"-"`                        // rsa公钥单行文本字符串
    app_cert_sn: String, //`json:"app_cert_sn"`              // 应用公钥证书 SN
    alipay_root_cert_sn: String, //`json:"alipay_root_cert_sn"`      // 支付宝根证书 SN
    app_id: String, //`json:"app_id"`                   // 是	32	支付宝分配给开发者的应用ID	2014072300007148
    app_auth_token: String, //`json:"app_auth_token,omitempty"` // 否	40	详见应用授权概述
    method: String, //`json:"method"`                   // 是	128	接口名称	alipay.trade.create
    format: String, //`json:"format,omitempty"`         // 否	40	仅支持JSON	JSON
    charset: String, //`json:"charset"`                  // 是	10	请求使用的编码格式，如utf-8,gbk,gb2312等	utf-8
    sign_type: String, //`json:"sign_type"`                // 是	10	商户生成签名字符串所使用的签名算法类型，目前支持RSA2和RSA，推荐使用RSA2	RSA2
    sign: String, //`json:"sign"`                     // 是	344	商户请求参数的签名串，详见签名	详见示例
    timestamp: String, //`json:"timestamp"`                // 是	19	发送请求的时间，格式"yyyy-MM-dd HH:mm:ss"	2014-07-24 03:07:50
    version: String,   //`json:"version"`                  // 是	3	调用的接口版本，固定为：1.0	1.0
    notify_url: String, //`json:"notify_url,omitempty"`     // 否	256	支付宝服务器主动通知商户服务器里指定的页面http/https路径。	http://api.test.alipay.net/atinterface/receive_notify.htm
    return_url: String, //`json:"return_url,omitempty"`     // 否 前台回跳地址 return_url 自动跳转回商户页面
    biz_content: String, //`json:"biz_content"`              //是		请求参数的集合，最大长度不限，除公共参数外所有请求参数都必须放在这个参数中传递，具体参照各产品快速接入文档
}

impl Request {
    fn set_timestamp(&mut self) {
        self.timestamp = get_now_beijing_time_str()
    }

    fn get_sorted_sign_source(&self) -> Result<String> {
        let mut m = HashMap::<String, String>::new();
        let v = serde_json::to_string(self.borrow())?;
        m = serde_json::from_str(v.as_str())?;
        let mut query_list = Vec::<String>::new();
        m.iter().for_each(|(k, v)| {
            if !v.is_empty() && k != "sign" {
                let query = format!("{}={}", k, strings::TrimSpace(v));
                query_list.push(query);
            }
        });
        query_list.sort();

        Ok(strings::Join(query_list, "&"))
    }
}

impl Requester for Request {
    fn new_with_config(pay_config: &impl Payer) -> Self {
        Self {
            private_key: pay_config.get_private_key(),
            public_key: pay_config.get_alipay_public_key(),
            app_cert_sn: pay_config.get_app_cert_sn(),
            alipay_root_cert_sn: pay_config.get_app_cert_sn(),
            app_id: pay_config.get_app_id(),
            format: pay_config.get_format(),
            charset: pay_config.get_charset(),
            sign_type: pay_config.get_sign_type(),
            version: pay_config.get_version(),
            timestamp: get_now_beijing_time_str(),
            ..Default::default()
        }
    }

    fn set_method(&mut self, method: &str) -> &mut Self {
        self.method = method.to_owned();
        self.borrow_mut()
    }

    fn method(&self) -> String {
        self.method.to_string()
    }

    fn set_biz_content(&mut self, b: &impl BizContenter) -> &mut Self {
        self.biz_content = get_biz_content_str(b);
        self.borrow_mut()
    }

    fn encode_payload(&mut self) -> Result<String> {
        let mut signer = builder().set_sign_type(&self.sign_type).build();
        signer.set_private_key(&self.private_key)?;
        let sign_sorted_source = self.get_sorted_sign_source()?;
        let signture = signer.sign(&sign_sorted_source)?;

        let mut encode_list = Vec::<String>::new();
        strings::Split(&sign_sorted_source, "&")
            .iter()
            .for_each(|v| {
                let mut splites: Vec<String> = strings::Split(v, "=")
                    .iter()
                    .map(|x| x.to_string())
                    .collect();
                splites[1] = url::QueryEscape(&splites[1]);
                encode_list.push(strings::Join(splites, "="));
            });
        let encode_query =
            strings::Join(encode_list, "&") + "&sign=" + &url::QueryEscape(&signture);
        Ok(encode_query)
    }
}
