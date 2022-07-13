use gostd::strings;
use serde::Serialize;
use std::collections::HashMap;

/// 独立请求参数接口 BizContenter
pub trait BizContenter: Serialize {
    fn method(&self) -> String;
    fn set(&mut self, key: &str, value: &str);
}

pub type BizObject = HashMap<String, String>;

/// 返回 例如“alipay.trade.create.response”的返回字段key
pub fn get_response_key(biz: &impl BizContenter) -> String {
    let method = biz.method() + ".response";
    strings::ReplaceAll(method, ".", "_")
}
