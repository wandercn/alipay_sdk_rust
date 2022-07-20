//! 证书相关函数，获取证书序列号，提取证书中的公钥
use gostd::strings;
use std::{
    fs,
    io::{Error, ErrorKind, Result},
};
use x509_parser::prelude::*;

use log::debug;

///    get_cert_sn 获取证书序列号SN
///    cert_path：X.509证书文件路径(appCertPublicKey.crt、alipayCertPublicKey_RSA2.crt)
///    返回 sn：证书序列号(app_cert_sn、alipay_cert_sn)
pub fn get_cert_sn(cert_path: impl AsRef<str>) -> Result<String> {
    debug!("cert_path: {}", cert_path.as_ref());
    let cert_data: &[u8] = &fs::read(cert_path.as_ref())?;
    cert_sn_from_utf8(cert_data)
}
///  cert_sn_from_utf8 从文本计算证书序列号(app_cert_sn、alipay_cert_sn)
pub fn cert_sn_from_utf8(cert_content: impl AsRef<[u8]>) -> Result<String> {
    let cert_data = cert_content.as_ref();
    if let Err(err) = parse_x509_pem(cert_data) {
        return Err(Error::new(
            ErrorKind::Other,
            format!("parse_x509_pem: {}", err.to_string()),
        ));
    }

    let (_, cert) = parse_x509_pem(cert_data).ok().expect("Pem is None");
    if let Ok(x509) = cert.parse_x509() {
        let mut name = x509.tbs_certificate.issuer().to_string();
        //提取出的证书的issuer本身是以CN开头的，则无需逆序，直接返回
        if !strings::HasPrefix(&name, "CN") {
            let mut attributes = strings::Split(&name, ", ");
            attributes.reverse();
            name = strings::Join(attributes, ",");
        }
        let serial_number = x509.serial.to_str_radix(10);
        Ok(format!("{:x}", md5::compute(name + &serial_number)))
    } else {
        Err(Error::new(ErrorKind::Other, "parse_x509 failed"))
    }
}

/// get_root_cert_sn 获取root证书序列号SN
///    root_cert_path：X.509证书文件路径(alipayRootCert.crt)
///    返回 sn：证书序列号(alipay_root_cert_sn)
pub fn get_root_cert_sn(root_cert_path: impl AsRef<str>) -> Result<String> {
    debug!("root_cert_path: {:?}", root_cert_path.as_ref());
    let certs_data = &fs::read(root_cert_path.as_ref())?;
    root_cert_sn_from_utf8(certs_data)
}

/// root_cert_sn_from_utf8 从文本计算根证书序列号(alipay_root_cert_sn)
pub fn root_cert_sn_from_utf8(cert_contents: impl AsRef<[u8]>) -> Result<String> {
    let cert_end = "-----END CERTIFICATE-----";
    let certs_str = String::from_utf8(cert_contents.as_ref().to_vec())
        .or(Err(Error::new(ErrorKind::Other, "form_utf8 failed")))?;
    let pems = strings::Split(&certs_str, &cert_end);
    let mut sn = String::new();
    for c in pems {
        let cert_data = c.to_owned() + cert_end;
        if let Err(_) = parse_x509_pem(cert_data.as_bytes()) {
            continue;
        }

        let (_, cert) = parse_x509_pem(cert_data.as_bytes())
            .ok()
            .expect("Pem is None");
        if let Ok(x509) = cert.parse_x509() {
            if !x509
                .signature_algorithm
                .algorithm
                .to_id_string()
                .starts_with("1.2.840.113549.1.1")
            {
                continue;
            }

            if sn.is_empty() {
                sn += &cert_sn_from_utf8(cert_data.as_bytes())?;
            } else {
                sn += &("_".to_owned() + &cert_sn_from_utf8(cert_data.as_bytes())?);
            }
        }
    }

    if sn.is_empty() {
        return Err(Error::new(
            ErrorKind::Other,
            "failed to get sn,please check your cert",
        ));
    }
    Ok(sn)
}

/// 从支付宝公钥证书文件中提取支付宝公钥(alipayCertPublicKey_RSA2.crt)
pub fn get_public_key_with_path<'a>(alipay_cert_path: impl AsRef<str>) -> Result<String> {
    debug!("alipay_cert_path: {:?}", alipay_cert_path.as_ref());
    let cert_data = &fs::read(alipay_cert_path.as_ref())?;
    let cert = load_certificate(cert_data)?;
    match cert.parse_x509() {
        Ok(certificate) => Ok(base64::encode(certificate.public_key().raw)),
        Err(err) => Err(Error::new(ErrorKind::Other, err.to_string())),
    }
}

/// 通过证书的文本内容加载证书
pub fn load_certificate(cert_content: impl AsRef<[u8]>) -> Result<Pem> {
    let cert_data = cert_content.as_ref();
    if let Err(err) = parse_x509_pem(cert_data) {
        return Err(Error::new(
            ErrorKind::Other,
            format!("parse_x509_pem: {}", err.to_string()),
        ));
    }

    let (_, cert) = parse_x509_pem(cert_data).ok().expect("Pem is None");
    if cert.label != "CERTIFICATE" {
        return Err(Error::new(ErrorKind::Other, "Failed to decode certificate"));
    }
    Ok(cert)
}
