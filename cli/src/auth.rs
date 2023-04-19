use std::env;

use wormhole_common::github::{Credential, DeviceFlow, WORMHOLE_CLIENT_ID};

pub async fn setup_github_token() -> Credential {
    let mut flow = DeviceFlow::start(WORMHOLE_CLIENT_ID.to_string())
        .await
        .unwrap();

    println!(
        "Please go to: {} and enter code: {}",
        flow.clone().verification_uri.unwrap(),
        flow.clone().user_code.unwrap()
    );

    let polled = flow.poll().await.unwrap();
    let token = polled.clone().token;

    env::set_var("GITHUB_TOKEN", token);

    return polled;
}
