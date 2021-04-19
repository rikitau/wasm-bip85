use wasm_bindgen::prelude::*;

use std::str::FromStr;

use bip85::bitcoin::secp256k1::Secp256k1;
use bip85::bitcoin::util::key::PrivateKey;
use bip85::bitcoin::util::bip32::ExtendedPrivKey;
use bip85::bitcoin::network::constants::Network;
use bip85::bip39::Mnemonic;

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
pub fn root_from_mnemonic(mnemonic: &str,
                          password: &str
                          ) -> Result<JsValue, JsValue>{
    let mn = jserr!(Mnemonic::parse(mnemonic));
    // mnemonic to seed with empty password
    let seed = mn.to_seed(password);
    // generate root bip-32 key from seed
    let root = jserr!(ExtendedPrivKey::new_master(Network::Bitcoin, &seed));
    Ok(root.to_string().into())
}

#[wasm_bindgen]
pub fn root_from_wif(wif: &str) -> Result<JsValue, JsValue>{
    let pk = jserr!(PrivateKey::from_wif(wif));
    let seed = pk.to_bytes();
    let root = jserr!(ExtendedPrivKey::new_master(Network::Bitcoin, &seed));
    Ok(root.to_string().into())
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

