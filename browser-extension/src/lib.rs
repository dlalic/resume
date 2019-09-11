use gloo_events::EventListener;
use gloo_events::EventListenerOptions;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::console;

#[wasm_bindgen]
pub fn handle_message() -> () {
    // TODO: display suggestion
}

fn change_page(body: &web_sys::Element, element_class: &str, page: u32) {
    let elements = body.get_elements_by_class_name(element_class);
    // TODO:
    console::log_1(&JsValue::from_str(&format!("{:?}", elements)));
}

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    let options = EventListenerOptions::enable_prevent_default();

    // Settings

    let clipboard_button = document.get_element_by_id("clipboard").unwrap();
    EventListener::new(&clipboard_button, "click", move |_event| {
        // TODO: implement clipboard setting
    })
    .forget();

    let suggestions_button = document.get_element_by_id("suggestions").unwrap();
    EventListener::new(&suggestions_button, "click", move |_event| {
        // TODO: implement suggestions setting
    })
    .forget();

    // Pagination

    let pages = vec!["employment", "education"];
    for page in pages {
        let previous_button_id = format!("{}__previous", page);
        let previous_button = document.get_element_by_id(&previous_button_id).unwrap();
        EventListener::new_with_options(&previous_button, "click", options, move |event| {
            event.prevent_default();
            // TODO: implement previous
        })
        .forget();

        let next_button_id = format!("{}__next", page);
        let next_button = document.get_element_by_id(&next_button_id).unwrap();
        EventListener::new_with_options(&next_button, "click", options, move |event| {
            event.prevent_default();
            // TODO: implement next
        })
        .forget();

        change_page(&body, page, 0);
    }

    // Clipboard

    let text_areas = body.get_elements_by_tag_name("textarea");
    for i in 0..text_areas.length() {
        let element = text_areas.get_with_index(i).unwrap();
        EventListener::new(&element, "click", move |event| {
            let target = event.target().unwrap();
            let text_area = target.unchecked_into::<web_sys::HtmlTextAreaElement>();
            text_area.select();
            copy_to_clipboard();
        })
        .forget();
    }
    Ok(())
}

fn copy_to_clipboard() {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let html_document = document.unchecked_into::<web_sys::HtmlDocument>();
    let _ = html_document.exec_command("copy");
}
