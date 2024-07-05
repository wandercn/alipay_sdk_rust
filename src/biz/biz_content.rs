use gostd::strings;
use serde::Serialize;
use std::{collections::HashMap, hash::BuildHasher};

/// 独立请求参数接口 BizContenter
pub trait BizContenter<V>: Serialize
where
    V: Serialize + Clone,
{
    fn method(&self) -> String;
    fn set(&mut self, key: &str, value: V);
}

pub type BizObject<V> = HashMap<String, V>;

/// 返回 例如“alipay.trade.create.response”的返回字段key
pub fn get_response_key<V>(biz: &impl BizContenter<V>) -> String
where
    V: Serialize + Clone  ,
{
    let method = biz.method() + ".response";
    strings::ReplaceAll(method, ".", "_")
}
