use serde::{Deserialize, Serialize};

/// A group message channel between multiple [`User`]s.
///
/// [`User`]: ../../user/struct.User.html
#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GroupChannel {}
