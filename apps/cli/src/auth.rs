use std::env;

use auth::github::{Credential, DeviceFlow, WORMHOLE_CLIENT_ID};

pub async fn setup_github_token() -> Credential {
    let mut flow = DeviceFlow::start(WORMHOLE_CLIENT_ID.to_string())
        .await
        .unwrap();

    println!(
        "Please go to: {} and enter code: {}",
        flow.verification_uri.as_ref().unwrap(),
        flow.user_code.as_ref().unwrap()
    );

    let polled = flow.poll().await.unwrap();
    let token = &polled.token;

    env::set_var("GITHUB_TOKEN", token);

    polled
}
