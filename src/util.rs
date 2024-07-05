//! 其他帮助函数模块
// #![allow(unused)]

use super::biz::BizContenter;
use gostd::builtin::len;
use gostd::io::StringWriter;
use gostd::strings;
use gostd::time;
use serde::Serialize;
use serde_json;
use std::collections::HashMap;
use std::hash::BuildHasher;
use uuid::Uuid;

pub fn get_biz_content_str<V>(w: &impl BizContenter<V>) -> String
where
    V: Serialize + Clone  ,
{
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

pub fn build_form(base_url: &str, parameters: &mut HashMap<String, String>) -> String {
    let mut buf = strings::Builder::new();
    buf.WriteString("<form name=\"alipaysubmit\" method=\"post\" action=\"");
    buf.WriteString(base_url);
    buf.WriteString("?charset=utf-8");
    buf.WriteString("\">\n");
    buf.WriteString(&build_hidden_fields(parameters));
    buf.WriteString("<input type=\"submit\" value=\"立即支付\" style=\"display:none\" >\n");
    buf.WriteString("</form>\n");
    buf.WriteString("<script>document.forms['alipaysubmit'].submit();</script>");
    buf.String()
}

fn build_hidden_fields(parameters: &mut HashMap<String, String>) -> String {
    if parameters.is_empty() {
        return "".to_string();
    }
    let mut buf = strings::Builder::new();
    for (key, value) in parameters {
        if value.is_empty() {
            continue;
        }
        buf.WriteString(&build_hidden_field(key, value));
    }
    buf.String()
}

fn build_hidden_field(key: &str, value: &str) -> String {
    let mut buf = strings::Builder::new();
    buf.WriteString("<input type=\"hidden\" name=\"");
    buf.WriteString(key);
    buf.WriteString("\" value=\"");
    // 转义双引号
    let a = strings::ReplaceAll(value, "\"", "&quot;");
    buf.WriteString(&a);
    buf.WriteString("\">\n");
    buf.String()
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
        while current != b'}' && current != b']' {
            index += 1;
            current = result.as_bytes()[index];
            if current == b'{' || current == b'[' {
                left_brackets += 1;
            }

            if (current == b']' || current == b'}') && left_brackets > 0 {
                index += 1;
                left_brackets -= 1;
                current = result.as_bytes()[index];
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
