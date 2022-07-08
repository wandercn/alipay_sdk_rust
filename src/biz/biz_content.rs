use gostd::strings;
use serde::Serialize;
use std::collections::HashMap;
pub trait BizContenter: Serialize {
    fn method(&self) -> String;
    fn set(&mut self, key: &str, value: &str);
}

pub type BizObject = HashMap<String, String>;

pub fn get_response_key(biz: &impl BizContenter) -> String {
    let method = biz.method() + ".response";
    strings::ReplaceAll(method, ".", "_")
}
