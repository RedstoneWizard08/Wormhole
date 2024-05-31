//! GitHub Device Flow implementation

use std::{thread, time};

use chrono::{offset::Utc, DateTime, Duration};
use reqwest::{Body, Client};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Wormhole's Client ID
pub const WORMHOLE_CLIENT_ID: &str = "d59635fddac868079384";

/// How many iterations of the polling loop
pub const POLL_ITERATIONS: i32 = 20;

/// Five seconds
pub const FIVE_SECONDS: time::Duration = time::Duration::new(5, 0);

/// Poll request data
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PollData {
    /// The client id
    pub client_id: String,

    /// The device code
    pub device_code: String,

    /// The grant type
    pub grant_type: String,
}

/// Setup request data
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SetupData {
    /// The client id
    pub client_id: String,

    /// The scope(s)
    pub scope: String,
}

/// A device flow
#[derive(Debug, Clone)]
pub struct DeviceFlow {
    /// The device code
    pub device_code: Option<String>,

    /// The client id
    pub client_id: String,

    /// The user code
    pub user_code: Option<String>,

    /// The verification uri
    pub verification_uri: Option<String>,

    /// The state
    pub state: State,
}

/// A credential
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Credential {
    /// The access token
    pub token: String,

    /// When the token will expire
    pub expiry: String,

    /// The refresh token
    pub refresh_token: String,
}

impl Credential {
    fn empty() -> Credential {
        Credential {
            token: String::new(),
            expiry: String::new(),
            refresh_token: String::new(),
        }
    }

    /// Check if the token is expired
    pub fn is_expired(&self) -> bool {
        let exp = match DateTime::parse_from_rfc3339(self.expiry.as_str()) {
            Ok(time) => time,
            Err(_) => return false,
        };
        let now = Utc::now();
        now > exp
    }
}

/// The state
#[derive(Debug, Clone)]
pub enum State {
    /// Processing
    Processing(time::Duration),

    /// Failure
    Failure(String),

    /// Success
    Success(Credential),

    /// Pending
    Pending,
}

/// Calculate the expiry time
pub fn calculate_expiry(expires_in: i64) -> String {
    let expires_in = Duration::try_seconds(expires_in).unwrap();
    let mut expiry: DateTime<Utc> = Utc::now();

    expiry += expires_in;
    expiry.to_rfc3339()
}

/// An error
#[derive(Debug, Clone)]
pub enum FlowError {
    /// Max iterations reached
    MaxIterationsReached,

    /// Unable to fetch
    UnableToFetch,

    /// Other
    Other(String),
}

impl DeviceFlow {
    /// Create a new device flow
    pub fn new(client_id: String) -> Self {
        Self {
            client_id,
            user_code: None,
            device_code: None,
            verification_uri: None,
            state: State::Pending,
        }
    }

    /// Start the device flow
    pub async fn start(client_id: String) -> Result<DeviceFlow, FlowError> {
        let mut flow = DeviceFlow::new(client_id);

        flow.setup().await;

        match flow.state {
            State::Processing(_) => Ok(flow),
            State::Failure(err) => Err(FlowError::Other(err)),
            _ => Err(FlowError::Other(
                "Something truly unexpected happened".to_string(),
            )),
        }
    }

    /// Setup the device flow
    pub async fn setup(&mut self) {
        let client = Client::new();

        let data = SetupData {
            client_id: self.client_id.clone(),
            scope: "".to_string(),
        };

        let data = serde_json::to_string(&data).unwrap();

        let resp = client
            .post("https://github.com/login/device/code")
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .body(Body::from(data))
            .send()
            .await;

        if let Ok(resp) = resp {
            let text = resp.json::<Value>().await.unwrap();
            let text = text.as_object().unwrap();

            if text.contains_key("error") && text.contains_key("error_description") {
                self.state = State::Failure(text["error_description"].as_str().unwrap().into());
            } else if text.contains_key("error") {
                self.state = State::Failure(format!(
                    "Error response: {:?}",
                    text["error"].as_str().unwrap()
                ));
            } else {
                self.user_code = Some(String::from(text["user_code"].as_str().unwrap()));
                self.device_code = Some(String::from(text["device_code"].as_str().unwrap()));
                self.verification_uri =
                    Some(String::from(text["verification_uri"].as_str().unwrap()));
                self.state = State::Processing(FIVE_SECONDS);
            }
        }
    }

    /// Poll the device flow
    pub async fn poll(&mut self) -> Result<Credential, FlowError> {
        for i in 0..POLL_ITERATIONS {
            self.update().await;

            if let State::Processing(interval) = self.state {
                if i == POLL_ITERATIONS {
                    return Err(FlowError::MaxIterationsReached);
                }

                thread::sleep(interval);
            } else {
                break;
            }
        }

        match &self.state {
            State::Success(cred) => Ok(cred.clone()),
            State::Failure(err) => Err(FlowError::Other(err.to_string())),
            _ => Err(FlowError::UnableToFetch),
        }
    }

    /// Update the device flow
    pub async fn update(&mut self) {
        let client = Client::new();

        let data = PollData {
            client_id: self.client_id.clone(),
            device_code: self.device_code.clone().unwrap(),
            grant_type: "urn:ietf:params:oauth:grant-type:device_code".to_string(),
        };

        let data = serde_json::to_string(&data).unwrap();

        let resp = client
            .post("https://github.com/login/oauth/access_token")
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .body(Body::from(data))
            .send()
            .await;

        if let Ok(resp) = resp {
            let text = resp.json::<Value>().await.unwrap();
            let text = text.as_object().unwrap();

            if text.contains_key("error") {
                match text["error"].as_str().unwrap() {
                    "authorization_pending" => {}
                    "slow_down" => {
                        if let State::Processing(interval) = self.state {
                            self.state = State::Processing(interval + FIVE_SECONDS);
                        }
                    }
                    other_reason => {
                        self.state =
                            State::Failure(format!("Error checking for token: {}", other_reason));
                    }
                };
            } else {
                let mut credential = Credential::empty();

                credential.token = text["access_token"].as_str().unwrap().to_string();

                if let Some(expires_in) = text.get("expires_in") {
                    credential.expiry = calculate_expiry(expires_in.as_i64().unwrap());
                    credential.refresh_token = text["refresh_token"].as_str().unwrap().to_string();
                }

                self.state = State::Success(credential);
            }
        }
    }
}
