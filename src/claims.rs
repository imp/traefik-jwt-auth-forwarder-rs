use std::time;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub(super) struct Claims {
    sub: uuid::Uuid,
    jti: uuid::Uuid,
    exp: u64,
    scope: String,
}

impl Claims {
    pub(super) fn validate(self) -> Result<Self, ()> {
        if self.expired() {
            return Err(());
        }

        if self.jti_revoked() {
            return Err(());
        }

        Ok(self)
    }

    pub(super) fn into_headers(self) -> [(&'static str, String); 4] {
        [
            ("x-jwt-subject", self.sub.to_string()),
            ("x-jwt-jti", self.jti.to_string()),
            ("x-jwt-expiry", self.exp.to_string()),
            ("x-jwt-scope", self.scope),
        ]
    }

    fn expired(&self) -> bool {
        let expiration = time::SystemTime::UNIX_EPOCH + time::Duration::from_secs(self.exp);
        time::SystemTime::now() > expiration
    }

    fn jti_revoked(&self) -> bool {
        // FIXME - need to check against revocation list
        self.jti.is_nil()
    }
}
