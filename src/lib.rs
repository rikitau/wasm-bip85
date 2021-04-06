use wasm_bindgen::prelude::*;

use std::str::FromStr;

use bitcoin::secp256k1::Secp256k1;
use bitcoin::util::bip32::ExtendedPrivKey;

// https://github.com/rustwasm/wasm-bindgen/issues/1742#issuecomment-643793491
macro_rules! jserr {
    ($expression:expr) => {
        match $expression {
            Ok(a) => a,
            Err(e) => {
                return Err(JsValue::from(format!("{}", e)));
            }
        }
    };
}

#[wasm_bindgen]
pub fn to_wif(xprv: &str,
              idx: u32
              ) -> Result<JsValue, JsValue>{
    let secp = Secp256k1::new();
    let root = jserr!(ExtendedPrivKey::from_str(xprv));
    let wif = jserr!(bip85::to_wif(&secp, &root, idx));
    Ok(wif.to_string().into())
}

#[wasm_bindgen]
pub fn to_mnemonic(xprv: &str,
                   words_number: u32,
                   idx: u32
                ) -> Result<JsValue, JsValue>{
    let secp = Secp256k1::new();
    let root = jserr!(ExtendedPrivKey::from_str(xprv));
    let mn = jserr!(bip85::to_mnemonic(&secp, &root, words_number, idx));
    Ok(mn.to_string().into())
}

