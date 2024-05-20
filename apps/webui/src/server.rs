// MIT License
//
// Copyright (c) 2024 Jacob Sapoznikow <jacob1coder@gmail.com>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE. 

use std::net::{IpAddr, SocketAddr};

use anyhow::Result;
use axum::serve;
use data::{
    diesel::r2d2::{ConnectionManager, Pool},
    Conn,
};
use glue::{abort::register_exit_handler, util::is_debug};
use tokio::{join, net::TcpListener};

use crate::{cli::Cli, glue::make_glue, router::RouterBuilder, state::AppState};

pub async fn run_server(pool: Pool<ConnectionManager<Conn>>, cli: Cli) -> Result<()> {
    info!("Setting up CTRL-C handler...");

    register_exit_handler()?;

    info!("Creating glue...");

    let glue = make_glue()?;

    info!("Creating state...");

    let state = AppState::new(cli.clone(), pool).await?;

    info!("Creating router...");

    let router = RouterBuilder::new()
        .glue(glue.clone())
        .routes()
        .log()
        .build(state)
        .into_make_service_with_connect_info::<SocketAddr>();

    info!("Initializing server...");

    let ip: IpAddr = cli.host.parse()?;
    let addr = SocketAddr::from((ip, cli.port));
    let listener = TcpListener::bind(&addr).await?;

    info!("Listening on {}:{}!", cli.host, cli.port);

    let server = tokio::spawn(async move { serve(listener, router).await });

    if is_debug() {
        info!("Starting client...");

        let client = glue.spawn().await;
        let (a, b) = join!(client, server);

        a?;
        b??;

        return Ok(());
    }

    server.await??;

    Ok(())
}
