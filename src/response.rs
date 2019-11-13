use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Response {
    pub query: Query,
}

#[derive(Deserialize, Debug)]
pub struct Query {
    pub search: Vec<SearchResult>,
}

#[derive(Deserialize, Debug)]
pub struct SearchResult {
    pub title: String,
    pub snippet: String,
}
