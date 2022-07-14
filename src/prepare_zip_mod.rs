//! prepare_zip_mod.rs should not use any javascript objects. Pure Rust.
//! All the javascript objects, functions and conversion should be in web_sys_mod.rs.

use crate::{web_sys_mod::*, PwaData};
use std::io::Write;
use unwrap::unwrap;

/// create and fill zip
pub fn create_and_fill_zip(vec: Vec<u8>, pwa_data: PwaData) {
    // get date time now
    let now = date_time_now();

    let img = decode_png(vec);
    let mut buf = &mut vec![0u8; 2_097_152];
    let mut zip = create_new_zip(&mut buf);

    // favicon.ico with 16, 32 and 48 icons
    encode_to_favicon_ico_and_add_to_zip(&mut zip, &img, &now, &pwa_data.rust_project_name);

    // region: png with various sizes for: favicon png, pwa Android and pwa iOS
    // 32, 72, 96, 120, 128, 144, 152, 167, 180, 192, 196, 512
    resize_img_and_add_to_zip(
        &mut zip,
        &img,
        32,
        "icon-032.png",
        &now,
        &pwa_data.rust_project_name,
    );
    resize_img_and_add_to_zip(
        &mut zip,
        &img,
        72,
        "icon-072.png",
        &now,
        &pwa_data.rust_project_name,
    );
    resize_img_and_add_to_zip(
        &mut zip,
        &img,
        96,
        "icon-096.png",
        &now,
        &pwa_data.rust_project_name,
    );
    resize_img_and_add_to_zip(
        &mut zip,
        &img,
        120,
        "icon-120.png",
        &now,
        &pwa_data.rust_project_name,
    );
    resize_img_and_add_to_zip(
        &mut zip,
        &img,
        128,
        "icon-128.png",
        &now,
        &pwa_data.rust_project_name,
    );
    resize_img_and_add_to_zip(
        &mut zip,
        &img,
        144,
        "icon-144.png",
        &now,
        &pwa_data.rust_project_name,
    );
    resize_img_and_add_to_zip(
        &mut zip,
        &img,
        152,
        "icon-152.png",
        &now,
        &pwa_data.rust_project_name,
    );
    resize_img_and_add_to_zip(
        &mut zip,
        &img,
        167,
        "icon-167.png",
        &now,
        &pwa_data.rust_project_name,
    );
    resize_img_and_add_to_zip(
        &mut zip,
        &img,
        180,
        "icon-180.png",
        &now,
        &pwa_data.rust_project_name,
    );
    resize_img_and_add_to_zip(
        &mut zip,
        &img,
        192,
        "icon-192.png",
        &now,
        &pwa_data.rust_project_name,
    );
    resize_img_and_add_to_zip(
        &mut zip,
        &img,
        196,
        "icon-196.png",
        &now,
        &pwa_data.rust_project_name,
    );
    resize_img_and_add_to_zip(
        &mut zip,
        &img,
        512,
        "icon-512.png",
        &now,
        &pwa_data.rust_project_name,
    );

    // maskable icon 192
    resize_img_and_add_to_zip(
        &mut zip,
        &img,
        192,
        "icon-maskable.png",
        &now,
        &pwa_data.rust_project_name,
    );
    // endregion

    // text files
    add_manifest_json_to_zip(
        &mut zip,
        &pwa_data.pwa_short_name,
        &pwa_data.pwa_name,
        &pwa_data.rust_project_name,
        &now,
    );
    add_index_html_to_zip(
        &mut zip,
        &pwa_data.pwa_name,
        &pwa_data.pwa_description,
        &now,
        &pwa_data.rust_project_name,
    );
    add_service_worker_js_to_zip(&mut zip, &now, &pwa_data.rust_project_name);
    add_start_service_worker_js_to_zip(&mut zip, &now, &pwa_data.rust_project_name);
    add_css_to_zip(&mut zip, &now, &pwa_data.rust_project_name);

    add_rust_yml_to_zip(&mut zip, &now, &pwa_data.rust_project_name);
    add_automation_tasks_rs_main_to_zip(
        &mut zip,
        &now,
        &pwa_data.rust_project_name,
        &pwa_data.project_homepage,
    );
    add_automation_gitignore_to_zip(&mut zip, &now, &pwa_data.rust_project_name);
    add_automation_cargo_toml_to_zip(&mut zip, &now, &pwa_data.rust_project_name);
    add_src_lib_to_zip(&mut zip, &now, &pwa_data.rust_project_name);
    add_src_dom_mod_to_zip(&mut zip, &now, &pwa_data.rust_project_name);
    add_src_web_sys_mod_to_zip(&mut zip, &now, &pwa_data.rust_project_name);

    add_license_to_zip(
        &mut zip,
        &now,
        &pwa_data.rust_project_name,
        &pwa_data.project_author,
    );
    add_cargo_toml_to_zip(
        &mut zip,
        &now,
        &pwa_data.rust_project_name,
        &pwa_data.pwa_description,
        &pwa_data.project_author,
        &pwa_data.project_homepage,
        &pwa_data.project_repository,
    );
    add_readme_md_to_zip(
        &mut zip,
        &now,
        &pwa_data.rust_project_name,
        &pwa_data.project_homepage,
        &pwa_data.project_repository,
        &pwa_data.project_author,
        &pwa_data.pwa_description,
    );
    add_gitignore_to_zip(&mut zip, &now, &pwa_data.rust_project_name);
    add_vscode_settings_to_zip(&mut zip, &now, &pwa_data.rust_project_name);

    let url = finish_zip(&mut zip);
    append_anchor_for_file_url(&url, "new_rust_wasm_pwa.zip");
    append_final_comment("Finally extract the zip and open the new folder in your favorite Rust code editor. There read the README.md for further instructions.");
}

