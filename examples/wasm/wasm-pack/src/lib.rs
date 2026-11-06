use verdant_parsers_git::{Lang, LanguageSetImpl};
use verdant_syntax::renderer::HtmlRenderer;
use wasm_bindgen::prelude::*;

// run this code when the module is loaded
#[wasm_bindgen(start)]
async fn run() -> Result<(), JsValue> {
    // get global objects
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    // do some highlighting
    let html = verdant_syntax::highlight(
        "fn main() {\n    println!(\"Hello, World!\");\n}",
        Lang::Rust,
        &LanguageSetImpl::new(),
        &mut HtmlRenderer,
        verdant_themes::one::dark(),
    )
    .map_err(JsError::from)?;

    let code_div = document.create_element("div")?;
    code_div.set_class_name("code");
    code_div.set_inner_html(&html);
    body.append_child(&code_div)?;

    Ok(())
}
