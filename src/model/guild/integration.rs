//! Models related to guild integrations.

use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use crate::model::id::{IntegrationId, RoleId};
use crate::model::user::User;

/// A guild integration with partial information.
#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PartialIntegration {
    /// The ID of the integration.
    pub id: IntegrationId,
    /// The name of the integration.
    pub name: String,
    /// The type of the integration.
    // TODO: Improve model.
    #[serde(rename = "type")]
    pub kind: String,
    /// The account information for the integration.
    pub account: IntegrationAccount,
}

/// A guild integration.
#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Integration {
    integration: PartialIntegration,
    /// Whether the integration is enabled.
    pub enabled: bool,
    /// Whether the integration is syncing.
    pub syncing: bool,
    /// The ID of the role the integration uses for subscribers.
    pub role_id: RoleId,
    /// The behavior for expiring subscribers.
    // TODO: Improve model.
    expire_behavior: i64,
    /// The grace period before expiring subscribers.
    // TODO: Improve model.
    expire_grace_period: i64,
    /// The user for the integration.
    pub user: User,
    /// When the integration was last synced.
    pub synced_at: DateTime<FixedOffset>,
}
wrap!(Integration => mut integration: PartialIntegration);

/// Account information for an [`Integration`].
///
/// [`Integration`]: ./struct.Integration.html
#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IntegrationAccount {
    /// The ID of the account.
    pub id: String,
    /// The name of the account.
    pub name: String,
}

impl_eq_fields!(IntegrationAccount: [id, name]);
impl_eq_fields!(PartialIntegration: [id, name, kind, account]);
impl_eq_fields!(Integration: [
    integration,
    enabled,
    syncing,
    role_id,
    expire_behavior,
    expire_grace_period,
    user,
    synced_at
]);

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn test_deserialize_partial() {
        let value = json!({
          "id": "33590653072239123",
          "name": "A Name",
          "type": "twitch",
          "account": {
            "name": "twitchusername",
            "id": "1234567"
          }
        });
        let integration = PartialIntegration {
            id: IntegrationId::from(33590653072239123),
            name: "A Name".to_owned(),
            kind: "twitch".to_owned(),
            account: IntegrationAccount {
                id: "1234567".to_owned(),
                name: "twitchusername".to_owned(),
            },
        };

        let deserialize = PartialIntegration::deserialize(&value).unwrap();
        assert_eq_fields!(integration, deserialize);
    }

    #[test]
    fn test_serialize_partial() {
        let value = json!({
          "id": "33590653072239123",
          "name": "A Name",
          "type": "twitch",
          "account": {
            "name": "twitchusername",
            "id": "1234567"
          }
        });
        let integration = PartialIntegration {
            id: IntegrationId::from(33590653072239123),
            name: "A Name".to_owned(),
            kind: "twitch".to_owned(),
            account: IntegrationAccount {
                id: "1234567".to_owned(),
                name: "twitchusername".to_owned(),
            },
        };

        assert_eq!(value, serde_json::to_value(&integration).unwrap());
    }
}
