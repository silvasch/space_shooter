use anyhow::Result;
use graphics_engine::GraphicsEngine;

pub async fn run() -> Result<()> {
    let engine = GraphicsEngine::new().await?;

    Ok(())
}