/// create a zip
pub fn create_new_zip(buf: &mut [u8]) -> zip::ZipWriter<std::io::Cursor<&mut [u8]>> {
    debug_write(&format!("create_new_zip"));
    let w = std::io::Cursor::new(buf);
    let zip = zip::ZipWriter::new(w);
    // return
    zip
}

/// resize img and append anchor
pub fn resize_img_and_add_to_zip(
    zip: &mut zip::ZipWriter<std::io::Cursor<&mut [u8]>>,
    img: &image::DynamicImage,
    img_size: u32,
    file_name: &str,
    now: &zip::DateTime,
    rust_project_name: &str,
) {
    debug_write(&format!("resize_img_and_add_to_zip img {}", img_size));
    let new_img = img.resize(img_size, img_size, image::imageops::FilterType::Lanczos3);
    let vec_u8 = encode_to_png(new_img);

    let options = zip::write::FileOptions::default()
        .compression_method(zip::CompressionMethod::Stored)
        .last_modified_time(*now);
    let file_name =
        format!("{rust_project_name}/web_server_folder/{rust_project_name}/icons/{file_name}");
    unwrap!(zip.start_file(&file_name, options));
    unwrap!(zip.write(&vec_u8));
}

/// add manifest.json to zip
pub fn add_manifest_json_to_zip(
    zip: &mut zip::ZipWriter<std::io::Cursor<&mut [u8]>>,
    pwa_short_name: &str,
    pwa_name: &str,
    rust_project_name: &str,
    now: &zip::DateTime,
) {
    debug_write(&format!("add_manifest_json_to_zip"));

    let options = zip::write::FileOptions::default()
        .compression_method(zip::CompressionMethod::Stored)
        .last_modified_time(*now);
    unwrap!(zip.start_file(
        &format!("{rust_project_name}/web_server_folder/{rust_project_name}/manifest.json"),
        options
    ));

    unwrap!(zip.write(
        r##"{
"short_name": "{pwa_short_name}",
"name": "{pwa_name}",
"icons": [
    {
        "src": "icons/icon-072.png",
        "sizes": "72x72",
        "type": "image/png",
        "density": "1.5"
    },
    {
        "src": "icons/icon-096.png",
        "sizes": "96x96",
        "type": "image/png",
        "density": "2.0"
    },
    {
        "src": "icons/icon-128.png",
        "sizes": "128x128",
        "type": "image/png",
        "density": "2.5"
    },
    {
        "src": "icons/icon-144.png",
        "sizes": "144x144",
        "type": "image/png",
        "density": "3.0"
    },
    {
        "src": "icons/icon-152.png",
        "sizes": "152x152",
        "type": "image/png",
        "density": "3.2"
    },
    {
        "src": "icons/icon-192.png",
        "sizes": "192x192",
        "type": "image/png",
        "density": "4.0"
    },
    {
        "src": "icons/icon-512.png",
        "sizes": "512x512",
        "type": "image/png"            
    },
    {
        "src": "icons/icon-maskable.png",
        "sizes": "192x192",
        "type": "image/png",
        "density": "4.0",
        "purpose": "any maskable"
    }
],
"start_url": "/{rust_project_name}/index.html",
"background_color": "#000000",
"display": "standalone",
"orientation": "portrait",
"theme_color": "#000000"
}"##
        .replace("{pwa_short_name}", pwa_short_name)
        .replace("{pwa_name}", pwa_name)
        .replace("{rust_project_name}", rust_project_name)
        .as_bytes()
    ));
}

/// add index.html to zip
pub fn add_index_html_to_zip(
    zip: &mut zip::ZipWriter<std::io::Cursor<&mut [u8]>>,
    pwa_name: &str,
    pwa_description: &str,
    now: &zip::DateTime,
    rust_project_name: &str,
) {
    debug_write(&format!("add_index_html_to_zip"));
    let options = zip::write::FileOptions::default()
        .compression_method(zip::CompressionMethod::Stored)
        .last_modified_time(*now);
    unwrap!(zip.start_file(
        &format!("{rust_project_name}/web_server_folder/{rust_project_name}/index.html"),
        options
    ));
    unwrap!(zip.write(
        r##"
<!DOCTYPE html>
<html lang="en">
    <head>
        <!-- classic header for a web page -->
        <meta http-equiv="Content-type" content="text/html; charset=utf-8" />
        <title>{pwa_name}</title>
        <meta name="Description" content="{pwa_description}">
        <meta name="viewport" content="width = device-width,initial-scale = 1.0" />
        <link rel="stylesheet" href="css/{rust_project_name}.css">
            
        <!-- favicons generic-->
        <link rel="icon" type="image/png" href="icons/icon-032.png" sizes="32x32">
        <link rel="icon" type="image/png" href="icons/icon-128.png" sizes="128x128">
        <link rel="icon" type="image/png" href="icons/icon-192.png" sizes="192x192">
        <!-- favicons Android -->
        <link rel="shortcut icon" href="icons/icon-196.png" sizes="196x196">
        <!-- favicons iOS -->
        <link rel="apple-touch-icon" href="icons/icon-152.png" sizes="152x152">
        <link rel="apple-touch-icon" href="icons/icon-167.png" sizes="167x167">
        <link rel="apple-touch-icon" href="icons/icon-180.png" sizes="180x180">

        <!-- Metadata for PWA -->
        <link rel="manifest" href="manifest.json">
        <meta name="mobile-web-app-capable" content="yes">
        <meta name="apple-mobile-web-app-capable" content="yes" />
        <meta name="apple-mobile-web-app-status-bar-style" content="black-translucent" />
        <meta name="theme-color" content="#000000">
        <link rel="apple-touch-icon" sizes="120x120" href="icons/icon-120.png">
        <link rel="apple-touch-icon" sizes="180x180" href="icons/icon-180.png">
    </head>
<body>
    <!-- a standard service worker is a must for PWA -->
    <script src="start_service_worker.js"></script>
    <!-- warning if javascript is not enabled -->
    <noscript>
        <h2>
                !!!???!!!<br>
                This web app <br>
                cannot work <br>
                without javascript<br>
                enabled<br>
                !!!???!!!</h2>
    </noscript>

        <!-- display a text while waiting for wasm download. 
      This content will change from the wasm code.-->
      <div id="div_for_wasm_html_injecting">
        <h2>
            Waiting to<br> download <br> the web app...<br> This is <br> very quick on fast<br> networks...
            <br>
        </h2>
    </div>

    <!-- import and init the wasm code -->
    <script type="module">
        import init from "./pkg/{rust_project_name}.js"; init("./pkg/{rust_project_name}_bg.wasm");
    </script>
</body>
</html>
    "##
        .replace("{pwa_name}", &html_encode(&pwa_name))
        .replace("{pwa_description}", &html_encode(&pwa_description))
        .replace("{rust_project_name}", &html_encode(&rust_project_name))
        .as_bytes()
    ));
}

