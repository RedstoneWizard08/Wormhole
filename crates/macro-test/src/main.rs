use std::marker::PhantomData;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppState<'a> {
    unused: PhantomData<&'a str>,
}

#[macros::serde_call]
pub async fn something(_state: AppState<'_>) {}

#[macros::serde_call]
#[allow(unused)]
pub async fn something_two(a: String, b: i32, c: String, _state: AppState<'_>) -> bool {
    false
}

macros::serde_funcs!(something, something_two);

#[tokio::main]
pub async fn main() {
    // println!("{}", __serde_invoke_something_two("{\"a\": \"\", \"b\": 1, \"c\": \"\"}".into()));
    let val = handle_serde_call(
        "something_two",
        "{\"a\": \"\", \"b\": 1, \"c\": \"\"}",
        AppState::default(),
    )
    .await;

    println!("{}", val);
}
