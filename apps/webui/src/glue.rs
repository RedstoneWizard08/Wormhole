use anyhow::Result;
use glue::{client, config::GlueConfig, framework::Framework, glue::Glue};

client!("$CARGO_MANIFEST_DIR/../../build");

pub fn make_glue() -> Result<Glue> {
    Ok(Glue::new(
        GlueConfig::builder()
            .base("http://localhost:4001")
            .dir(client::CLIENT_DIR)
            .project(format!("{}/../..", env!("CARGO_MANIFEST_DIR")))
            .cmd("pnpm")
            .arg("run")
            .arg("web:dev")
            .framework(Framework::Vite("/vite-hmr"))
            .build()?,
    ))
}
