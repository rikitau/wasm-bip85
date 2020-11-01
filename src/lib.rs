use wasm_bindgen::prelude::*;

use std::str::FromStr;

use bitcoin::secp256k1::Secp256k1;
use bitcoin::util::bip32::ExtendedPrivKey;

#[wasm_bindgen]
pub fn to_wif(xprv: &str,
              idx: u32
              ) -> Result<JsValue, JsValue>{
    let secp = Secp256k1::new();
    let root = ExtendedPrivKey::from_str(xprv).expect("invalid xprv");
    let wif = bip85::to_wif(&secp, &root, idx).expect("cant convert");
    Ok(wif.to_string().into())
}

#[wasm_bindgen]
pub fn to_mnemonic(xprv: &str,
                   words_number: u32,
                   idx: u32
                ) -> Result<JsValue, JsValue>{
    let secp = Secp256k1::new();
    let root = ExtendedPrivKey::from_str(xprv).expect("invalid xprv");
    let mn = bip85::to_mnemonic(&secp, &root, words_number, idx).expect("cant convert, invalid words number or index?");
    Ok(mn.to_string().into())
}
