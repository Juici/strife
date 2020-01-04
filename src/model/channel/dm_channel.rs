use serde::{Deserialize, Serialize};

/// A direct message channel between the [`ClientUser`] and another [`User`].
///
/// [`ClientUser`]: ../../user/struct.ClientUser.html
/// [`User`]: ../../user/struct.User.html
#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DMChannel {}
