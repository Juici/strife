use serde::Serialize;

use crate::http::unstable::Route;
use crate::model::guild::audit_log::AuditLogEvent;
use crate::model::id::{AuditLogEntryId, GuildId, UserId};

/// A builder for filtering audit logs.
#[derive(Debug, Serialize)]
pub struct AuditLogFilter {
    #[serde(skip_serializing_if = "Option::is_none")]
    user_id: Option<UserId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    action_type: Option<AuditLogEvent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    before: Option<AuditLogEntryId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u8>,
}

impl AuditLogFilter {
    pub(crate) fn new() -> Self {
        AuditLogFilter {
            user_id: None,
            action_type: None,
            before: None,
            limit: None,
        }
    }

    pub(crate) fn into_route(self, guild_id: GuildId) -> Route<'static> {
        let AuditLogFilter {
            user_id,
            action_type,
            before,
            limit,
        } = self;

        Route::GetAuditLogs {
            guild_id,
            user_id,
            action_type,
            before,
            limit,
        }
    }

    /// Sets the filter for actions made by the given user.
    pub fn user(&mut self, user_id: UserId) {
        self.user_id = Some(user_id);
    }

    /// Sets the filter for entries of the given type.
    pub fn action_type(&mut self, event: AuditLogEvent) {
        self.action_type = Some(event);
    }

    /// Sets the filter for entries before the given entry.
    pub fn before(&mut self, entry_id: AuditLogEntryId) {
        self.before = Some(entry_id);
    }

    /// Sets the filter to limit the number of entries returned.
    pub fn limit(&mut self, limit: u8) {
        self.limit = Some(limit);
    }
}
