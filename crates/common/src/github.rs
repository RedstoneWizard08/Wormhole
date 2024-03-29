use std::{thread, time};

use chrono::{offset::Utc, DateTime, Duration};
use reqwest::{Body, Client};
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub const WORMHOLE_CLIENT_ID: &str = "d59635fddac868079384";
pub const POLL_ITERATIONS: i32 = 20;
pub const FIVE_SECONDS: time::Duration = time::Duration::new(5, 0);

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PollData {
    pub client_id: String,
    pub device_code: String,
    pub grant_type: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SetupData {
    pub client_id: String,
    pub scope: String,
}

#[derive(Debug, Clone)]
pub struct DeviceFlow {
    pub device_code: Option<String>,
    pub client_id: String,
    pub user_code: Option<String>,
    pub verification_uri: Option<String>,
    pub state: State,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Credential {
    pub token: String,
    pub expiry: String,
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

    pub fn is_expired(&self) -> bool {
        let exp = match DateTime::parse_from_rfc3339(self.expiry.as_str()) {
            Ok(time) => time,
            Err(_) => return false,
        };
        let now = Utc::now();
        now > exp
    }
}

#[derive(Debug, Clone)]
pub enum State {
    Processing(time::Duration),
    Failure(String),
    Success(Credential),
    Pending,
}

pub fn calculate_expiry(expires_in: i64) -> String {
    let expires_in = Duration::try_seconds(expires_in).unwrap();
    let mut expiry: DateTime<Utc> = Utc::now();

    expiry = expiry + expires_in;

    return expiry.to_rfc3339();
}

#[derive(Debug, Clone)]
pub enum FlowError {
    MaxIterationsReached,
    UnableToFetch,
    Other(String),
}

impl DeviceFlow {
    pub fn new(client_id: String) -> Self {
        return Self {
            client_id,
            user_code: None,
            device_code: None,
            verification_uri: None,
            state: State::Pending,
        };
    }

    pub async fn start(client_id: String) -> Result<DeviceFlow, FlowError> {
        let mut flow = DeviceFlow::new(client_id);

        flow.setup().await;

        return match flow.state {
            State::Processing(_) => Ok(flow),
            State::Failure(err) => Err(FlowError::Other(err)),
            _ => Err(FlowError::Other(
                "Something truly unexpected happened".to_string(),
            )),
        };
    }

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

        return match &self.state {
            State::Success(cred) => Ok(cred.clone()),
            State::Failure(err) => Err(FlowError::Other(err.to_string())),
            _ => Err(FlowError::UnableToFetch),
        };
    }

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
