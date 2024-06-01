//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

use gethostname::gethostname;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_gethostname_wasm() {
    let hostname = gethostname();

    assert_eq!(hostname.to_string_lossy(), "127.0.0.1");
}