/// add service_worker.js to zip
pub fn add_service_worker_js_to_zip(
    zip: &mut zip::ZipWriter<std::io::Cursor<&mut [u8]>>,
    now: &zip::DateTime,
    rust_project_name: &str,
) {
    debug_write(&format!("add_service_worker_js_to_zip"));
    let options = zip::write::FileOptions::default()
        .compression_method(zip::CompressionMethod::Stored)
        .last_modified_time(*now);
    unwrap!(zip.start_file(
        &format!("{rust_project_name}/web_server_folder/{rust_project_name}/service_worker.js"),
        options
    ));
    let version_from_date = format!(
        "{}.{}{:02}.{}{:02}",
        now.year(),
        now.month(),
        now.day(),
        now.hour(),
        now.minute()
    );
    unwrap!(zip.write(
        r##"
'use strict';

// Incrementing VERSION in CACHE_NAME will kick off the 
// install event and force previously cached
// resources to be cached again.
// but the new service worker will not be activated until all 
// tabs with this webapp are closed.

const CACHE_NAME = '{version_from_date}';

self.addEventListener('install', event => {
    console.log('event install ', CACHE_NAME);
    // the ugly trick of avoiding the waiting phase
    self.skipWaiting();

    event.waitUntil(
        caches.open(CACHE_NAME).then(function (cache) {
            return cache.addAll(
                [
                    'index.html',
                ]
            );
        })
    );
});

self.addEventListener('activate', event => {
    console.log('event activate');
    // Delete all caches that aren't CACHE_NAME.
    event.waitUntil(
        caches.keys().then(cacheNames => {
            return Promise.all(
                cacheNames.map(cacheName => {
                    if (CACHE_NAME.indexOf(cacheName) === -1) {
                        // If this cache name isn't right, then delete it.
                        console.log('Deleting out of date cache:', cacheName);
                        return caches.delete(cacheName);
                    }
                })
            );
        })
    );
});

self.addEventListener('fetch', event => {
    // console.log('event fetch');
    // Let the browser do its default thing
    // for non-GET requests.
    if (event.request.method != 'GET') return;

    // Prevent the default, and handle the request ourselves.
    event.respondWith(async function () {
        // Try to get the response from a cache.
        const cache = await caches.open(CACHE_NAME);
        const cachedResponse = await cache.match(event.request);

        if (cachedResponse) {
            // console.log('from cache');
            // If we found a match in the cache, return it, but also
            // update the entry in the cache in the background.
            event.waitUntil(cache.add(event.request));
            return cachedResponse;
        }

        // If we didn't find a match in the cache, use the network and cache it for later.
        const response = await fetch(event.request);
        cache.put(event.request, response.clone());
        return response;
    }());
});
"##
        .replace("{version_from_date}", &version_from_date)
        .as_bytes()
    ));
}

/// add start_service_worker.js to zip
pub fn add_start_service_worker_js_to_zip(
    zip: &mut zip::ZipWriter<std::io::Cursor<&mut [u8]>>,
    now: &zip::DateTime,
    rust_project_name: &str,
) {
    debug_write(&format!("add_start_service_worker_js_to_zip"));
    let options = zip::write::FileOptions::default()
        .compression_method(zip::CompressionMethod::Stored)
        .last_modified_time(*now);
    unwrap!(zip.start_file(
        &format!(
            "{rust_project_name}/web_server_folder/{rust_project_name}/start_service_worker.js"
        ),
        options
    ));
    unwrap!(zip.write(
        r##"
if ('serviceWorker' in navigator) {
    navigator.serviceWorker.register('service_worker.js').then(function (registration) {
        console.log('Registration succeeded.');
    }).catch(function (error) {
        console.log('Registration failed with ' + error);
    });
};
//Listen for claiming of our ServiceWorker
navigator.serviceWorker.addEventListener('controllerchange', function () {
    console.log('Service worker status changed: ', this.controller.state);
    // Listen for changes in the state of our ServiceWorker
    navigator.serviceWorker.controller.addEventListener('statechange', function () {
        // If the ServiceWorker becomes "activated", let the user know they can go offline!
        if (this.state === 'activated') {
            window.location.reload();
        }
    });
});
"##
        .as_bytes()
    ));
}

