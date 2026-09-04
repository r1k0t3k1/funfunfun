
#[async_trait::async_trait]
pub trait Listener: Send + Sync {
    fn start(&mut self) -> anyhow::Result<()>;
    async fn stop(&mut self) -> anyhow::Result<()>;
}

