use anyhow::Result;

pub struct GraphicsEngine {}

impl GraphicsEngine {
    pub async fn new() -> Result<Self> {
        Ok(GraphicsEngine {})
    }
}
