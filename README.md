# [alipay_sdk_rust](https://github.com/wandercn/alipay_sdk_rust)

[![crates.io](https://img.shields.io/crates/v/alipay_sdk_rust.svg?color=yellow)](https://crates.io/crates/alipay_sdk_rust)
[![Released API docs](https://docs.rs/alipay_sdk_rust/badge.svg)](https://docs.rs/alipay_sdk_rust)
[![GPL3 licensed](https://img.shields.io/github/license/wandercn/alipay_sdk_rust.svg)](./LICENSE)
[![Downloads of Crates.io](https://img.shields.io/crates/d/alipay_sdk_rust.svg)](https://crates.io/crates/alipay_sdk_rust)
[![Lines of code](https://img.shields.io/tokei/lines/github/wandercn/alipay_sdk_rust.svg)](#)
[![Build](https://img.shields.io/github/actions/workflow/status/wandercn/alipay_sdk_rust/.github/workflows/rust.yml?branch=master)](#)
[![Languages](https://img.shields.io/github/languages/top/wandercn/alipay_sdk_rust.svg)](#)





 Alipay sdk in rust 支付宝支付 SDK。只支持最安全的RSA2，公钥证书方式签名验证方式,默认只支持utf-8编码和JSON格式。目前只支持商户直接接入模式

 RSA2密钥生成请参考<https://opendocs.alipay.com/common/02kipl> 中的公钥证书方式生成，使用CSR文件申请，密钥格式必须使用PKCS1(非java适用)


 ## 升级修改

 - [x] BizContenter trait 对应的set 方法设支持vlaue值可以是字符串，整型和浮点型，数组和对象同时存储在一个HashMap中。版本：v1.0.5
 - [x] 适配最新的沙箱环境测试 版本：v1.0.5
 - [x] async_verify_sign 增加了异步通知验签函数 版本：v1.0.8
 - [x] 增加了自定义错误进行错误处理 版本：v1.0.8 
 
 # Example
 ## apidoc
 <https://opendocs.alipay.com/apis/api_1/alipay.trade.create>

 ## alipay.trade.create(统一收单交易创建接口)

 注意：开发环境使用沙箱环境下的CSR公钥证书配置调试代码,生产环境需要切换正式申请的公钥证书配置。沙箱环境配置参考<https://opendocs.alipay.com/common/02kkv7>
 
 (如下的Example使用的是沙箱环境下本人申请的自定义公钥证书) (已经适配新版沙箱环境,新版沙箱网关地址是https://openapi-sandbox.dl.alipaydev.com/gateway.do)

 ```rust
use alipay_sdk_rust::biz::{self, BizContenter};
use alipay_sdk_rust::pay::{PayClient, Payer};
use alipay_sdk_rust::response::TradeCreateResponse;
use std::io::Result;
fn main() -> Result<()> {
    let out_trade_no = gostd_time::Now().UnixNano().to_string();
    let mut biz_content = biz::TradeCreateBiz::new();
    biz_content.set_subject("huawei Mate50".into());
    biz_content.set_out_trade_no(out_trade_no.into()); // "1620630871769533112"
    biz_content.set_total_amount("5".into());
    biz_content.set("seller_id", "2088721038897364".into());
    biz_content.set_buyer_id("2088722038897372".into());
    biz_content.set("Timestamp", "2024-07-08 16:09:04".into());
    let client = new_pay_client()?;
    let res = client.trade_create(&biz_content)?;
    println!("{}", serde_json::to_string(&res)?);
    Ok(())
}

fn new_pay_client() -> Result<impl Payer> {
    let client = PayClient::builder()
 .api_url("https://openapi-sandbox.dl.alipaydev.com/gateway.do")
 .app_id("9021000138690920")
 .alipay_root_cert_sn("687b59193f3f462dd5336e5abf83c5d8_02941eef3187dddf3d3b83462e1dfcf6")
 .alipay_public_key("MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAjRE7n0tuGp4e/3RwUnExLxU0JUDsbOOpFgGO/UJT2jidePGRN/GJlqMREfkyYh+JwAhs2ehEUG1j5wnKBIeqCHN8SixwQamyqkMypSJhCUIxfPGOkdD2Vj7n+/iVCFJvmUWBUN5OqD57x410mf1HXlcKegmyFVB1BzHkDLRm3y4+iBMYNsPkZNNZisbv/5etDodCLVQRS/2EiBSHLEjD85Vtm6gCkYieKRdIshyUMwiNr1dIJIJEhdrNt0hsEDkXAbeIRUN9j7OijoZ6Wr076i3+NBuci5wOC7bzLPbU56v2TmJ1yHPqFjAkHKMEesWPHDv7tKANUdsSV03fx1XI6QIDAQAB")
 .app_cert_sn("2ae63de7d1a864071bd72875b6aea838")
 .charset_utf8()
 .format_json()
 .private_key("MIIEowIBAAKCAQEAuYr//cNhdjma7rhDg+dWpSi2H7TvZFZA6PrY9IkTHkJUFU4GXEYeek3AfVPdTJU3/RnJQ4OgnD2mCqFGSgUogePNSBnpmqO8qrSNf0b0LCmt9OT+uuURbuxZjK4XmlEsSqfcDz0ZudApc93PdODpBr5Fw1QFQVz4GPlWieTlVf97GLJBHjTa5i41GYGKe9Wk9AsFOAetTaKdKc5Kq/UVlvOZ7Izl+AzjmmDuVqqn2mUhJPj0Upv9Uw/h9DwLB/I7ebaBl5t+ePnvkaezGr/GqRCMyPmBrRWCW7ztZy9xQi19a8aPbuRjwnUgwfHcxQSczYdSmRvU3L5pSvoMJXBf5QIDAQABAoIBAE9IWgKxTiAalV2UVyUx4RBJySjbQ7/4K1xic/pp6EVxykQgRw9+1t8kAjCLeXCUAoygTCQRRrv8i8rXBxXyBlEaakl46R0vAhtfc2cDhqqoVLDkeQSbhUuEqu82JNfH/8kW2S2Zyjhf/paIHXGv+WLsQxsn9+bqNwlU4emiaWzJNeo8VDWH/7muOOZDEkj/1z/BPzf74RrgkNQFJM6n3EqOjQNdV2ogmGiArBGfGCBfdX0m7X1m7m0RdUHd0PhohUMAp7N6MCtyioFT6vPjrNZHTFCULAI6AqtToJ2zyrBOsyr2kLy3EQXPDMd3Zk+p0NEjmOLQKzD0rGkOjrpzc7ECgYEA/pEdQrS8HvNKQoUK22CEw633GY8Ef7iLio7aSY87FEVSYITpJsZsPQ8Bm3pLGuj3LiF4usmzEeB7ZheJATIVivW2W5AHTFPaHrRz6pQRTVKQ+asczGVrpGYF1XZwtcvWeuSmwOLHTA5cTlasV5e4lP0Ee/A0JwzoqYZPVFd5Gr8CgYEAupZoSZyW2m39tAK/X2RIW5dlglaZtg8PSWmk1LkBItP2WrL7Jre3Oa1CZQNlpCGEkRVzeb32q5EBEG/QTVD3g4smaJ2OYpSNpJVmkjof2/BnfYswzhYiaukK1KV88jNylek3LOULYPAHPxt7gyNG1W+R/rbdF86Xyx/fVrZJolsCgYA7srpC73GCbJmImfyez7ay0PQ2uTRb42JN/wwv3fgD2/Hikqi+oB+3/fHpjTdZuzJkThMq8qwc0CEAIxrM/frRWKgINvGPM8beFoFsjIa2NFutQTSCtnIU3pwVoC30ZDYLId39M/F6449AC9FkxT7TKbbIfd8tHTjesJGf/Nd4rwKBgQCz0fB/wOHBcO9oZJ16mXMCkk3J5xwMUUcyQRYRzJpDrnNlxcg18Cgs9cVfBH8YRxUznbDES0dG4WBdLuVZHgbsR1aky/NFHlXDoBhSJKI/nNigLVzDwKE6140QksxvvxVXVINQSQ1GRw00zI+kOoMxf2z8Rc+d+gRgRj8/mj4ssQKBgBgSkawYSapzlqtRn397p3QiOMPA0OKaoL5ovVxLKKsr/L8b1SrPntk0CUHYVFjE6P5l4RuFURoGnny5ThXWvYQ0A1xkKGMmVTbQJld+K2rqNyghtAX5CtA0laTngJHK4CFoHNNwQwExbQrWzaCBDEjVZOjTrD2/fgoN1wlOtJL7")
 .public_key("MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAuYr//cNhdjma7rhDg+dWpSi2H7TvZFZA6PrY9IkTHkJUFU4GXEYeek3AfVPdTJU3/RnJQ4OgnD2mCqFGSgUogePNSBnpmqO8qrSNf0b0LCmt9OT+uuURbuxZjK4XmlEsSqfcDz0ZudApc93PdODpBr5Fw1QFQVz4GPlWieTlVf97GLJBHjTa5i41GYGKe9Wk9AsFOAetTaKdKc5Kq/UVlvOZ7Izl+AzjmmDuVqqn2mUhJPj0Upv9Uw/h9DwLB/I7ebaBl5t+ePnvkaezGr/GqRCMyPmBrRWCW7ztZy9xQi19a8aPbuRjwnUgwfHcxQSczYdSmRvU3L5pSvoMJXBf5QIDAQAB")
 .sign_type_rsa2()
 .version_1_0()
 .build()?;
    Ok(client)
}
 ```

 # output

 ```
{"response":{"code":"10000","msg":"Success","trade_no":"2024070822001497370503225329","out_trade_no":"1720422500728049000"},"sign":"d37IteN0GyXM68+bD0jyV+RwD4M+f+PqT/Ax+Dv5+HnTXqLVSWAB1gbbiEG1lluB66RoTdw5Uw2yjNf+6v4UZ42PlzL1dAeOz9/hkWuulZD+qvrKPPtSKle5WSxhv0IpKA6d0D2KWGAtRa0Egq1t1t7m6tKSQNwlDIMJMwKAQNR+RZPisD8r6utSKTZt+5DFY+eDlBFMgyeCVFX26JLyVLuuAOnLnLMuLAS5a4G+nYKY80jBgaX/qZApq8VCJT1XH48XEU9GElXaB3oc1OleTZJnBeB6RSnG2HMpashOpvmWv09Sg8P+BcS1i/Q2QJoIAVbWy57FJpGJ3cHxdMOfNQ==","alipay_cert_sn":"71d5f5ab5fa06fcad86b56c0c5888217"}
 ```

# 怎么获取证书序列号,提取证书公钥 ? (已经适配新版沙箱环境)

```rust
use simplelog::{
    ColorChoice, CombinedLogger, ConfigBuilder, LevelFilter, TermLogger, TerminalMode,
    ThreadLogMode,
};

fn init_log() {
    let config = ConfigBuilder::new()
        .set_time_format_rfc3339()
        .set_thread_mode(ThreadLogMode::Names)
        .build();
    CombinedLogger::init(vec![TermLogger::new(
        LevelFilter::Debug,
        config,
        TerminalMode::Mixed,
        ColorChoice::Always,
    )])
    .expect("init log faield!");
}

use alipay_sdk_rust::cert;

const APP_CERT_SN_FILE: &str = "/Users/xxx/Documents/appCertPublicKey_2021000117650139.crt";
const ALIPAY_ROOT_CERT_FILE: &str = "/Users/xxxx/Documents/alipayRootCert.crt";
const ALIPAY_CERT_PUBLIC_KEY_RSA2_FILE: &str = "/Users/xxx/Documents/alipayCertPublicKey_RSA2.crt";
fn main() {
    init_log();
    match cert::get_cert_sn(APP_CERT_SN_FILE) {
        Ok(sn) => {
            println!("app_cert_sn: {}", sn)
        }
        Err(err) => {
            println!("get app_cert_sn faild: {}", err)
        }
    }

    match cert::get_root_cert_sn(ALIPAY_ROOT_CERT_FILE) {
        Ok(sn) => {
            println!("alipay_root_cert_sn : {}", sn)
        }
        Err(err) => {
            println!("get alipay_root_cert_sn faild: {}", err)
        }
    }
    match cert::get_public_key_with_path(ALIPAY_CERT_PUBLIC_KEY_RSA2_FILE) {
        Ok(sn) => {
            println!("alipay_cert_public_key : {}", sn)
        }
        Err(err) => {
            println!("faild: {}", err)
        }
    }
}
```

## output
```
2024-07-08T07:02:58.722034Z [DEBUG] (main) alipay_sdk_rust::cert: cert_path: /Users/lsmiao/Documents/支付宝新沙箱环境/appPublicCert.crt
app_cert_sn: 2ae63de7d1a864071bd72875b6aea838
2024-07-08T07:02:58.783303Z [DEBUG] (main) alipay_sdk_rust::cert: root_cert_path: "/Users/lsmiao/Documents/支付宝新沙箱环境/alipayRootCert.crt"
alipay_root_cert_sn : 687b59193f3f462dd5336e5abf83c5d8_02941eef3187dddf3d3b83462e1dfcf6
2024-07-08T07:02:58.784153Z [DEBUG] (main) alipay_sdk_rust::cert: alipay_cert_path: "/Users/lsmiao/Documents/支付宝新沙箱环境/alipayPublicCert.crt"
alipay_cert_public_key : MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAjRE7n0tuGp4e/3RwUnExLxU0JUDsbOOpFgGO/UJT2jidePGRN/GJlqMREfkyYh+JwAhs2ehEUG1j5wnKBIeqCHN8SixwQamyqkMypSJhCUIxfPGOkdD2Vj7n+/iVCFJvmUWBUN5OqD57x410mf1HXlcKegmyFVB1BzHkDLRm3y4+iBMYNsPkZNNZisbv/5etDodCLVQRS/2EiBSHLEjD85Vtm6gCkYieKRdIshyUMwiNr1dIJIJEhdrNt0hsEDkXAbeIRUN9j7OijoZ6Wr076i3+NBuci5wOC7bzLPbU56v2TmJ1yHPqFjAkHKMEesWPHDv7tKANUdsSV03fx1XI6QIDAQAB
```


## 捐赠方式

If you like my open source project and would like to support me, you can donate through the following methods:
- **Dogecoin address** `DHbDfZWTYjpiGctAyYy1F9YzViTfVRW6aY`
- **AliPay:** `limiao2008@gmail.com`
- **ETH address:** `0x74682cbE11De154E38D8B220ba177c28481F41a8`
- **PayPal:** `paypal.me/wandercn`

Thank you for your support!
