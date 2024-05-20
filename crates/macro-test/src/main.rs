// MIT License
//
// Copyright (c) 2024 Jacob Sapoznikow <jacob1coder@gmail.com>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE. 

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
