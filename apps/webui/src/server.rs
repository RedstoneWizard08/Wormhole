//! The server module.

use std::net::{IpAddr, SocketAddr};

use anyhow::Result;
use axum::serve;
use glue::{abort::register_exit_handler, util::is_debug};
use tokio::{join, net::TcpListener};

use crate::{cli::Cli, glue::make_glue, router::RouterBuilder};

/// Run the server.
pub async fn run_server(cli: Cli) -> Result<()> {
    info!("Setting up CTRL-C handler...");

    register_exit_handler()?;

    info!("Creating glue...");

    let glue = make_glue()?;

    info!("Creating router...");

    let router = RouterBuilder::new()
        .glue(glue.clone())
        .routes()
        .await
        .log()
        .build()
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
