use std::{
    collections::HashMap, env, io::Cursor, process::Command, str::FromStr, sync::Arc,
    time::Duration,
};

use anyhow::{anyhow, Result};
use envcrypt::envc;
use once_cell::sync::Lazy;
use reqwest::Client;
use tiny_http::{Header, Response, Server, StatusCode};
use url::Url;

use crate::msa_token::MsaTokenResponse;

/// The port for MSA auth redirect.
/// This will be for the local server at http://localhost:[port]/callback
/// This was determined by combining the ASCII codes for W (87) and H (72)
pub const MSA_REDIRECT_PORT: u32 = 8772;
// pub const MSA_REDIRECT_PORT: u32 = 4002;

/// Wormhole's client ID for MSA.
pub const MSA_CLIENT_ID: &str = "61f104b9-3b1c-49bb-b0b8-2bb1f42f581c";

/// Wormhole's client secret for MSA.
const MSA_CLIENT_SECRET: Lazy<&str> = Lazy::new(|| envc!("MSA_CLIENT_SECRET"));

/// The auth callback URL.
pub const CALLBACK_URL: &str =
    const_format::formatcp!("http://localhost:{}/callback", MSA_REDIRECT_PORT);
// pub const CALLBACK_URL: &str = "https://dev-websocket.kadaroja.com/callback";

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
        ("redirect_uri", CALLBACK_URL),
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

    let thread = tokio::spawn(async move {
        tokio::time::sleep(Duration::from_millis(AUTH_TIMEOUT)).await;

        unsafe { STOP = true };

        server_clone.unblock();
        drop(server_clone);
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

                    unsafe { STOP = true };
                    thread.abort();
                    server.unblock();

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

    unsafe { STOP = true };
    thread.abort();
    server.unblock();

    Err(anyhow!("Could not authenticate!"))
}

pub async fn get_auth_token(code: impl AsRef<str>) -> Result<String> {
    let res = Client::new()
        .post("https://login.microsoftonline.com/consumers/oauth2/v2.0/token")
        .form::<HashMap<&str, &str>>(&HashMap::from_iter(vec![
            ("code", code.as_ref()),
            ("grant_type", "authorization_code"),
            ("client_secret", &MSA_CLIENT_SECRET.clone()),
            ("client_id", MSA_CLIENT_ID),
            ("scope", "XboxLive.signin"),
            ("redirect_uri", CALLBACK_URL),
        ]))
        .send()
        .await?
        .json::<MsaTokenResponse>()
        .await?;

    Ok(res.access_token)
}

#[tokio::test]
pub async fn test() -> Result<()> {
    let tkn = get_auth_token(msa_code()?).await?;

    dbg!(tkn);

    Ok(())
}
