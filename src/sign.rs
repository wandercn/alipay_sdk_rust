#![allow(unused)]
use sha2::{Sha256, Digest};
use base64;
use gostd::{
    builtin::*,
    bytes,
    io::{ByteWriter, StringWriter},
};
use rsa::{
    pkcs1::{DecodeRsaPrivateKey, DecodeRsaPublicKey},
    RsaPrivateKey, RsaPublicKey, Hash, PaddingScheme,PublicKey 
};
use std::{
    borrow::{Borrow, BorrowMut},
    io::{Error, ErrorKind, Result},
};

use crate::{
    biz::{self, BizContenter},
    util::json_get,
};

pub trait Signer {
    fn set_private_key(&mut self, private_key_str: &str) -> Result<()>;
    fn sign(&self, source: &str) -> Result<String>;
    fn verify(&self, source: &str, signature: &str) -> Result<bool>;
    fn set_public_key(&mut self, public_key_str: &str) -> Result<()>;
}

pub struct SignerBuiler {
    rsa2: bool,
}

impl SignerBuiler {
    pub fn set_sign_type(&mut self, sign_type: &str) -> &Self {
        match sign_type {
            "RSA2" => self.sign_type_rsa2(),
            _ => self.sign_type_rsa2(),
        }
    }

    pub fn sign_type_rsa2(&mut self) -> &Self {
        self.rsa2 = true;
        self.borrow_mut()
    }

    pub fn build(&self) -> impl Signer {
        if self.rsa2 {
            return SignSHA256WithRSA::default();
        }
        SignSHA256WithRSA::default()
    }
}

pub fn builder() -> SignerBuiler {
    SignerBuiler { rsa2: false }
}

#[derive(Debug, Default)]
pub struct SignSHA256WithRSA {
    private_key: Option<rsa::RsaPrivateKey>,
    public_key: Option<rsa::RsaPublicKey>,
}

impl Signer for SignSHA256WithRSA {
    // SetPrivateKey 通过RSA文本字符串设置RSA私钥
    fn set_private_key(&mut self, private_key_str: &str) -> Result<()> {
        let private_key = load_private_key(private_key_str)?;
        self.private_key = Some(private_key);
        Ok(())
    }

    fn sign(&self, source: &str) -> Result<String> {
        let digest = Sha256::digest(source.as_bytes());
      if self.private_key.is_none(){
        return Err(Error::new(ErrorKind::Other,"private_key is None")) ;
      }
       if let Ok(signature_byte)= self.private_key.as_ref().unwrap().sign( PaddingScheme::new_pkcs1v15_sign(Some(Hash::SHA2_256)), digest.as_slice()){
        Ok(base64::encode(signature_byte))
       }else{
        Err(Error::new(ErrorKind::Other,"pkcs1v15_sign failed"))
       }
    }

    fn verify(&self, source: &str, signature: &str) -> Result<bool> {
        let mut hashed = Sha256::new();
        hashed.update(source);
        match self.public_key.as_ref().unwrap().verify(
            PaddingScheme::new_pkcs1v15_sign(Some(Hash::SHA2_256)),
             &hashed.finalize(), 
             signature.as_bytes()){
                Ok(())=>Ok(true),
                Err(err)=> Err(err),
             };
       
             Ok(false)
    }

    // SetPublicKey 通过RSA文字字符串设置RSA私钥
    fn set_public_key(&mut self, public_key_str: &str) -> Result<()> {
        let public_key = load_public_key(public_key_str)?;
        self.public_key = Some(public_key);
        Ok(())
    }
}

pub fn load_private_key(private_key_str: &str) -> Result<RsaPrivateKey> {
    if let Ok(private_key) =
        RsaPrivateKey::from_pkcs1_der(&format_pkcs1_private_key(private_key_str))
    {
        Ok(private_key)
    } else {
        Err(Error::new(
            ErrorKind::Other,
            "RsaPrivateKey from_pkcs1_der failed",
        ))
    }
}

pub fn load_public_key(public_key_str: &str) -> Result<RsaPublicKey> {
    if let Ok(public_key) =
        RsaPublicKey::from_pkcs1_der(format_public_key(public_key_str).as_slice())
    {
        Ok(public_key)
    } else {
        Err(Error::new(
            ErrorKind::Other,
            "RsaPublicKey from_pkcs1_der failed",
        ))
    }
}

const kPublicKeyPrefix: &str = "-----BEGIN PUBLIC KEY-----";
const kPublicKeySuffix: &str = "-----END PUBLIC KEY-----";

const kPKCS1Prefix: &str = "-----BEGIN RSA PRIVATE KEY-----";
const KPKCS1Suffix: &str = "-----END RSA PRIVATE KEY-----";

const kPKCS8Prefix: &str = "-----BEGIN PRIVATE KEY-----";
const KPKCS8Suffix: &str = "-----END PRIVATE KEY-----";

const kPublicKeyType: &str = "PUBLIC KEY";
const kPrivateKeyType: &str = "PRIVATE KEY";
const kRSAPrivateKeyType: &str = "RSA PRIVATE KEY";

pub fn format_pkcs1_private_key(raw: &str) -> Vec<byte> {
    raw.as_bytes().to_vec()
    // format_key(raw, kPKCS1Prefix, KPKCS1Suffix, 64)
}

pub fn format_pkcs8_private_key(raw: &str) -> Vec<byte> {
    format_key(raw, kPKCS8Prefix, KPKCS8Suffix, 64)
}

pub fn format_public_key(raw: &str) -> Vec<byte> {
    format_key(raw, kPublicKeyPrefix, kPublicKeySuffix, 64)
}

fn format_key(raw: &str, prefix: &str, suffix: &str, line_count: usize) -> Vec<byte> {
    let mut buffer = bytes::Buffer::new();
    buffer.WriteString(prefix);
    buffer.WriteString("\n");
    let raw_len = line_count;
    let key_len = raw.len();
    let mut raws = key_len / raw_len;
    let temp = key_len % raw_len;
    if temp > 0 {
        raws += 1;
    }
    let mut start = 0;
    let mut end = start + raw_len;
    for i in 0..raws {
        if i == raws - 1 {
            buffer.WriteString(raw.get(start..).unwrap());
        } else {
            buffer.WriteString(raw.get(start..end).unwrap());
        }
        buffer.WriteByte(b'\n');
        start += raw_len;
        end = start + raw_len
    }
    buffer.WriteString(suffix);
    buffer.WriteString("\n");
    return buffer.Bytes();
}
