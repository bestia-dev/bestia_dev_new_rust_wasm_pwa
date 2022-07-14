//! prepare_zip_mod.rs should not use any javascript objects. Pure Rust.
//! All the javascript objects, functions and conversion should be in web_sys_mod.rs.

use unwrap::unwrap;

use crate::{web_sys_mod::*, PwaData};

/// create and fill zip
pub fn create_and_fill_zip(vec: Vec<u8>, pwa_data: PwaData) {
    // get date time now
    let now = date_time_now();

    let img = decode_png(vec);
    let mut buf = &mut vec![0u8; 2_097_152];
    let mut zip = create_new_zip(&mut buf);

    // favicon.ico with 16, 32 and 48 icons
    encode_to_favicon_ico_and_add_to_zip(&mut zip, &img, &now, &pwa_data.rust_project_name);

    // png with various sizes for: favicon png, pwa Android and pwa iOS
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
    let file_name = format!("{}/icons/{}", rust_project_name, file_name);
    unwrap!(zip.start_file(&file_name, options));
    use std::io::Write;
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
    unwrap!(zip.start_file(&format!("{}/manifest.json", rust_project_name), options));

    use std::io::Write;
    // format! makes {} special characters. Use double {{ and }} for {} literal.
    unwrap!(zip.write(
        &format!(
            r##"{{
    "short_name": "{}",
    "name": "{}",
    "icons": [
        {{
            "src": "icons/icon-072.png",
            "sizes": "72x72",
            "type": "image/png",
            "density": "1.5"
        }},
        {{
            "src": "icons/icon-096.png",
            "sizes": "96x96",
            "type": "image/png",
            "density": "2.0"
        }},
        {{
            "src": "icons/icon-128.png",
            "sizes": "128x128",
            "type": "image/png",
            "density": "2.5"
        }},
        {{
            "src": "icons/icon-144.png",
            "sizes": "144x144",
            "type": "image/png",
            "density": "3.0"
        }},
        {{
            "src": "icons/icon-152.png",
            "sizes": "152x152",
            "type": "image/png",
            "density": "3.2"
        }},
        {{
            "src": "icons/icon-192.png",
            "sizes": "192x192",
            "type": "image/png",
            "density": "4.0"
        }},
        {{
            "src": "icons/icon-512.png",
            "sizes": "512x512",
            "type": "image/png"            
        }},
        {{
            "src": "icons/icon-maskable.png",
            "sizes": "192x192",
            "type": "image/png",
            "density": "4.0",
            "purpose": "any maskable"
        }}
    ],
    "start_url": "/{}/index.html",
    "background_color": "#000000",
    "display": "standalone",
    "orientation": "portrait",
    "theme_color": "#000000"
}}"##,
            pwa_short_name, pwa_name, rust_project_name
        )
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
    unwrap!(zip.start_file(&format!("{}/index.html", rust_project_name), options));
    let index_html = format!(
        r##"
    <!DOCTYPE html>
    <html lang="en">
        <head>
            <!-- classic header for a web page -->
            <meta http-equiv="Content-type" content="text/html; charset=utf-8" />
            <title>{}</title>
            <meta name="Description" content="{}">
            <meta name="viewport" content="width = device-width,initial-scale = 1.0" />
               
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

        <div id="div_content">
            <h1>Hello PWA world!</h1>
            <p>Install me.</p> 
        </div>
    </body>
</html>
    "##,
        html_encode(&pwa_name),
        html_encode(&pwa_description)
    );
    use std::io::Write;
    unwrap!(zip.write(index_html.as_bytes()));
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
    unwrap!(zip.start_file(&format!("{}/service_worker.js", rust_project_name), options));
    use std::io::Write;
    let version_from_date = format!(
        "{}.{}{:02}.{}{:02}",
        now.year(),
        now.month(),
        now.day(),
        now.hour(),
        now.minute()
    );
    unwrap!(zip.write(
        format!(r##"
            'use strict';

            // Incrementing VERSION in CACHE_NAME will kick off the 
            // install event and force previously cached
            // resources to be cached again.
            // but the new service worker will not be activated until all 
            // tabs with this webapp are closed.
            
            const CACHE_NAME = '{}';
            
            self.addEventListener('install', event => {{
                console.log('event install ', CACHE_NAME);
                // the ugly trick of avoiding the waiting phase
                self.skipWaiting();
            
                event.waitUntil(
                    caches.open(CACHE_NAME).then(function (cache) {{
                        return cache.addAll(
                            [
                                'index.html',
                            ]
                        );
                    }})
                );
            }});
            
            self.addEventListener('activate', event => {{
                console.log('event activate');
                // Delete all caches that aren't CACHE_NAME.
                event.waitUntil(
                    caches.keys().then(cacheNames => {{
                        return Promise.all(
                            cacheNames.map(cacheName => {{
                                if (CACHE_NAME.indexOf(cacheName) === -1) {{
                                    // If this cache name isn't right, then delete it.
                                    console.log('Deleting out of date cache:', cacheName);
                                    return caches.delete(cacheName);
                                }}
                            }})
                        );
                    }})
                );
            }});
            
            self.addEventListener('fetch', event => {{
                // console.log('event fetch');
                // Let the browser do its default thing
                // for non-GET requests.
                if (event.request.method != 'GET') return;
            
                // Prevent the default, and handle the request ourselves.
                event.respondWith(async function () {{
                    // Try to get the response from a cache.
                    const cache = await caches.open(CACHE_NAME);
                    const cachedResponse = await cache.match(event.request);
            
                    if (cachedResponse) {{
                        // console.log('from cache');
                        // If we found a match in the cache, return it, but also
                        // update the entry in the cache in the background.
                        event.waitUntil(cache.add(event.request));
                        return cachedResponse;
                    }}
            
                    // If we didn't find a match in the cache, use the network and cache it for later.
                    const response = await fetch(event.request);
                    cache.put(event.request, response.clone());
                    return response;
                }}());
            }});
"##,version_from_date).as_bytes()
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
        &format!("{}/start_service_worker.js", rust_project_name),
        options
    ));
    use std::io::Write;
    unwrap!(zip.write(
        br##"
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
    let mut vec_u8: Vec<u8> = Vec::new();
    let _x = unwrap!(new_img.write_to(&mut vec_u8, image::ImageOutputFormat::Png));
    // return
    vec_u8
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
    unwrap!(zip.start_file(&format!("{}/favicon.ico", rust_project_name,), options));
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
