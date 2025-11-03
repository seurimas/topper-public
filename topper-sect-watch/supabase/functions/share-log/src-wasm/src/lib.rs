use topper_aetolia::explainer::sect_parser::AetoliaSectParser;
use wasm_bindgen::prelude::*;

extern crate wasm_bindgen;

#[wasm_bindgen]
pub fn parse_html_to_page(
    html_content: String,
    filtered_body: JsValue,
    filtered_commands: JsValue,
) -> String {
    let mut parser = AetoliaSectParser::new();
    let result = parser.parse_nodes(html_content);
    match result {
        Ok(mut page) => {
            for body in
                serde_wasm_bindgen::from_value::<Vec<String>>(filtered_body).unwrap_or_default()
            {
                page.filter_out_from_body(&body);
            }
            for command in
                serde_wasm_bindgen::from_value::<Vec<String>>(filtered_commands).unwrap_or_default()
            {
                page.filter_out_command(&command);
            }
            serde_json::to_string(&page).unwrap_or("{}".to_string())
        }
        Err(err) => format!("{{\"error\": \"{}\"}}", err),
    }
}
