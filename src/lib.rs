use wasm_bindgen::prelude::*;
use pulldown_cmark::{Parser, html::push_html};

#[wasm_bindgen]
pub fn convert_markdown_to_html(markdown_input: &str) -> String {
    let parser = Parser::new(markdown_input);          // Step 1: Parse markdown
    let mut html_output = String::new();               // Step 2: Output buffer
    push_html(&mut html_output, parser);               // Step 3: Convert to HTML
    html_output                                         // Step 4: Return result
}
