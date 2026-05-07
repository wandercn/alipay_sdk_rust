use jsonmap::JsonV;
use serde::Serialize;

pub trait BizContenterV3: Serialize {
    fn path(&self) -> String;
    fn http_method(&self) -> String {
        "POST".to_string()
    }
    fn set(&mut self, key: &str, value: JsonV<String>);
}

pub type BizObjectV3 = jsonmap::JsonMap<String>;
pub type V3 = JsonV<String>;

mod trade_pay;
pub use trade_pay::*;
mod trade_create;
pub use trade_create::*;
mod trade_precreate;
pub use trade_precreate::*;
mod trade_query;
pub use trade_query::*;
mod trade_refund;
pub use trade_refund::*;
mod trade_cancel;
pub use trade_cancel::*;
mod trade_close;
pub use trade_close::*;
mod trade_fastpay_refund_query;
pub use trade_fastpay_refund_query::*;
mod trade_app_pay;
pub use trade_app_pay::*;
mod trade_wap_pay;
pub use trade_wap_pay::*;
mod trade_page_pay;
pub use trade_page_pay::*;
mod trade_order_settle;
pub use trade_order_settle::*;
mod trade_order_settle_query;
pub use trade_order_settle_query::*;
mod trade_royalty_relation_bind;
pub use trade_royalty_relation_bind::*;
mod trade_royalty_relation_unbind;
pub use trade_royalty_relation_unbind::*;