/// add css to zip
pub fn add_css_to_zip(
    zip: &mut zip::ZipWriter<std::io::Cursor<&mut [u8]>>,
    now: &zip::DateTime,
    rust_project_name: &str,
) {
    debug_write(&format!("add_css_to_zip"));
    let options = zip::write::FileOptions::default()
        .compression_method(zip::CompressionMethod::Stored)
        .last_modified_time(*now);
    unwrap!(zip.start_file(
        &format!(
            "{rust_project_name}/web_server_folder/{rust_project_name}/css/{rust_project_name}.css"
        ),
        options
    ));
    unwrap!(zip.write(
        r##"
/* css variables */
:root {
    /* color palette */
    /* use of variables: var(--color_tooltip_1); */
    /* users can simply change the colors with the chrome extension: user CSS
    https://chrome.google.com/webstore/detail/user-css/okpjlejfhacmgjkmknjhadmkdbcldfcb
    */
    /* background color */
    --b_color_body: #24292E;
    --b_color_button: dodgerblue;
    /* front color */
    --f_color_body: #c4cce0;
    --f_color_link: white;
}

/* region: basics */

html {
    font-family: 'Roboto', sans-serif;
    max-width: 1200px;
    min-width: 300px;
    width: 100%;
    /*margin auto means centered horizontally*/
    margin: auto;
    padding-right: 0px;
    overflow-y: auto;
    overflow-x: hidden;
    word-wrap: break-word;
    overflow-wrap: break-word;
    box-sizing: border-box;
    background-color: var(--b_color_body);
    line-height: 1.5;
    color: var(--f_color_body);
    /*This is the base font-size. All other font-size 
use rem units that are
relative to this font-size.*/
    /*width greater than 600 px*/
    font-size: 34px;
    -webkit-font-smoothing: antialiased;
    text-shadow: 1px 1px 1px rgba(0, 0, 0, 0.004);
}

body {
    max-width: 600px;
    margin: 0;
    padding: 0;
    font-size: 60%;
    line-height: 1.5;
    background-color: var(--b_color_body);
    color: var(--f_color_body);
}
/*CSS Reset*/

div, span, applet, object, iframe, h1, h2, h3, h4, h5, h6, p, blockquote, pre, 
a, abbr, acronym, address, big, cite, code, del, dfn, em, font, img, ins, kbd, 
q, s, samp, small, strike, strong, sub,
sup, tt, var, b, u, i, center, dl, dt, dd, ol, ul, li, fieldset, form, label,
legend, table, caption, tbody, tfoot, thead, tr, th,
td {
    margin: 0;
    padding: 0;
    border: 0;
    border-style: none;
    outline: 0;
    vertical-align: baseline;
    background: transparent;
}

/* no color */
a:link,
a:visited,
a:hover,
a:active {
    color: inherit;
    text-decoration: none;
}

h1,
h2,
h3,
h4 {
    margin-bottom: 16px;
}

p {
    line-height: 1.5;
}

pre {
    white-space: pre-wrap;
}

input[type=text] {
    background-color: var(--b_color_code);
    color: var(--f_color_code);
    width: 600px;
    border: 1px;
    border-radius: 3px;
    padding: 2px;
    font-size: 100%;
    font-family: Consolas, Liberation Mono, Courier, monospace;
}

/* endregion: basics */

.button {
    background: var(--b_color_button);
    border-radius: 6px;
    font-size: 120%;
}

"##
        .as_bytes()
    ));
}

/// finish zip
pub fn finish_zip(zip: &mut zip::ZipWriter<std::io::Cursor<&mut [u8]>>) -> String {
    debug_write(&format!("finish_zip"));
    let zip_result: std::io::Cursor<&mut [u8]> = unwrap!(zip.finish());
    let pos = zip_result.position();
    let mut vec_u8 = zip_result.into_inner().to_vec();
    vec_u8.truncate(pos as usize);
    let url = create_download_url(vec_u8);
    // return
    url
}

/// decode png
pub fn decode_png(vec: Vec<u8>) -> image::DynamicImage {
    let img = image::io::Reader::new(std::io::Cursor::new(vec));
    let img = unwrap!(img.with_guessed_format());
    let img = unwrap!(img.decode());
    // return
    img
}

/// encode to png
pub fn encode_to_png(new_img: image::DynamicImage) -> Vec<u8> {
    debug_write(&format!("encode new_img"));
    let vec_u8: Vec<u8> = Vec::new();
    let mut cursor_1 = std::io::Cursor::new(vec_u8);
    let _x = unwrap!(new_img.write_to(&mut cursor_1, image::ImageOutputFormat::Png));
    // return
    cursor_1.get_ref().to_owned()
}

// favicon.ico with 16 and 32 icons
pub fn encode_to_favicon_ico_and_add_to_zip(
    zip: &mut zip::ZipWriter<std::io::Cursor<&mut [u8]>>,
    img: &image::DynamicImage,
    now: &zip::DateTime,
    rust_project_name: &str,
) {
    // Create a new, empty icon collection:
    let mut icon_dir = ico::IconDir::new(ico::ResourceType::Icon);
    favicon_add_entry(img, 16, &mut icon_dir);
    favicon_add_entry(img, 32, &mut icon_dir);
    favicon_add_entry(img, 48, &mut icon_dir);

    // Finally, add the ICO file to zip:
    let options = zip::write::FileOptions::default()
        .compression_method(zip::CompressionMethod::Stored)
        .last_modified_time(*now);
    unwrap!(zip.start_file(
        &format!("{rust_project_name}/web_server_folder/{rust_project_name}/favicon.ico"),
        options
    ));
    unwrap!(icon_dir.write(zip));
}

pub fn favicon_add_entry(img: &image::DynamicImage, size: u32, icon_dir: &mut ico::IconDir) {
    // icons need smaller images 48, 32 and 16
    let img_rgba_vec = img
        .resize(size, size, image::imageops::FilterType::Lanczos3)
        .into_rgba8()
        .into_raw();
    // create an IconImage from raw RGBA pixel data from another image library
    let icon_image = ico::IconImage::from_rgba_data(size, size, img_rgba_vec);
    icon_dir.add_entry(ico::IconDirEntry::encode(&icon_image).unwrap());
}

