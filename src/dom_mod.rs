//! dom_mod.rs manipulates the html dom

use crate::{web_sys_mod::*, PwaData};

/// The app starts with this function
pub fn start_function() {
    // Initialize debugging for when/if something goes wrong.
    console_error_panic_hook::set_once();
    // write the app version just for debug purposes
    debug_write(&format!(
        "bestia_dev_new_rust_wasm_pwa v{}",
        env!("CARGO_PKG_VERSION")
    ));
    // set the window initial size
    resize_window(800, 600);
    // load from local storage
    let data = load_all_from_local_storage();
    // inject html into DOM
    inject_htm_into_dom(&data);
    // prepare events that read local file, pass the function to execute
    add_listener_on_file_change_to_read_single_file("file_input", &on_file_change);
}

/// load all from local storage
pub fn load_all_from_local_storage() -> PwaData {
    let data = PwaData {
        pwa_short_name: load_string_from_local_storage("pwa_short_name", "pwa_short_name"),
        pwa_name: load_string_from_local_storage("pwa_name", "pwa_name"),
        pwa_description: load_string_from_local_storage("pwa_description", "pwa_description"),
        rust_project_name: load_string_from_local_storage("rust_project_name", "rust_project_name"),
    };
    // return
    data
}

/// inject html into dom
pub fn inject_htm_into_dom(pwa_data: &PwaData) {
    // rust has `Raw string literals` that are great!
    // just add r# before and # after the start and end double quotes.
    let html = format!(
        r##"
        <h2>New Rust Wasm PWA</h2>
		<p>Creates a minimal working Rust Wasm PWA.</p>
		<p>First enter human readable names and description:</p>
		<div class="button-wrap">
            <label for="pwa_short_name">PWA short name:</label>  
            <input style="width:20%;" type="text" id="pwa_short_name" value="{}"/>
        </div>
		<div class="button-wrap">
            <label for="pwa_name">PWA name:</label>  
            <input style="width:40%;"  type="text" id="pwa_name" value="{}"/>
        </div>
		<div class="button-wrap">
            <label for="pwa_description">PWA description:</label>  
            <input style="width:80%;" type="text" id="pwa_description" value="{}"/>
        </div>
        <p>Second enter the Rust project name. It must be all lower-case and underscore:</p>
        <div class="button-wrap">
            <label for="rust_project_name">Rust project name:</label>  
            <input style="width:40%;" type="text" id="rust_project_name" value="{}"/>
        </div>
        <p>Select the png file for the icons (must be 512x512):</p>
        
        <!--tricky div+label+css to change Input file appearance -->
        <div class="button-wrap">
            <label class="button" for="file_input">Select File</label>
            <!--only one single png file. No "multiple". The event listeners are added in Rust code.-->  
            <input type="file" id="file_input" accept="image/png"/>
        </div>
        "##,
        html_encode(&pwa_data.pwa_short_name),
        html_encode(&pwa_data.pwa_name),
        html_encode(&pwa_data.pwa_description),
        html_encode(&pwa_data.rust_project_name)
    );

    set_inner_html("div_for_wasm_html_injecting", &html);
}

/// on file change code that is not boilerplate
pub fn on_file_change(vec: Vec<u8>) {
    // save Input Text elements to local storage
    let pwa_data = read_input_elements_and_save_to_local_storage();
    crate::prepare_zip_mod::create_and_fill_zip(vec, pwa_data);
}

/// read input elements and save to local storage
pub fn read_input_elements_and_save_to_local_storage() -> PwaData {
    let pwa_data = PwaData {
        pwa_short_name: get_input_element_value_string_by_id("pwa_short_name"),
        pwa_name: get_input_element_value_string_by_id("pwa_name"),
        rust_project_name: get_input_element_value_string_by_id("rust_project_name"),
        pwa_description: get_input_element_value_string_by_id("pwa_description"),
    };
    save_to_local_storage("pwa_short_name", &pwa_data.pwa_short_name);
    save_to_local_storage("pwa_name", &pwa_data.pwa_name);
    save_to_local_storage("rust_project_name", &pwa_data.rust_project_name);
    save_to_local_storage("pwa_description", &pwa_data.pwa_description);
    // return
    pwa_data
}
