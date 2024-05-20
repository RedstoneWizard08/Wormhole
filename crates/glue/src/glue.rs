// MIT License
//
// Copyright (c) 2024 Jacob Sapoznikow <jacob1coder@gmail.com>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE. 

use std::process::ExitStatus;

use axum::Router;
use tokio::{spawn, task::JoinHandle};

use crate::{
    abort::ABORT_HANDLES,
    config::GlueConfig,
    framework::Framework,
    router::{register_embedded, register_proxy},
    runner::start_client,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Glue {
    opts: GlueConfig,
}

impl Glue {
    pub fn new(opts: GlueConfig) -> Self {
        Self { opts }
    }

    pub fn register<T>(&self, router: Router<T>, use_proxy: bool) -> Router<T>
    where
        T: Clone + Send + Sync + 'static,
    {
        if use_proxy {
            register_proxy(
                self.opts.clone().base.unwrap(),
                router,
                self.opts.clone().framework,
            )
        } else {
            register_embedded(self.opts.clone().dir.unwrap(), router)
        }
    }

    pub async fn start(&self) -> ExitStatus {
        start_client(
            self.opts.project.clone().unwrap(),
            self.opts.cmd.clone(),
            self.opts.framework.unwrap_or(Framework::None),
        )
        .await
    }

    pub async fn spawn(&self) -> JoinHandle<ExitStatus> {
        let this = self.clone();
        let thread = spawn(async move { this.start().await });
        let handle = thread.abort_handle();

        ABORT_HANDLES.lock().unwrap().push(handle);

        thread
    }
}
