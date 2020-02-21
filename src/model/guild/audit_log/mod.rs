//! Models related to guild audit logs.

mod action_type;
mod change;

use serde::{Deserialize, Serialize};

use crate::model::guild::integration::PartialIntegration;
use crate::model::id::{AuditLogEntryId, UserId};
use crate::model::user::User;
use crate::model::webhook::Webhook;

pub use self::action_type::AuditLogEvent;
use crate::model::guild::audit_log::change::AuditLogChanges;
// pub use self::change::AuditLogChange;

/// A log of moderation/administrative actions made in a [`Guild`].
///
/// [`Guild`]: ../struct.Guild.html
#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AuditLog {
    /// The webhooks found in the audit log.
    pub webhooks: Vec<Webhook>,
    /// The users found in the audit log.
    pub users: Vec<User>,
    /// The audit log entries.
    pub audit_log_entries: Vec<AuditLogEntry>,
    /// The integrations in the audit log, with partial information.
    pub integrations: Vec<PartialIntegration>,
}

/// An entry in an [`AuditLog`].
///
/// [`AuditLog`]: ./struct.AuditLog.html
#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AuditLogEntry {
    /// The ID of the entry.
    pub id: AuditLogEntryId,
    /// The ID of the affected entity.
    // TODO: Improve model.
    pub target_id: Option<String>,
    /// The ID of the user that performed the action.
    pub user_id: UserId,
    /// The changes made.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub changes: Option<AuditLogChanges>,
    /// The type of action that occurred.
    pub action_type: AuditLogEvent,
    // /// Additional information.
    // #[serde(default, skip_serializing_if = "Option::is_none")]
    // pub options: Option<AuditLogEntryInfo>,
    /// The reason for the change.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}
