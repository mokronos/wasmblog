use leptos::{error::Result, *};
use serde::{Deserialize, Serialize};
use js_sys::eval;

fn main() {
    mount_to_body(|| view! {
        <App/>
    });
}

#[component]
fn App() -> impl IntoView {
    view! {
        <SearchBar/>
        <Content/>
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct Json {
    title: String,
    date: String,
    url: String,
}

async fn fetch_json(_: ()) -> Result<Vec<Json>> {
    let path = format!("json/frontmatters.json");
    log(format!("fetching json: {}", path).as_str());
    let res = reqwasm::http::Request::get(
        &path
        )
        .send()
        .await?
        .json::<Vec<Json>>()
        .await?;
    Ok(res)
}

#[component]
fn SearchBar() -> impl IntoView {
    let (url, set_url) = create_signal("".to_string());
    let options = create_local_resource(||(), fetch_json );

    view! {
        <div>
            <input list="urls" type="text"/>
            <datalist id="urls">
            </datalist>
            <p>{url}</p>
        </div>
    }
}

#[component]
fn Content() -> impl IntoView {
    let (html, set_html) = create_signal("".to_string());
    let (url, set_url) = create_signal("ml_glossary".to_string());
    let html = create_local_resource(url, fetch_html );

    let final_html = move || match html() {
        None => "Loading...".to_string(),
        Some(data) => data.unwrap_or_else(|_| "Error".to_string()),
    };

    create_effect(move |_| {
        // this seems really wrong but it works
        // need to somehow run this after the html is rendered
        // need to still figure out if this runs after the html renders
        // or its just lucky that it works
        final_html();
        refresh_math();
    });

    view! {
        <div>
            <div id="content" inner_html=final_html/>
        </div>
    }
}

async fn fetch_html(url: String) -> Result<String> {
    let path = format!("json/{}.html", url);
    log(format!("fetching html: {}", path).as_str());
    let res = reqwasm::http::Request::get(
        &path
        )
        .send()
        .await?
        .text()
        .await?;
        
    Ok(res)
}

fn log(s: &str) {
    let code = format!( r#" console.log('{}'); "#, s);
    eval(&code).expect("Failed to log message");
}

fn refresh_math() {
    let code = r#"MathJax.typeset();"#;
    eval(&code).expect("Failed to refresh math");
    log("refreshed mathjax");
}
