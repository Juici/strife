use async_trait::async_trait;

mod private {
    use crate::model::id::ChannelId;

    pub trait AsChannelId {
        fn channel_id(&self) -> ChannelId;
    }
}

/// A trait for models that can have [`Message`]s to them.
///
/// [`Message`]: ../struct.Message.html
#[async_trait]
pub trait Converse: private::AsChannelId {}
