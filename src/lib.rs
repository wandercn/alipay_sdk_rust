//! Alipay sdk in rust 支付宝支付 SDK。只支持最安全的RSA2，公钥证书方式签名验证方式,默认只支持utf-8编码和JSON格式。目前只支持商户直接接入模式
//!
//
//! RSA2密钥生成请参考<https://opendocs.alipay.com/common/02kipl> 中的公钥证书方式生成，使用CSR文件申请，密钥格式必须使用PKCS1(非java适用)
//!
//!
//! # 升级修改
//!
//!  - [x] BizContenter trait 对应的set 方法设支持vlaue值可以是字符串，整型和浮点型，数组和对象同时存储在一个HashMap中。版本：v1.0.5
//!  - [x] 适配最新的沙箱环境测试 版本：v1.0.5
//!
//!
//! # Example
//! ## apidoc
//! <https://opendocs.alipay.com/apis/api_1/alipay.trade.create>
//!
//! ## alipay.trade.create(统一收单交易创建接口)
//!
//!  注意：开发环境使用沙箱环境下的CSR公钥证书配置调试代码,生产环境需要切换正式申请的公钥证书配置。沙箱环境配置参考<https://opendocs.alipay.com/common/02kkv7>
//!
//! (如下的Example使用的是沙箱环境下本人申请的自定义公钥证书) (已经适配新版沙箱环境,新版沙箱网关地址是https://openapi-sandbox.dl.alipaydev.com/gateway.do)
//!
//! ```rust
//! use std::io::Result;
//! use alipay_sdk_rust::biz::{self, BizContenter};
//! use alipay_sdk_rust::pay::{PayClient, Payer};
//! use alipay_sdk_rust::response::TradeCreateResponse;
//! fn main() -> Result<()> {
//!     let out_trade_no = gostd::time::Now().UnixNano().to_string();
//!     let mut biz_content = biz::TradeCreateBiz::new();
//!     biz_content.set_subject("huawei Mate50".into());
//!     biz_content.set_out_trade_no(out_trade_no.into()); // "1620630871769533112"
//!     biz_content.set_total_amount("5".into());
//!     biz_content.set("seller_id", "2088721038897364".into());
//!     biz_content.set_buyer_id("2088722038897372".into());
//!     biz_content.set("Timestamp", "2024-07-08 16:09:04".into());/////!
//!     let client = new_pay_client()?;
//!     let res = client.trade_create(&biz_content)?;
//!    println!("{}", serde_json::to_string(&res)?);
//!     Ok(())
//! }
//!
//! fn new_pay_client() -> Result<impl Payer> {
//!    let client = PayClient::builder()
//! .api_url("https://openapi-sandbox.dl.alipaydev.com/gateway.do")
//! .app_id("9021000138690920")
//! .alipay_root_cert_sn("687b59193f3f462dd5336e5abf83c5d8_02941eef3187dddf3d3b83462e1dfcf6")
//! .alipay_public_key("MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAjRE7n0tuGp4e/3RwUnExLxU0JUDsbOOpFgGO/UJT2jidePGRN/GJlqMREfkyYh+JwAhs2ehEUG1j5wnKBIeqCHN8SixwQamyqkMypSJhCUIxfPGOkdD2Vj7n+/iVCFJvmUWBUN5OqD57x410mf1HXlcKegmyFVB1BzHkDLRm3y4+iBMYNsPkZNNZisbv/5etDodCLVQRS/2EiBSHLEjD85Vtm6gCkYieKRdIshyUMwiNr1dIJIJEhdrNt0hsEDkXAbeIRUN9j7OijoZ6Wr076i3+NBuci5wOC7bzLPbU56v2TmJ1yHPqFjAkHKMEesWPHDv7tKANUdsSV03fx1XI6QIDAQAB")
//! .app_cert_sn("2ae63de7d1a864071bd72875b6aea838")
//! .charset_utf8()
//! .format_json()
//! .private_key("MIIEowIBAAKCAQEAuYr//cNhdjma7rhDg+dWpSi2H7TvZFZA6PrY9IkTHkJUFU4GXEYeek3AfVPdTJU3/RnJQ4OgnD2mCqFGSgUogePNSBnpmqO8qrSNf0b0LCmt9OT+uuURbuxZjK4XmlEsSqfcDz0ZudApc93PdODpBr5Fw1QFQVz4GPlWieTlVf97GLJBHjTa5i41GYGKe9Wk9AsFOAetTaKdKc5Kq/UVlvOZ7Izl+AzjmmDuVqqn2mUhJPj0Upv9Uw/h9DwLB/I7ebaBl5t+ePnvkaezGr/GqRCMyPmBrRWCW7ztZy9xQi19a8aPbuRjwnUgwfHcxQSczYdSmRvU3L5pSvoMJXBf5QIDAQABAoIBAE9IWgKxTiAalV2UVyUx4RBJySjbQ7/4K1xic/pp6EVxykQgRw9+1t8kAjCLeXCUAoygTCQRRrv8i8rXBxXyBlEaakl46R0vAhtfc2cDhqqoVLDkeQSbhUuEqu82JNfH/8kW2S2Zyjhf/paIHXGv+WLsQxsn9+bqNwlU4emiaWzJNeo8VDWH/7muOOZDEkj/1z/BPzf74RrgkNQFJM6n3EqOjQNdV2ogmGiArBGfGCBfdX0m7X1m7m0RdUHd0PhohUMAp7N6MCtyioFT6vPjrNZHTFCULAI6AqtToJ2zyrBOsyr2kLy3EQXPDMd3Zk+p0NEjmOLQKzD0rGkOjrpzc7ECgYEA/pEdQrS8HvNKQoUK22CEw633GY8Ef7iLio7aSY87FEVSYITpJsZsPQ8Bm3pLGuj3LiF4usmzEeB7ZheJATIVivW2W5AHTFPaHrRz6pQRTVKQ+asczGVrpGYF1XZwtcvWeuSmwOLHTA5cTlasV5e4lP0Ee/A0JwzoqYZPVFd5Gr8CgYEAupZoSZyW2m39tAK/X2RIW5dlglaZtg8PSWmk1LkBItP2WrL7Jre3Oa1CZQNlpCGEkRVzeb32q5EBEG/QTVD3g4smaJ2OYpSNpJVmkjof2/BnfYswzhYiaukK1KV88jNylek3LOULYPAHPxt7gyNG1W+R/rbdF86Xyx/fVrZJolsCgYA7srpC73GCbJmImfyez7ay0PQ2uTRb42JN/wwv3fgD2/Hikqi+oB+3/fHpjTdZuzJkThMq8qwc0CEAIxrM/frRWKgINvGPM8beFoFsjIa2NFutQTSCtnIU3pwVoC30ZDYLId39M/F6449AC9FkxT7TKbbIfd8tHTjesJGf/Nd4rwKBgQCz0fB/wOHBcO9oZJ16mXMCkk3J5xwMUUcyQRYRzJpDrnNlxcg18Cgs9cVfBH8YRxUznbDES0dG4WBdLuVZHgbsR1aky/NFHlXDoBhSJKI/nNigLVzDwKE6140QksxvvxVXVINQSQ1GRw00zI+kOoMxf2z8Rc+d+gRgRj8/mj4ssQKBgBgSkawYSapzlqtRn397p3QiOMPA0OKaoL5ovVxLKKsr/L8b1SrPntk0CUHYVFjE6P5l4RuFURoGnny5ThXWvYQ0A1xkKGMmVTbQJld+K2rqNyghtAX5CtA0laTngJHK4CFoHNNwQwExbQrWzaCBDEjVZOjTrD2/fgoN1wlOtJL7")
//! .public_key("MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAuYr//cNhdjma7rhDg+dWpSi2H7TvZFZA6PrY9IkTHkJUFU4GXEYeek3AfVPdTJU3/RnJQ4OgnD2mCqFGSgUogePNSBnpmqO8qrSNf0b0LCmt9OT+uuURbuxZjK4XmlEsSqfcDz0ZudApc93PdODpBr5Fw1QFQVz4GPlWieTlVf97GLJBHjTa5i41GYGKe9Wk9AsFOAetTaKdKc5Kq/UVlvOZ7Izl+AzjmmDuVqqn2mUhJPj0Upv9Uw/h9DwLB/I7ebaBl5t+ePnvkaezGr/GqRCMyPmBrRWCW7ztZy9xQi19a8aPbuRjwnUgwfHcxQSczYdSmRvU3L5pSvoMJXBf5QIDAQAB")
//! .sign_type_rsa2()
//! .version_1_0()
//! .build()?;
//!    Ok(client)
//!}
//! ```
//!
//! # output
//!
//! `
//! {"response":{"code":"10000","msg":"Success","trade_no":"2024070822001497370503230532","out_trade_no":"1720424278524805000"},"sign":"YJ38ophTFhnNdEmfFUpWxuZ1Y3b11s8UoJ5Zh8iiIt2/YwqBnHOGsiM2SgY2vkTtrsuZPan1idU88w0knEntX8iMZLXVx1osY39JqfKEvRsOXjTa4JebhtVhkhJZQE57SgOMxKMNoCixbi734G+HgbzidjWGv1IfVBQUdz5qgOi48Fu7Tt0AY3I4lHiXw9eHe8TO9FFiY1p6sIUUMXufaZzvU5px3u6345MBikQF30pjX6dLYSkstsCEINU06oz3dQ/TW3iuwSLV0Do0Mqy7+/Bfde/oEhDnbGIvoUCJT+jfqdEYGUR3Ped6Ydr7sZRV7UsKSZ16YVuIHJ8S28GiEA==","alipay_cert_sn":"71d5f5ab5fa06fcad86b56c0c5888217"}
//! `
pub mod biz;
pub mod cert;
pub mod pay;
pub mod request;
pub mod response;
pub mod sign;
pub mod util;

#[cfg(test)]
mod tests;
