use gostd::strings;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, hash::BuildHasher};

/// 独立请求参数接口 BizContenter
/// 通过枚举类型支持 HashMap 内的Vlaue值可以同时存储 浮点型，整数，字符串，数组，对象类型。
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum V {
    Boolean(bool),
    Integer(i32),
    Float(f64),
    String(String),
    Array(Vec<V>),
    Object(HashMap<String, V>),
}

impl From<bool> for V {
    fn from(value: bool) -> Self {
        V::Boolean(value)
    }
}

impl From<i32> for V {
    fn from(value: i32) -> Self {
        V::Integer(value)
    }
}

impl From<f64> for V {
    fn from(value: f64) -> Self {
        V::Float(value)
    }
}

impl From<String> for V {
    fn from(value: String) -> Self {
        V::String(value)
    }
}
impl From<&str> for V {
    fn from(value: &str) -> Self {
        V::String(value.to_owned())
    }
}

impl From<Vec<V>> for V {
    fn from(value: Vec<V>) -> Self {
        V::Array(value)
    }
}

impl From<HashMap<String, V>> for V {
    fn from(value: HashMap<String, V>) -> Self {
        V::Object(value)
    }
}

pub trait BizContenter: Serialize {
    fn method(&self) -> String;
    fn set(&mut self, key: &str, value: V);
}

pub type BizObject = HashMap<String, V>;

/// 返回 例如“alipay.trade.create.response”的返回字段key
pub fn get_response_key(biz: &impl BizContenter) -> String {
    let method = biz.method() + ".response";
    strings::ReplaceAll(method, ".", "_")
}
