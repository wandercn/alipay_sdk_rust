#![allow(unused)]
//! 请求参数模块,构造各种接口的独立请求参数。
mod biz_content;
pub use biz_content::*;
mod trade_app_pay;
pub use trade_app_pay::*;
mod trade_cancel;
pub use trade_cancel::*;
mod trade_close;
pub use trade_close::*;
mod trade_create;
pub use trade_create::*;
mod trade_fastpay_refund_query;
pub use trade_fastpay_refund_query::*;
mod trade_page_pay;
pub use trade_page_pay::*;

mod trade_page_refund;
pub use trade_page_refund::*;
mod trade_pay;
pub use trade_pay::*;
mod trade_precreate;
pub use trade_precreate::*;
mod trade_query;
pub use trade_query::*;
mod trade_refund;
pub use trade_refund::*;
mod trade_wap_pay;
pub use trade_wap_pay::*;
mod trade_order_settle;
pub use trade_order_settle::*;
mod trade_order_settle_query;
pub use trade_order_settle_query::*;
mod trade_royalty_relation_bind;
pub use trade_royalty_relation_bind::*;

mod trade_royalty_relation_unbind;
pub use trade_royalty_relation_unbind::*;


