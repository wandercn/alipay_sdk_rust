//! Alipay sdk in rust 支付宝支付 SDK。只支持最安全的RSA2，公钥证书方式签名验证方式,默认只支持utf-8编码和JSON格式。目前只支持商户直接接入模式
//!
//! RSA2密钥生成请参考<https://opendocs.alipay.com/common/02kipl> 中的公钥证书方式生成，使用CSR文件申请，密钥格式必须使用PKCS1(非java适用)
//!
//! # Example
//! ## apidoc
//! <https://opendocs.alipay.com/apis/api_1/alipay.trade.create>
//!
//! ## alipay.trade.create(统一收单交易创建接口)
//!
//!  注意：开发环境使用沙箱环境下的CSR公钥证书配置调试代码,生产环境需要切换正式申请的公钥证书配置。沙箱环境配置参考<https://opendocs.alipay.com/common/02kkv7>
//!
//! (如下的Example使用的是沙箱环境下本人申请的自定义公钥证书)
//!
//! ```rust
//! use std::io::Result;
//! use alipay_sdk_rust::biz::{self, BizContenter};
//! use alipay_sdk_rust::pay::{PayClient, Payer};
//! use alipay_sdk_rust::response::TradeCreateResponse;
//! use alipay_sdk_rust::util;
//! fn main() -> Result<()> {
//!     let out_trade_no = util::get_out_trade_no();
//!     let mut biz_content = biz::TradeCreateBiz::new();
//!     biz_content.set_subject("huawei Mate50");
//!     biz_content.set_out_trade_no(&out_trade_no); // "1620630871769533112"
//!     biz_content.set_total_amount("5");
//!     biz_content.set("seller_id", "2088621955702975");
//!     biz_content.set_buyer_id("2088102175953034");
//!     biz_content.set("Timestamp", "2022-07-11 16:09:04");/////!
//!     let client = new_pay_client()?;
//!     let res = client.trade_create(&biz_content)?;
//!    println!("{}", serde_json::to_string(&res)?);
//!     Ok(())
//! }
//!
//! fn new_pay_client() -> Result<impl Payer> {
//!     let client = PayClient::builder()
//! .api_url("https://openapi.alipaydev.com/gateway.do")
//! .app_id("2021000117650139")
//! .alipay_root_cert_sn("687b59193f3f462dd5336e5abf83c5d8_02941eef3187dddf3d3b83462e1dfcf6")
//! .alipay_public_key("MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAoEz4Tegg1Ytfn2N+jWzDTOBbUNhcpIXz7dV3K8yIsDzinPwYTzzZhlCLYlKbfJGpSDVU0K3I6TnyGfNPEg+R7EC/3ycY1nETh8mxQ4dgrz8irKlLxSRDHVtPg4luncDz2u0hMab9QJdqF8DXD5H+r0Pdt6CSJgKJqLa0awPV3/8cTQZbhZ4ontnRdcwWWcp4/TunkEc891Aa5FmzWp4hgBYcu3BGcazS1HQA4r6wTwRkiqKsCwZeVTag4CiOeqe/vRFTxTMKF4gjRzdhTapUfcBCmXblEA3i8/7OILEyNHceRAxxxIpUjTyoRJ4/2xd0kWbw1gmkLFM0Fzee0eVgoQIDAQAB")
//! .app_cert_sn("8c68b9753e5b9e0bb7704a981936ecce")
//! .charset_utf8()
//! .format_json()
//! .private_key("MIIEogIBAAKCAQEAog0N+rHllTO+e42Bc5mpvowolWVStyurL3Ou/86uRMN8im7WG1v44h09IaZpw4k6dpYEj89d7aLd7IwnBR5Wg84Ox2LMR/Y/Pzo10hjlvJJOk+igqepSTtB/4UX0cG/9tWceHAWOFuD8uw/SSJegC91a0MmLUBUpd4wWnN1iOSi0442iNvNk79Z6xLKIs4LNJGCNddxcofvpdqq4/5ywxHo24m5zPQf6/ttGk5jQQVrF+y2ckdHKd2h7ZSOYI7nzlzbZqK0UOMDuTvRs696fPa5wSEshE0RQBcn5iCltNTPyLsL3RGUUlsLOPsyT6cFZtUAKJ+J6wEqQxM5TrvNHywIDAQABAoIBAGDTuhGccFipVVzP3ZSsMV+4sZsqsrTd8+hjkCIrZbeSsvyoY2hvmRPKcreDjtiWS4eF9e3T8wTF9yKbT8lgKkORQQVkBDnPalUmO/hwhf0Z0rfQHQfKCiorrO129iqk0AyvM698pj0HbBt9xaE4cBoGxnfQpVxReLiEzRInucP6lhE79v1BwXCwMtRVvFPPIFaLJ02JGIywN+jnkpLwJj8TAu5u3JawlRnsFJgQeTdsHs4G6E11WBeo7OZtKPiKWMcj1nPU0Dnr+6VG89Rx/cxqlMrlTJBhKsLEzcVQwcc3M3UnMOU3Of7Mj1olnUGJ90apVukDFM5OI4Mfqi9etekCgYEA9ViJqVUdzJwqTK/gmbAsRvri9+rmWhfoqBMGeUoHktOGR/hMKJ/LrVa1oBIcVVNdLbI7Ks0kySGCa6qm/4YP2DCihj0GKwnHdPQ94pd3lWvUFZimNKi5V/+sREU0dKSqK3b7F0njtpR6zn+x8KpktO7izL3o8740KpSb7xGb0BUCgYEAqRaLShbDYjhSfvIzWAbuPNpvvtDWUNip1cuJwzTvDthECV5ltkhLGVWVnStch6OeTbK+llLDVw+j/YT1KetQcZ60tw2spn8nq6UvC2IFa2h61zpa8VWeRDfhyEIzoBE8DFAyeWjqYHyHJlh0BzRA2P3ts2LwwwhRa6OHhYzQ0F8CgYAUUvpMab2nNoSWh7dOY/a3Bo+IxA/DBNoEGldd8tD/y8AC9EGy19HykQ1Irldkhhxg7bPTDt1uP/Vi3+cnob5sRVMhVarOI+g++wCpZazFVwJhq5yRHi0EaiymFymKRB3IrfmM61UOyewGcTOXYTYoeuWU2mKS1n3RzS/BtS64JQKBgB2husVAGftzfVmL3l2V0VhOu3iIJpbCcXjrE3hnJWHHmpy9sztvjeGhsvd5Kt0GWm6pXWcAmAUA069RBpnTCCTxOCBAQDppXC1jZEwtYF/DTou7SUazx2mTFXk/yMZLXueVglLuhOxlxlV8+NBuYtLkJSzjsOes5H/lh5Fq7QknAoGAJ/LzTBLPy3terUgqejSxB4pr6PMbNd8wEStHN1RmR2v9Msuto7PUT7OOjQYIJwQLnxQUDr65bB5uR35v+L/rC6XUkzJM18YWvmOhFM8OsIYc4HdDhSmeFpMXdbd6entMJEX0bWrTbS/UdEcqE30kwuNuEFQ07LopGY1gBEe1G8U=")
//! .public_key("MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAog0N+rHllTO+e42Bc5mpvowolWVStyurL3Ou/86uRMN8im7WG1v44h09IaZpw4k6dpYEj89d7aLd7IwnBR5Wg84Ox2LMR/Y/Pzo10hjlvJJOk+igqepSTtB/4UX0cG/9tWceHAWOFuD8uw/SSJegC91a0MmLUBUpd4wWnN1iOSi0442iNvNk79Z6xLKIs4LNJGCNddxcofvpdqq4/5ywxHo24m5zPQf6/ttGk5jQQVrF+y2ckdHKd2h7ZSOYI7nzlzbZqK0UOMDuTvRs696fPa5wSEshE0RQBcn5iCltNTPyLsL3RGUUlsLOPsyT6cFZtUAKJ+J6wEqQxM5TrvNHywIDAQAB")
//! .sign_type_rsa2()
//! .version_1_0()
//! .build()?;
//!     Ok(client)
//! }
//! ```
//!
//! # output
//!
//! `
//! {"response":{"code":"10000","msg":"Success","trade_no":"2022071322001453030502038801","out_trade_no":"1620630871769533112"},"sign":"NgDoQ8wIjV0MY3/hA3BDvHOz3Jw7y6CTdGPD+Q4GBsvrAHDDRwbdki+jSGr66zutKtklUYJsizKVIGbmJmKKBhVSNdxWCRI++keWfHjWDLjy59hiRix0l8oFh+dhnXaQqjAXEjqOxjtd6WGgO9FhgX1Kz6GAZ8NJobwzXKor8fotA0E5ztpcPcRRF4KmdVioofAdSf0o9UTpM24uFmGuBwi0Cfae70jctpmn0CMXJ36g2FEe3pcZIWm/KWDAXwGH6daQccULwVjUYN01OyeM93wKuLXJwEhvIeLpJeW4AiXpU21W/qNgYINPkjRA/h/HmG6ooG14VfdHNXPjuQ0/sg==","alipay_cert_sn":"28e0499cc4ef722406edd30274314430"}
//! `
pub mod biz;
pub mod pay;
pub mod request;
pub mod response;
pub mod sign;
pub mod util;

#[cfg(test)]
mod tests;
