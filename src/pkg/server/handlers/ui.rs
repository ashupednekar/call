use askama::Template;
use axum::response::Html;

#[derive(Template)]
#[template(path = "index.html")]
struct Index;


pub async fn handle_ui() -> Html<String>{
    Html(Index{}.render().unwrap())
}
