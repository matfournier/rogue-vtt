mod domain;
mod middleware;
mod routes;
mod server;
mod services;
mod util;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    server::serve().await?;
    Ok(())
}
