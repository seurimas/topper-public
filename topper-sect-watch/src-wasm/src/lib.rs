mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn initialize_page(page_string: &str) {
    let page = ExplainerPage::from_json(page_string);
}
