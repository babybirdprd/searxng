use askama::Template;
use crate::models::SearchResult;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    pub instance_name: String,
}

#[derive(Template)]
#[template(path = "results.html")]
pub struct ResultsTemplate {
    pub query: String,
    pub results: Vec<SearchResult>,
    pub instance_name: String,
}

#[derive(Template)]
#[template(path = "opensearch.xml", escape = "xml")]
pub struct OpenSearchTemplate {
    pub instance_name: String,
    pub base_url: String,
}

#[derive(Template)]
#[template(path = "atom.xml", escape = "xml")]
pub struct AtomTemplate {
    pub query: String,
    pub results: Vec<SearchResult>,
    pub instance_name: String,
    pub base_url: String,
}

#[derive(Template)]
#[template(path = "rss.xml", escape = "xml")]
pub struct RssTemplate {
    pub query: String,
    pub results: Vec<SearchResult>,
    pub instance_name: String,
    pub base_url: String,
}

pub mod helpers {
    use std::collections::HashMap;

    pub fn icon(name: &str) -> String {
        let catalog = get_icon_catalog();
        catalog.get(name)
            .cloned()
            .unwrap_or_else(|| "")
            .replace("__jinja_class_placeholder__", "sxng-icon-set")
    }

    pub fn icon_big(name: &str) -> String {
        let catalog = get_icon_catalog();
        catalog.get(name)
            .cloned()
            .unwrap_or_else(|| "")
            .replace("__jinja_class_placeholder__", "sxng-icon-set-big")
    }

    fn get_icon_catalog() -> HashMap<&'static str, &'static str> {
        let mut m = HashMap::new();
        m.insert("alert", r#"<svg viewBox="0 0 512 512" class="ionicon __jinja_class_placeholder__" aria-hidden="true"><path d="M256 80c-8.66 0-16.58 7.36-16 16l8 216a8 8 0 0 0 8 8h0a8 8 0 0 0 8-8l8-216c.58-8.64-7.34-16-16-16" fill="none" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="32px"/><circle cx="256" cy="416" r="16" fill="none" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="32px"/></svg>"#);
        m.insert("settings", r#"<svg viewBox="0 0 512 512" class="ionicon __jinja_class_placeholder__" aria-hidden="true"><path d="M262.29 192.31a64 64 0 1 0 57.4 57.4 64.13 64.13 0 0 0-57.4-57.4M416.39 256a154 154 0 0 1-1.53 20.79l45.21 35.46a10.81 10.81 0 0 1 2.45 13.75l-42.77 74a10.81 10.81 0 0 1-13.14 4.59l-44.9-18.08a16.11 16.11 0 0 0-15.17 1.75A164.5 164.5 0 0 1 325 400.8a15.94 15.94 0 0 0-8.82 12.14l-6.73 47.89a11.08 11.08 0 0 1-10.68 9.17h-85.54a11.11 11.11 0 0 1-10.69-8.87l-6.72-47.82a16.07 16.07 0 0 0-9-12.22 155 155 0 0 1-21.46-12.57 16 16 0 0 0-15.11-1.71l-44.89 18.07a10.81 10.81 0 0 1-13.14-4.58l-42.77-74a10.8 10.8 0 0 1 2.45-13.75l38.21-30a16.05 16.05 0 0 0 6-14.08c-.36-4.17-.58-8.33-.58-12.5s.21-8.27.58-12.35a16 16 0 0 0-6.07-13.94l-38.19-30A10.81 10.81 0 0 1 49.48 186l42.77-74a10.81 10.81 0 0 1 13.14-4.59l44.9 18.08a16.11 16.11 0 0 0 15.17-1.75A164.5 164.5 0 0 1 187 111.2a15.94 15.94 0 0 0 8.82-12.14l6.73-47.89A11.08 11.08 0 0 1 213.23 42h85.54a11.11 11.11 0 0 1 10.69 8.87l6.72 47.82a16.07 16.07 0 0 0 9 12.22 155 155 0 0 1 21.46 12.57 16 16 0 0 0 15.11 1.71l44.89-18.07a10.81 10.81 0 0 1 13.14 4.58l42.77 74a10.8 10.8 0 0 1-2.45 13.75l-38.21 30a16.05 16.05 0 0 0-6.05 14.08c.33 4.14.55 8.3.55 12.47" fill="none" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="32px"/></svg>"#);
        m.insert("information-circle", r#"<svg viewBox="0 0 512 512" aria-hidden="true" class="__jinja_class_placeholder__"><path d="M248 64C146.39 64 64 146.39 64 248s82.39 184 184 184 184-82.39 184-184S349.61 64 248 64z" fill="none" stroke="currentColor" stroke-miterlimit="10" stroke-width="32"/><path fill="none" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="32" d="M220 220h32v116"/><path fill="none" stroke="currentColor" stroke-linecap="round" stroke-miterlimit="10" stroke-width="32" d="M208 340h88"/><path d="M248 130a26 26 0 1026 26 26 26 0 00-26-26z" fill="currentColor" stroke="currentColor" stroke-miterlimit="10" stroke-width="1"/></svg>"#);
        m.insert("heart", r#"<svg viewBox="0 0 512 512" class="ionicon __jinja_class_placeholder__" aria-hidden="true"><path d="M352.92 80C288 80 256 144 256 144s-32-64-96.92-64c-52.76 0-94.54 44.14-95.08 96.81-1.1 109.33 86.73 187.08 183 252.42a16 16 0 0 0 18 0c96.26-65.34 184.09-143.09 183-252.42-.54-52.67-42.32-96.81-95.08-96.81" fill="none" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="32px"/></svg>"#);
        m.insert("search", r#"<svg viewBox="0 0 512 512" class="ionicon __jinja_class_placeholder__" aria-hidden="true"><path d="M221.09 64a157.09 157.09 0 1 0 157.09 157.09A157.1 157.1 0 0 0 221.09 64Z" fill="none" stroke="currentColor" stroke-miterlimit="10" stroke-width="32px"/><path d="M338.29 338.29 448 448" fill="none" stroke="currentColor" stroke-linecap="round" stroke-miterlimit="10" stroke-width="32px"/></svg>"#);
        m
    }
}
