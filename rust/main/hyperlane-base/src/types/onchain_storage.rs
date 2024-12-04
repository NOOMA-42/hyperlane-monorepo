use crate::{AgentMetadata, CheckpointSyncer};
use async_trait::async_trait;
use derive_new::new;
use eyre::Result;
use hyperlane_core::{OnchainCheckpointStorage, SignedAnnouncement, SignedCheckpointWithMessageId};
use std::fmt;

#[derive(new)]
/// Onchain storage client
pub struct OnchainStorageClient {
    storage: Box<dyn OnchainCheckpointStorage>,
}

// required by `CheckpointSyncer`
impl fmt::Debug for OnchainStorageClient {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("OnchainStorage")
            .field("storage", &self.storage)
            .finish()
    }
}

#[async_trait]
impl CheckpointSyncer for OnchainStorageClient {
    async fn latest_index(&self) -> Result<Option<u32>> {
        todo!() // self.fetch_latest_index().await
    }

    async fn write_latest_index(&self, index: u32) -> Result<()> {
        self.write_latest_index(index).await
    }

    async fn fetch_checkpoint(&self, index: u32) -> Result<Option<SignedCheckpointWithMessageId>> {
        self.fetch_checkpoint(index).await
    }

    async fn write_checkpoint(
        &self,
        signed_checkpoint: &SignedCheckpointWithMessageId,
    ) -> Result<()> {
        self.write_checkpoint(signed_checkpoint).await
    }

    async fn write_metadata(&self, metadata: &AgentMetadata) -> Result<()> {
        self.write_metadata(metadata).await
    }

    async fn write_announcement(&self, signed_announcement: &SignedAnnouncement) -> Result<()> {
        self.write_announcement(signed_announcement).await
    }

    fn announcement_location(&self) -> String {
        self.storage.announcement_location()
    }
}
