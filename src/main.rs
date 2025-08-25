mod domain;
mod routes;
mod server;
mod util;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    server::serve().await?;
    Ok(())
}
