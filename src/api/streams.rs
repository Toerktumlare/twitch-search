use chrono::{DateTime, Utc};
use serde::Deserialize;
use std::error::Error;

type SearchResult<T, U> = Result<(T, U), Box<dyn Error>>;

const ROOT_URL: &'static str = "https://api.twitch.tv/helix/streams?first=100&game_id=1469308723";

#[derive(Deserialize, Debug)]
struct Response {
    data: Vec<Entry>,
    pagination: Pagination,
}

#[derive(Deserialize, Debug)]
struct Pagination {
    cursor: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Entry {
    pub user_name: String,
    pub game_name: String,
    pub title: String,
    pub viewer_count: u32,
    pub language: String,
    pub game_id: String,
    pub started_at: DateTime<Utc>,
}

pub fn search<'a>(
    client_id: &'a str,
    token: &'a str,
    page: Option<String>,
) -> SearchResult<Vec<Entry>, Option<String>> {
    let stream_url = match page {
        Some(page_id) => format!("{}&after={}", ROOT_URL, page_id),
        None => ROOT_URL.to_string(),
    };

    let auth_header = format!("Bearer {}", token);

    let resp: Response = ureq::get(&stream_url)
        .set("Authorization", &auth_header)
        .set("Client-Id", &client_id)
        .call()?
        .into_json()?;

    Ok((resp.data, resp.pagination.cursor))
}
