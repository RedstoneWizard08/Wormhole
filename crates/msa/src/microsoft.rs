use std::{
    collections::HashMap, env, io::Cursor, process::Command, str::FromStr, sync::Arc, thread,
    time::Duration,
};

use anyhow::{anyhow, Result};
use envcrypt::envc;
use reqwest::Client;
use tiny_http::{Header, Response, Server, StatusCode};
use url::Url;

use crate::msa_token::MsaTokenResponse;

/// The port for MSA auth redirect.
/// This will be for the local server at http://localhost:[port]/callback
/// This was determined by combining the ASCII codes for W (87) and H (72)
// pub const MSA_REDIRECT_PORT: u32 = 8772;
// Used for testing.
pub const MSA_REDIRECT_PORT: u32 = 4000;

/// Wormhole's client ID for MSA.
pub const MSA_CLIENT_ID: &str = "61f104b9-3b1c-49bb-b0b8-2bb1f42f581c";

/// How much time the user has to complete an authentication flow.
/// This is 2 minutes (2 * 60 * 1000 ms).
pub const AUTH_TIMEOUT: u64 = 2 * 60 * 1000;

pub const AUTH_OK_HTML: &str = include_str!("./html/ok.html");
pub const AUTH_ERR_HTML: &str = include_str!("./html/err.html");
pub const AUTH_ERR_STATE_HTML: &str = include_str!("./html/err_state.html");

pub fn msa_code() -> Result<String> {
    let validation_state = "__TODO__";
    let mut url = Url::parse("https://login.microsoftonline.com/consumers/oauth2/v2.0/authorize")?;

    let params = &[
        ("client_id", MSA_CLIENT_ID),
        (
            "redirect_uri",
            // &format!("http://localhost:{}/callback", MSA_REDIRECT_PORT),
            "https://dev.kadaroja.com/callback",
        ),
        ("response_type", "code"),
        ("response_mode", "query"),
        ("scope", "XboxLive.signin"),
        ("state", validation_state),
    ];

    url.query_pairs_mut().extend_pairs(params);

    let server = Arc::new(Server::http(format!("0.0.0.0:{}", MSA_REDIRECT_PORT)).unwrap());
    let server_clone = Arc::clone(&server);
    static mut STOP: bool = false;

    if let Ok(path) = env::var("BROWSER") {
        Command::new(path).arg(url.as_str()).spawn()?.wait()?;
    } else {
        open::that(url.as_str())?;
    }

    thread::spawn(move || {
        thread::sleep(Duration::from_millis(AUTH_TIMEOUT));

        unsafe { STOP = true };
        server_clone.unblock();
    });

    loop {
        if unsafe { STOP } {
            break;
        }

        if let Ok(req) = server.recv() {
            let url = Url::parse(&format!("http://__unused__{}", req.url()))?;

            if url.path() == "/callback"
                && url
                    .query()
                    .map(|v| v.contains("code=") && v.contains("state="))
                    .unwrap_or(false)
            {
                let code = url
                    .query_pairs()
                    .find(|v| v.0 == "code")
                    .unwrap()
                    .1
                    .to_string();

                let state = url
                    .query_pairs()
                    .find(|v| v.0 == "state")
                    .unwrap()
                    .1
                    .to_string();

                if state == validation_state {
                    req.respond(Response::new(
                        StatusCode(200),
                        vec![Header::from_str("Content-Type: text/html").unwrap()],
                        Cursor::new(AUTH_OK_HTML),
                        None,
                        None,
                    ))?;

                    return Ok(code);
                }

                req.respond(Response::new(
                    StatusCode(500),
                    vec![Header::from_str("Content-Type: text/html").unwrap()],
                    Cursor::new(AUTH_ERR_STATE_HTML),
                    None,
                    None,
                ))?;
            } else {
                req.respond(Response::new(
                    StatusCode(500),
                    vec![Header::from_str("Content-Type: text/html").unwrap()],
                    Cursor::new(AUTH_ERR_HTML),
                    None,
                    None,
                ))?;
            }
        }
    }

    Err(anyhow!("Could not authenticate!"))
}

pub async fn get_auth_token(code: impl AsRef<str>) -> Result<String> {
    let secret: &str = envc!("MSA_CLIENT_SECRET");
    let client = Client::new();

    let res = client
        .post("https://login.microsoftonline.com/consumers/oauth2/v2.0/token")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .form::<HashMap<&str, &str>>(&HashMap::from_iter(vec![
            ("code", code.as_ref()),
            ("grant_type", "authorization_code"),
            ("client_secret", secret),
            ("client_id", MSA_CLIENT_ID),
            ("scope", "XboxLive.signin"),
            (
                "redirect_uri",
                // &format!("http://localhost:{}/callback", MSA_REDIRECT_PORT),
                "https://dev.kadaroja.com/callback",
            ),
        ]))
        .send()
        .await?
        .text()
        .await?;

    let res = serde_json::from_str::<MsaTokenResponse>(&res)?;

    Ok(res.access_token)
}
