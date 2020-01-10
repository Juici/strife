use async_trait::async_trait;

use crate::model::channel::ChannelType;
use crate::model::id::ChannelId;

mod private {
    use crate::model::channel::guild::{
        Category, GuildChannel, NewsChannel, StoreChannel, TextChannel, VoiceChannel,
    };
    use crate::model::channel::{Channel, DMChannel, Group};

    macro_rules! impl_sealed {
        [$($T:ident),* $(,)*] => {$(
            impl Sealed for $T {}
        )*};
    }

    /// Private trait to prevent downstream implementations of [`Converse`].
    ///
    /// [`Converse`]: ../trait.Converse.html
    pub trait Sealed {}

    impl_sealed![
        Channel,
        DMChannel,
        Group,
        GuildChannel,
        TextChannel,
        VoiceChannel,
        Category,
        NewsChannel,
        StoreChannel,
    ];
}

/// A trait for models that can have [`Message`]s to them.
///
/// [`Message`]: ../struct.Message.html
#[async_trait]
pub trait Converse: Send + Sync + private::Sealed {
    /// Returns the ID of the channel in which to send messages.
    async fn channel_id(&self) -> ChannelId;

    /// Returns the type of the channel in which to send messages.
    fn channel_type(&self) -> ChannelType;
}
