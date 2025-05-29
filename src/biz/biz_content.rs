use core::hash;
use std::{clone, hash::Hash};
use gostd::strings;
use jsonmap::{JsonMap, JsonV};
use serde::{Deserialize, Serialize};

pub trait BizContenter: Serialize {
    fn method(&self) -> String;
    fn set(&mut self, key: &str, value: JsonV<String>);
    fn set_array(&mut self, key: &str, value: Vec<JsonV<String>>);
}

pub type BizObject = JsonMap<String>;
pub type V = JsonV<String>;

/// 返回 例如“alipay.trade.create.response”的返回字段key
pub fn get_response_key(biz: &impl BizContenter) -> String {
    let method = biz.method() + ".response";
    strings::ReplaceAll(method, ".", "_")
}