/// add .github/workflows/rust.yml to zip
pub fn add_rust_yml_to_zip(
    zip: &mut zip::ZipWriter<std::io::Cursor<&mut [u8]>>,
    now: &zip::DateTime,
    rust_project_name: &str,
) {
    debug_write(&format!("add_rust_yml_to_zip"));
    let options = zip::write::FileOptions::default()
        .compression_method(zip::CompressionMethod::Stored)
        .last_modified_time(*now);
    unwrap!(zip.start_file(
        &format!("{rust_project_name}/.github/workflows/rust.yml"),
        options
    ));
    unwrap!(zip.write(
        r##"
name: RustAction

on:
  push:
    branches: [ main ]
  pull_request:
    branches: [ main ]

env:
  CARGO_TERM_COLOR: always

jobs:
  fmt_build_test:

    runs-on: ubuntu-latest

    steps:
    - uses: jetli/wasm-pack-action@v0.3.0
      with:
        version: 'latest'
    - uses: actions/checkout@v2
    - name: Fmt
      run: cargo fmt -- --check
    - name: Build
      run: wasm-pack build --target web --release

"##
        .as_bytes()
    ));
}

/// add .github/workflows/rust.yml to zip
pub fn add_automation_tasks_rs_main_to_zip(
    zip: &mut zip::ZipWriter<std::io::Cursor<&mut [u8]>>,
    now: &zip::DateTime,
    rust_project_name: &str,
    project_homepage: &str,
) {
    debug_write(&format!("add_automation_tasks_rs_main_to_zip"));
    let options = zip::write::FileOptions::default()
        .compression_method(zip::CompressionMethod::Stored)
        .last_modified_time(*now);
    unwrap!(zip.start_file(
        &format!("{rust_project_name}/automation_tasks_rs/src/main.rs"),
        options
    ));
    unwrap!(zip.write(
        r##"
//! automation_tasks_rs for {rust_project_name}

use cargo_auto_lib::*;

// ANSI colors for Linux terminal
// https://github.com/shiena/ansicolor/blob/master/README.md
#[allow(dead_code)]
pub const RED: &str = "\x1b[31m";
#[allow(dead_code)]
pub const YELLOW: &str = "\x1b[33m";
#[allow(dead_code)]
pub const GREEN: &str = "\x1b[32m";
#[allow(dead_code)]
pub const RESET: &str = "\x1b[0m";


fn main() {
    exit_if_not_run_in_rust_project_root_directory();

    // get CLI arguments
    let mut args = std::env::args();
    // the zero argument is the name of the program
    let _arg_0 = args.next();
    match_arguments_and_call_tasks(args);
}

// region: match, help and completion

/// match arguments and call tasks functions
fn match_arguments_and_call_tasks(mut args: std::env::Args) {
    // the first argument is the user defined task: (no argument for help), build, release,...
    let arg_1 = args.next();
    match arg_1 {
        None => print_help(),
        Some(task) => {
            if &task == "completion" {
                completion();
            } else {
                println!("{YELLOW}Running automation task: {task}{RESET}");
                if &task == "build" {
                    task_build();
                } else if &task == "doc" {
                    task_doc();
                } else if &task == "test" {
                    task_test();
                } else if &task == "commit_and_push" {
                    let arg_2 = args.next();
                    task_commit_and_push(arg_2);
                } else if &task == "publish_to_web" {
                    task_publish_to_web();
                } else {
                    println!("{RED}Error: Task {task} is unknown.{RESET}");
                    print_help();
                }
            }
        }
    }
}

/// write a comprehensible help for user defined tasks
fn print_help() {
    println!(
        r#"
    {YELLOW}Welcome to cargo-auto !
    This program automates your custom tasks when developing a Rust project.{RESET}

    User defined tasks in automation_tasks_rs:
cargo auto build - builds the crate with wasm-pack, fmt
cargo auto doc - builds the docs, copy to docs directory
cargo auto test - runs all the tests
cargo auto commit_and_push "message" - commits with message and push with mandatory message
    (If you use SSH, it is easy to start the ssh-agent in the background and ssh-add your credentials for git.)
cargo auto publish_to_web - publish to web, git tag
    (You need credentials for publishing over SSH. Use ssh-agent and ssh-add to store the credentials for SSH.)
"#,
    );
    print_examples_cmd();
}

/// all example commands in one place
fn print_examples_cmd(){
/*
    println!(r#"run examples:
cargo run --example example1
"#);
*/
}

/// sub-command for bash auto-completion of `cargo auto` using the crate `dev_bestia_cargo_completion`
fn completion() {
    let args: Vec<String> = std::env::args().collect();
    let word_being_completed = args[2].as_str();
    let last_word = args[3].as_str();

    if last_word == "cargo-auto" || last_word == "auto" {
        let sub_commands = vec!["build", "doc", "test", "commit_and_push", "publish_to_web"];
        completion_return_one_or_more_sub_commands(sub_commands, word_being_completed);
    }
    /*
    // the second level if needed
    else if last_word == "new" {
        let sub_commands = vec!["x"];
        completion_return_one_or_more_sub_commands(sub_commands, word_being_completed);
    }
    */
}

// endregion: match, help and completion

// region: tasks

/// cargo build
fn task_build() {
    let cargo_toml = CargoToml::read();
    auto_version_increment_semver_or_date();
    run_shell_command("cargo fmt");    
    auto_cargo_toml_to_md();
    auto_lines_of_code("");
    run_shell_command("cargo fmt");
    run_shell_command("wasm-pack build --target web");
    run_shell_command("\\rsync -a --delete-after pkg/ web_server_folder/{rust_project_name}/pkg/");
    println!(
        r#"{YELLOW}
    After `cargo auto build` open a separate bash terminal and run once
cd ~/rustprojects/{package_name}/web_server_folder; basic-http-server
    Then leave that terminal to run in the background. It will constantly serve the new files as you build.
    In VSCode open the port 4000 for forwarding from the docker container.
    Finally open the browser in Win10 on 
http://localhost:4000/{package_name}/

    if ok, then,
cargo auto doc
{RESET}"#,
package_name = cargo_toml.package_name(),
    );
    print_examples_cmd();
}

/// cargo doc, then copies to /docs/ folder, because this is a github standard folder
fn task_doc() {
    let cargo_toml = CargoToml::read();
    auto_cargo_toml_to_md();
    auto_lines_of_code("");
    auto_plantuml(&cargo_toml.package_repository().unwrap());
    auto_md_to_doc_comments();

    run_shell_command("cargo doc --no-deps --document-private-items");
    // copy target/doc into docs/ because it is github standard
    run_shell_command("rsync -a --info=progress2 --delete-after target/doc/ docs/");
    // Create simple index.html file in docs directory
    run_shell_command(&format!(
        "echo \"<meta http-equiv=\\\"refresh\\\" content=\\\"0; url={}/index.html\\\" />\" > docs/index.html",
        cargo_toml.package_name().replace("-","_")
    ));
    run_shell_command("cargo fmt");
    // message to help user with next move
    println!(
        r#"{YELLOW}
    After `cargo auto doc`, check `docs/index.html`. If ok, then test the documentation code examples
cargo auto test
{RESET}"#
    );
}

/// cargo test
fn task_test() {
    run_shell_command("cargo test");
    println!(
        r#"{YELLOW}
    After `cargo auto test`. If ok, then 
cargo auto commit_and_push "message"
    with mandatory commit message
{RESET}"#
    );
}

/// commit and push
fn task_commit_and_push(arg_2: Option<String>) {
    match arg_2 {
        None => println!("{RED}Error: Message for commit is mandatory.{RESET}"),
        Some(message) => {
            run_shell_command(&format!(r#"git add -A && git commit --allow-empty -m "{}""#, message));
            run_shell_command("git push");
            println!(
                r#"{YELLOW}
    After `cargo auto commit_and_push "message"`
cargo auto publish_to_web
{RESET}"#
            );
        }
    }
}

/// publish to web
fn task_publish_to_web() {
    println!(r#"{YELLOW}Use ssh-agent and ssh-add to store your credentials for publish to web.{RESET}"#);
    let cargo_toml = CargoToml::read();
    // git tag
    let shell_command = format!(
        "git tag -f -a v{version} -m version_{version}",
        version = cargo_toml.package_version()
    );
    run_shell_command(&shell_command);
    let shell_command = format!(
        "rsync -e ssh -a --info=progress2 --delete-after ~/rustprojects/{package_name}/web_server_folder/ {project_author}@{project_homepage_domain}:/var/www/{project_homepage_domain}/{rust_project_name}/",
        package_name = cargo_toml.package_name()
    );
    run_shell_command(&shell_command);
    println!(
        r#"{YELLOW}
    After `cargo auto publish_to_web`, 
    check 
https://bestia.dev/{package_name}
{RESET}"#,
        package_name = cargo_toml.package_name()
    );
}


// endregion: tasks

"##
.replace("{rust_project_name}", rust_project_name)
.replace("{project_homepage_domain}", project_homepage.trim_start_matches("https://").trim_start_matches("http://"))
.as_bytes()
    ));
}

pub fn add_automation_gitignore_to_zip(
    zip: &mut zip::ZipWriter<std::io::Cursor<&mut [u8]>>,
    now: &zip::DateTime,
    rust_project_name: &str,
) {
    debug_write(&format!("add_automation_gitignore_to_zip"));
    let options = zip::write::FileOptions::default()
        .compression_method(zip::CompressionMethod::Stored)
        .last_modified_time(*now);
    unwrap!(zip.start_file(
        &format!("{rust_project_name}/automation_tasks_rs/.gitignore"),
        options
    ));
    unwrap!(zip.write(
        r##"
/target

"##
        .as_bytes()
    ));
}

pub fn add_automation_cargo_toml_to_zip(
    zip: &mut zip::ZipWriter<std::io::Cursor<&mut [u8]>>,
    now: &zip::DateTime,
    rust_project_name: &str,
) {
    debug_write(&format!("add_automation_cargo_toml_to_zip"));
    let options = zip::write::FileOptions::default()
        .compression_method(zip::CompressionMethod::Stored)
        .last_modified_time(*now);
    unwrap!(zip.start_file(
        &format!("{rust_project_name}/automation_tasks_rs/Cargo.toml"),
        options
    ));
    unwrap!(zip.write(
        r##"
[package]
name = "automation_tasks_rs"
version = "0.1.1"
authors = ["bestia.dev"]
homepage = "https://bestia.dev"
edition = "2021"
description = "cargo auto - automation tasks written in Rust language"
publish = false

[dependencies]
cargo_auto_lib = "0.7.42"

"##
        .as_bytes()
    ));
}

pub fn add_src_lib_to_zip(
    zip: &mut zip::ZipWriter<std::io::Cursor<&mut [u8]>>,
    now: &zip::DateTime,
    rust_project_name: &str,
) {
    debug_write(&format!("add_src_lib_to_zip"));
    let options = zip::write::FileOptions::default()
        .compression_method(zip::CompressionMethod::Stored)
        .last_modified_time(*now);
    unwrap!(zip.start_file(&format!("{rust_project_name}/src/lib.rs"), options));
    unwrap!(zip.write(
        r##"
//! lib.rs is just for the wasm_bindgen_start function and to connect to all the modules.
//! and for the big doc comments
//!
// region: auto_md_to_doc_comments include README.md A //!

// endregion: auto_md_to_doc_comments include README.md A //!

use wasm_bindgen::prelude::*;

mod dom_mod;
mod web_sys_mod;

#[wasm_bindgen(start)]
/// To start the Wasm application, wasm_bindgen runs this functions
pub fn wasm_bindgen_start() -> Result<(), JsValue> {
    // Initialize debugging for when/if something goes wrong.
    console_error_panic_hook::set_once();
    // write the app version just for debug purposes
    web_sys_mod::debug_write(&format!(
        "{} v{}",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    ));
    dom_mod::start_function();
    // return
    Ok(())
}

"##
        .as_bytes()
    ));
}

pub fn add_src_dom_mod_to_zip(
    zip: &mut zip::ZipWriter<std::io::Cursor<&mut [u8]>>,
    now: &zip::DateTime,
    rust_project_name: &str,
) {
    debug_write(&format!("add_src_dom_mod_to_zip"));
    let options = zip::write::FileOptions::default()
        .compression_method(zip::CompressionMethod::Stored)
        .last_modified_time(*now);
    unwrap!(zip.start_file(&format!("{rust_project_name}/src/dom_mod.rs"), options));
    unwrap!(zip.write(
        r###"
//! dom_mod.rs manipulates the html dom

use crate::web_sys_mod::*;

/// The app starts with this function
pub fn start_function() {
    // inject html into DOM
    inject_htm_into_dom();
    // prepare events that read local file, pass the function to execute
    add_listener_to_button("button_1", &on_click_button_1);
}

/// inject html into dom
pub fn inject_htm_into_dom() {
    let html = format!(
        r##"
        <h2 id="hello_text">Hello World !</h2>
		
        <div class="button-wrap">
            <label for="my_name">Enter your name:</label>  
            <input style="width:40%;" type="text" id="my_name" value="my_name"/>
        </div>

        <!--tricky div+label+css to change Input file appearance -->
        <div class="button-wrap">
            <input id="button_1" type="button" class="button" value="Reload"/>
        </div>
        "##
    );

    set_inner_html("div_for_wasm_html_injecting", &html);
}

/// the listener calls this function
fn on_click_button_1() {
    let my_name = get_input_element_value_string_by_id("my_name");
    let my_name = html_encode(&my_name);
    set_inner_html("hello_text", &format!("Hello {my_name} !"));
}

"###
        .as_bytes()
    ));
}

pub fn add_src_web_sys_mod_to_zip(
    zip: &mut zip::ZipWriter<std::io::Cursor<&mut [u8]>>,
    now: &zip::DateTime,
    rust_project_name: &str,
) {
    debug_write(&format!("add_src_web_sys_mod_to_zip"));
    let options = zip::write::FileOptions::default()
        .compression_method(zip::CompressionMethod::Stored)
        .last_modified_time(*now);
    unwrap!(zip.start_file(&format!("{rust_project_name}/src/web_sys_mod.rs"), options));
    unwrap!(zip.write(
        r###"
//! web_sys_mod.rs
//! helper functions for web_sys, window, dom, console, html elements,...
//! Trying to isolate/hide all javascript code and conversion in this module.

// region: use
use unwrap::unwrap;
use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::console;
// endregion: use

/// return the global window object
pub fn window() -> web_sys::Window {
    unwrap!(web_sys::window())
}

/// get element by id
pub fn get_element_by_id(element_id: &str) -> web_sys::Element {
    let document = unwrap!(window().document());
    unwrap!(document.get_element_by_id(element_id))
}

/// debug write into session_storage
pub fn debug_write(text: &str) {
    // writing to the console
    console::log_1(&JsValue::from_str(text));
}

/// get html element by id
pub fn get_html_element_by_id(element_id: &str) -> web_sys::HtmlElement {
    let element = get_element_by_id(element_id);
    let html_element: web_sys::HtmlElement = unwrap!(element.dyn_into::<web_sys::HtmlElement>());
    //return
    html_element
}

/// HTML encode - naive
pub fn html_encode(input: &str) -> String {
    input
        .replace("&", "&amp;")
        .replace("\"", "&quot;")
        .replace("'", "&apos;")
        .replace("<", "&lt;")
        .replace(">", "&gt;")
}

/// get input element value string by id
pub fn get_input_element_value_string_by_id(element_id: &str) -> String {
    let input_element = get_element_by_id(element_id);
    let input_html_element = unwrap!(input_element.dyn_into::<web_sys::HtmlInputElement>());
    input_html_element.value()
}

/// set inner html into dom
/// The inner_html must be correctly HTML encoded !
pub fn set_inner_html(element_id: &str, inner_html: &str) {
    let div_for_wasm_html_injecting = get_element_by_id(element_id);
    div_for_wasm_html_injecting.set_inner_html(inner_html);
}

/// add event listener for button
pub fn add_listener_to_button(element_id: &str, fn_on_click_button: &'static (dyn Fn() + 'static)) {
    let handler_1 = Box::new(move || {
        fn_on_click_button();
    }) as Box<dyn FnMut()>;
    let closure = Closure::wrap(handler_1);

    let html_element = get_html_element_by_id(element_id);
    html_element.set_onclick(Some(closure.as_ref().unchecked_ref()));
    closure.forget();
}

"###
        .as_bytes()
    ));
}

pub fn add_gitignore_to_zip(
    zip: &mut zip::ZipWriter<std::io::Cursor<&mut [u8]>>,
    now: &zip::DateTime,
    rust_project_name: &str,
) {
    debug_write(&format!("add_gitignore_to_zip"));
    let options = zip::write::FileOptions::default()
        .compression_method(zip::CompressionMethod::Stored)
        .last_modified_time(*now);
    unwrap!(zip.start_file(&format!("{rust_project_name}/.gitignore"), options));
    unwrap!(zip.write(
        r##"
/target
/pkg/

"##
        .as_bytes()
    ));
}

pub fn add_license_to_zip(
    zip: &mut zip::ZipWriter<std::io::Cursor<&mut [u8]>>,
    now: &zip::DateTime,
    rust_project_name: &str,
    project_author: &str,
) {
    debug_write(&format!("add_license_to_zip"));
    let options = zip::write::FileOptions::default()
        .compression_method(zip::CompressionMethod::Stored)
        .last_modified_time(*now);
    unwrap!(zip.start_file(&format!("{rust_project_name}/LICENSE"), options));
    unwrap!(zip.write(
        r##"
MIT License

Copyright (c) 2022 {project_author}

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.

"##
        .replace("{project_author}", project_author)
        .as_bytes()
    ));
}

pub fn add_cargo_toml_to_zip(
    zip: &mut zip::ZipWriter<std::io::Cursor<&mut [u8]>>,
    now: &zip::DateTime,
    rust_project_name: &str,
    pwa_description: &str,
    project_author: &str,
    project_homepage: &str,
    project_repository: &str,
) {
    debug_write(&format!("add_cargo_toml_to_zip"));
    let options = zip::write::FileOptions::default()
        .compression_method(zip::CompressionMethod::Stored)
        .last_modified_time(*now);
    unwrap!(zip.start_file(&format!("{rust_project_name}/Cargo.toml"), options));
    unwrap!(zip.write(
        r##"
[package]
name = "{rust_project_name}"
version = "2022.314.1836"
authors = ["{project_author}"]
homepage = "{project_homepage}"
edition = "2021"
description = "{pwa_description}"
repository = "{project_repository}"
readme = "README.md"
license = "MIT"
keywords = ["wasm"]
categories = ["tutorial"]
publish = false

[lib]
# cdylib is for the wasm module library
crate-type = ["cdylib"]

[dependencies]
# the macro unwrap is great for WASM, because it shows the correct file and line number of the error
unwrap = "1.2.1"
wasm-bindgen = { version = "0.2.69", features = ["serde-serialize"] }
console_error_panic_hook = "0.1.6"
js-sys = "0.3.46"

[dependencies.web-sys]
version = "0.3.46"
features = [
  "AbortController",
  "console",
  "Document",
  "Element",
  "ErrorEvent",
  "HtmlElement",
  "HtmlInputElement",
  "Window",
]

[dev-dependencies]
wasm-bindgen-test = "0.3.19"

[profile.release]
panic = "abort"

"##
        .replace("{pwa_description}", pwa_description)
        .replace("{rust_project_name}", rust_project_name)
        .replace("{project_author}", project_author)
        .replace("{project_homepage}", project_homepage)
        .replace("{project_repository}", project_repository)
        .as_bytes()
    ));
}

pub fn add_readme_md_to_zip(
    zip: &mut zip::ZipWriter<std::io::Cursor<&mut [u8]>>,
    now: &zip::DateTime,
    rust_project_name: &str,
    project_homepage: &str,
    project_repository: &str,
    project_author: &str,
    pwa_description:&str,
) {
    debug_write(&format!("add_readme_md_to_zip"));
    let options = zip::write::FileOptions::default()
        .compression_method(zip::CompressionMethod::Stored)
        .last_modified_time(*now);
    unwrap!(zip.start_file(&format!("{rust_project_name}/README.md"), options));
    unwrap!(zip.write(
        r##"
[comment]: # (auto_md_to_doc_comments segment start A)

# {rust_project_name}

[comment]: # (auto_cargo_toml_to_md start)

**{pwa_description}**  
***version: 2021.204.1558  date: 2021-02-04 author: [{project_author}]({project_homepage}) repository: [GitHub]({project_repository})***  

[comment]: # (auto_cargo_toml_to_md end)

[comment]: # (auto_lines_of_code start)
[![Lines in Rust code](https://img.shields.io/badge/Lines_in_Rust-99-green.svg)]({project_repository})
[![Lines in Doc comments](https://img.shields.io/badge/Lines_in_Doc_comments-7-blue.svg)]({project_repository})
[![Lines in Comments](https://img.shields.io/badge/Lines_in_comments-18-purple.svg)]({project_repository})
[![Lines in examples](https://img.shields.io/badge/Lines_in_examples-0-yellow.svg)]({project_repository})
[![Lines in tests](https://img.shields.io/badge/Lines_in_tests-0-orange.svg)]({project_repository})

[comment]: # (auto_lines_of_code end)

[![License](https://img.shields.io/badge/license-MIT-blue.svg)]({project_repository}/blob/master/LICENSE)
[![Rust]({project_repository}/workflows/RustAction/badge.svg)]({project_repository}/)

Hashtags: #rustlang #tutorial #pwa #wasm #webassembly  

## Try it

<{project_homepage}/{rust_project_name}/>  

## Template

This is a template of a simple Rust Wasm PWA.  
It is created with this PWA utility:  
<https://bestia.dev/bestia_dev_new_rust_wasm_pwa>

## Development of the utility

Use `cargo-auto` to automate development tasks: `cargo install cargo-auto`.  
Then inside the Rust project folder run `cargo auto` for the instructions.
PWA files MUST be served by a web server. We will use the most simple development web server:  
`cargo install basic-http-server`.  
Open a new terminal window in VSCode and go to the web server folder and run the server:  
`cd ~/rustprojects/{rust_project_name}/web_server_folder; basic-http-server`  
Inside VSCode add the port 4000 for forwarding out of the docker container.
Open the browser in Win10 on:  
<http://127.0.0.1:4000/{rust_project_name}/>  

[comment]: # (auto_md_to_doc_comments segment end A)

"##
.replace("{rust_project_name}", rust_project_name)
.replace("{project_homepage}", project_homepage)
.replace("{project_repository}", project_repository)
.replace("{project_author}", project_author)
.replace("{pwa_description}", pwa_description)
.as_bytes()
    ));
}

pub fn add_vscode_settings_to_zip(
    zip: &mut zip::ZipWriter<std::io::Cursor<&mut [u8]>>,
    now: &zip::DateTime,
    rust_project_name: &str,
) {
    debug_write(&format!("add_vscode_settings_to_zip"));
    let options = zip::write::FileOptions::default()
        .compression_method(zip::CompressionMethod::Stored)
        .last_modified_time(*now);
    unwrap!(zip.start_file(
        &format!("{rust_project_name}/.vscode/settings.json"),
        options
    ));
    unwrap!(zip.write(
        r##"
{
    "cSpell.words": [
        "apos",
        "bestia",
        "bindgen",
        "cdylib",
        "endregion",
        "jetli",
        "onclick",
        "plantuml",
        "rustprojects",
        "serde"
    ]
}
"##
        .as_bytes()
    ));
}
