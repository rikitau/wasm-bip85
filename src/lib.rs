use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

use std::str::FromStr;

use bitcoin::secp256k1::Secp256k1;
use bitcoin::util::bip32::ExtendedPrivKey;

// Called when the wasm module is instantiated
#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    // Use `web_sys`'s global `window` function to get a handle on the global
    // window object.
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");

    let root = ExtendedPrivKey::from_str(
        "xprv9s21ZrQH143K2LBWUUQRFXhucrQqBpKdRRxNVq2zBqsx8HVqFk2uYo8kmbaL\
         LHRdqtQpUm98uKfu3vca1LqdGhUtyoFnCNkfmXRyPXLjbKb"
    ).unwrap();
    let secp = Secp256k1::new();

    let mut idx = 0;
    let derived = bip85::to_mnemonic(&secp, &root, 12, idx).unwrap();

    //val.set_inner_html(&format!("WIF key: {}", derived));
    //body.append_child(&val)?;
    let wif_el = document.get_element_by_id("wif").expect("can't find wif element");
    wif_el.set_inner_html(&derived.to_string());

    let a = Closure::wrap(Box::new(move || {
        idx += 1;
        let mn = bip85::to_mnemonic(&secp, &root, 12, idx).unwrap();
        wif_el.set_inner_html(&mn.to_string());
    }) as Box<dyn FnMut()>);
    document
        .get_element_by_id("index")
        .expect("should have #index on the page")
        .dyn_ref::<HtmlElement>()
        .expect("#index should be an `HtmlElement`")
        .set_onchange(Some(a.as_ref().unchecked_ref()));

    // See comments in `setup_clock` above for why we use `a.forget()`.
    a.forget();

    Ok(())
}

#[wasm_bindgen]
pub fn add(a: u32, b: u32) -> u32 {
    a + b
}

