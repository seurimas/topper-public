use topper_aetolia::explainer::sect_parser::AetoliaSectParser;
use wasm_bindgen::prelude::*;

extern crate wasm_bindgen;

#[wasm_bindgen]
pub fn parse_html_to_page(html_content: String) -> String {
    let mut parser = AetoliaSectParser::new();
    let result = parser.parse_nodes(html_content);
    match result {
        Ok(page) => serde_json::to_string(&page).unwrap_or("{}".to_string()),
        Err(err) => format!("{{\"error\": \"{}\"}}", err),
    }
}
