use std::error::Error;

type AuthResult<T> = Result<T, Box<dyn Error>>;

const AUTH_URL: &'static str = "https://id.twitch.tv/oauth2/token";

pub fn fetch_token(client_id: &str, client_secret: &str) -> AuthResult<String> {
    let auth_url = format!(
        "{}?client_id={}&client_secret={}&grant_type=client_credentials",
        AUTH_URL, client_id, client_secret
    );

    let resp = ureq::post(&auth_url)
        .set("client_id", &client_id)
        .set("client_secret", &client_secret)
        .call()?;

    let json: serde_json::Value = resp.into_json()?;

    Ok(json["access_token"].as_str().unwrap().to_string())
}
