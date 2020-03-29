use std::time::Duration;
use wasm_bindgen_test::*;
use wasm_bindgen_test_ext::timeout::Timeout;

wasm_bindgen_test_configure!(run_in_browser);

use conduit_wasm::app::App as YewApp;
use yew::App;

#[wasm_bindgen_test]
async fn home_page_has_articles() {
    let app: App<YewApp> = yew::App::new();
    app.mount(yew::utils::document().get_element_by_id("output").unwrap());

    let articles = yew::utils::document().get_elements_by_class_name("article-preview");

    console_log!("Initial articles length: {}", articles.length());
    assert_eq!(articles.length(), 1);

    console_log!("Waiting for articles to load.");
    Timeout::new(Duration::new(5, 0)).await;

    console_log!("Loaded articles length: {}", articles.length());
    assert_eq!(articles.length(), 10);
}