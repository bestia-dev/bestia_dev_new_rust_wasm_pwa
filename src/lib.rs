//! bestia_dev_new_rust_wasm_pwa
//! lib.rs is just for the wasm_bindgen_start function and to connect to all the modules.
//! and for the big doc comments

use wasm_bindgen::prelude::*;

mod dom_mod;
mod prepare_zip_mod;
mod web_sys_mod;

pub struct PwaData {
    pub pwa_short_name: String,
    pub pwa_name: String,
    pub pwa_description: String,
    pub rust_project_name: String,
}

#[wasm_bindgen(start)]
/// To start the Wasm application, wasm_bindgen runs this functions
pub fn wasm_bindgen_start() -> Result<(), JsValue> {
    dom_mod::start_function();
    // return
    Ok(())
}
