use gostd::strings;

use crate::sign::{builder, Signer};
const PRIVATE_KEY_FILE: &str = "src/tests/ffactory.org_私钥.txt"; // 沙箱环境自定义测试密钥
const PUBILC_KEY_FILE: &str = "src/tests/ffactory.org_公钥.txt"; // 沙箱环境自定义测试密钥
const SOURCE_URL: &str = r#"alipay_root_cert_sn=687b59193f3f462dd5336e5abf83c5d8_02941eef3187dddf3d3b83462e1dfcf6&app_cert_sn=1aca24501cc5cd3ac19089ff49d99d63&app_id=2021000117650139&biz_content={"buyer_id":"2088102175953034","out_trade_no":"1620630871769533001","subject":"iPhone","timestamp":"2021-05-08 17:09:04","total_amount":"88.88"}&charset=utf-8&format=JSON&method=alipay.trade.create&sign_type=RSA2&timestamp=2021-05-12 17:18:09&version=1.0"#;
#[test]
fn test_sign() -> std::io::Result<()> {
    let project_dir = env!("CARGO_MANIFEST_DIR");
    let private_key_str: String =
        std::fs::read_to_string(format!("{}/{}", project_dir, PRIVATE_KEY_FILE))?;
    let mut sign = builder().sign_type_rsa2().build();
    sign.set_private_key(&private_key_str)?;
    let public_key_str = std::fs::read_to_string(format!("{}/{}", project_dir, PUBILC_KEY_FILE))?;
    sign.set_public_key(&public_key_str)?;

    let mut source_split = strings::Split(SOURCE_URL, "&");
    source_split.sort();

    let sorted_source = strings::Join(source_split, "&");

    let signature = sign.sign(&sorted_source)?;
    let is_passed = sign.verify(&sorted_source, &signature)?;
    assert_eq!(true, is_passed);
    Ok(())
}
