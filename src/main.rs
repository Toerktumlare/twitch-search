use crate::{
    api::{
        auth::fetch_token,
        streams::{search, Entry},
    },
    utils::print,
};
use std::{env, process::exit};

mod api;
mod utils;

const IGNORED_STREAMERS: &'static [&str] = &["Toerktumlare"];

fn main() {
    let search_term = match env::args().skip(1).next() {
        Some(s) => s.to_lowercase(),
        None => "".to_string(),
    };

    let client_id = env::var("TWITCH_CLIENT_ID").unwrap_or_else(|_| {
        eprint!("TWITCH_CLIENT_ID env variable not set");
        exit(1);
    });

    let client_secret = env::var("TWITCH_CLIENT_SECRET").unwrap_or_else(|_| {
        eprint!("TWITCH_CLIENT_SECRET env variable not set");
        exit(1);
    });

    println!("\u{1F50D} Searching twitch streams for: {}", search_term);

    let mut total_entries = 0;
    let mut found = 0;

    let token = fetch_token(&client_id, &client_secret).unwrap_or_else(|e| {
        eprintln!("Error while fetching token, cause: {}", e);
        exit(1);
    });

    let mut page = None;

    let mut results: Vec<Entry> = Vec::new();
    loop {
        let (entries, page_id) = search(&client_id, &token, page).unwrap_or_else(|e| {
            eprintln!("Failed to fetch search, cause: {}", e);
            exit(1);
        });

        page = page_id;
        total_entries += entries.len();

        for entry in entries
            .into_iter()
            .filter(|e| {
                if IGNORED_STREAMERS.contains(&e.user_name.as_str()) {
                    return false;
                }

                if e.title.to_lowercase().contains(&search_term) {
                    true
                } else {
                    false
                }
            })
            .collect::<Vec<_>>()
        {
            results.push(entry);
            found += 1;
        }
        if page.is_none() {
            break;
        }
    }

    println!("");
    results.iter().for_each(|result| print(&result));
    println!("");

    println!("Done ({}/{}) \u{2764}", found, total_entries);
}
