mod block;
mod blocks;
mod html;
mod span;
mod spans;

use html::Html;

pub fn generate(markdown: impl Html, title: &str, style: &str) -> String {
    let html = markdown.html();
    String::from(format!(
        "<!doctype html>
        <meta charset=\"utf-8\">
        <title>{}</title>
    <style>
    {}
    </style>
    <body>
    <main>
    {}
    </main>
    </body>",
        title, style, html,
    ))
}
