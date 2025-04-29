//! 其他帮助函数模块
// #![allow(unused)]

use super::biz::BizContenter;
use crate::error::AliPayResult;
use gostd::builtin::len;
use gostd::io::StringWriter;
use gostd::strings;
use gostd::time;

use serde_json;
use std::collections::HashMap;

use uuid::Uuid;

pub fn get_biz_content_str(w: &impl BizContenter) -> String {
    match serde_json::to_string(&w) {
        Ok(res) => res,
        Err(_) => "".to_owned(),
    }
}

pub fn get_now_beijing_time_str() -> String {
    let layout = "2006-01-02 15:04:05";
    let loc = time::FixedZone("CST", 3600 * 8);
    let now_time = time::Now().In(loc);
    now_time.Format(layout)
}

pub fn get_out_trade_no() -> String {
    Uuid::new_v4().to_string()
}

pub fn build_form(
    base_url: &str,
    parameters: &mut HashMap<String, String>,
) -> AliPayResult<String> {
    let mut buf = strings::Builder::new();
    buf.WriteString("<form name=\"alipaysubmit\" method=\"post\" action=\"")?;
    buf.WriteString(base_url)?;
    buf.WriteString("?charset=utf-8")?;
    buf.WriteString("\">\n")?;
    buf.WriteString(&build_hidden_fields(parameters)?)?;
    buf.WriteString("<input type=\"submit\" value=\"立即支付\" style=\"display:none\" >\n")?;
    buf.WriteString("</form>\n")?;
    buf.WriteString("<script>document.forms['alipaysubmit'].submit();</script>")?;
    Ok(buf.String())
}

fn build_hidden_fields(parameters: &mut HashMap<String, String>) -> AliPayResult<String> {
    if parameters.is_empty() {
        return Ok("".to_string());
    }
    let mut buf = strings::Builder::new();
    for (key, value) in parameters {
        if value.is_empty() {
            continue;
        }
        buf.WriteString(&build_hidden_field(key, value)?)?;
    }
    Ok(buf.String())
}

fn build_hidden_field(key: &str, value: &str) -> AliPayResult<String> {
    let mut buf = strings::Builder::new();
    buf.WriteString("<input type=\"hidden\" name=\"")?;
    buf.WriteString(key)?;
    buf.WriteString("\" value=\"")?;
    // 转义双引号
    let a = strings::ReplaceAll(value, "\"", "&quot;");
    buf.WriteString(&a)?;
    buf.WriteString("\">\n")?;
    Ok(buf.String())
}

// 只支持value是{}或[]或""包裹的key，不支持数字
pub fn json_get(result: &str, key: &str) -> String {
    let len = len!(key);
    let i = strings::LastIndex(result, key);
    let mut current = result.as_bytes()[i as usize + len];
    let mut index = i as usize + len;
    while current != b':' {
        index += 1;
        current = result.as_bytes()[index];
    }
    let mut start = index + 1;
    let end: usize;
    index += 1;
    current = result.as_bytes()[index];
    let mut left_brackets = 0_usize;
    if current == b'{' || current == b'[' {
        loop {
            index += 1;
            current = result.as_bytes()[index];
            if current == b'{' || current == b'[' {
                left_brackets += 1;
            }

            if (current == b']' || current == b'}') && left_brackets == 0 {
                break;
            }

            if (current == b']' || current == b'}') && left_brackets > 0 {
                left_brackets -= 1;
            }
        }
        end = index + 1;
    } else {
        index += 1;
        current = result.as_bytes()[index];
        start = index;
        while current != b'"' {
            index += 1;
            current = result.as_bytes()[index];
        }
        end = index;
    }
    match String::from_utf8(result.as_bytes()[start..end].to_vec()) {
        Ok(v) => v,
        Err(_) => "".to_string(),
    }
}

use gostd::net::url;

// 获取支付宝CallBack异步消息的待签名字符串和签名
// 自行实现签名文档 https://opendocs.alipay.com/common/02mse7?pathHash=096e611e
// 返回值 source - 签名字符串 , sign - 签名 , sign_type - 签名类型
pub fn get_async_callback_msg_source(raw_body: &[u8]) -> AliPayResult<(String, String, String)> {
    // 解析 URL 查询字符串
    let raw_str = std::str::from_utf8(raw_body)?;
    let values = url::ParseQuery(&raw_str)?;

    let sign_type = values.Get("sign_type");
    // ParseQuery函数会把sign字符串的+解析成空格，需要还原回去
    let sign = values.Get("sign").replace(" ", "+");

    // 待签名字符串不包括sign和sign_type,需要删除
    let mut filtered_values = values.clone();
    filtered_values.Del("sign");
    filtered_values.Del("sign_type");

    // 按字典排序
    let mut keys: Vec<String> = vec![];
    for (k, _) in &filtered_values {
        keys.push(k.to_string());
    }
    keys.sort();

    // 拼接成待签名字符串
    let source: String = keys
        .iter()
        .map(|k| format!("{}={}", k.to_string(), filtered_values.to_owned().Get(k)))
        .collect::<Vec<String>>()
        .join("&");

    Ok((source, sign, sign_type))
}

use base64::{engine::general_purpose, DecodeError, Engine as _};

pub fn base64_encode<T>(input: T) -> String
where
    T: AsRef<[u8]>,
{
    general_purpose::STANDARD.encode(input)
}

pub fn base64_decode<T>(input: T) -> Result<Vec<u8>, DecodeError>
where
    T: AsRef<[u8]>,
{
    general_purpose::STANDARD.decode(input)
}
