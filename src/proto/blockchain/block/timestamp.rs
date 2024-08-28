use std::ops::Deref;

use serde::{Deserialize, Serialize};

use crate::node::webui::ServerResult;

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct BlockTimestamp(u32);

impl Deref for BlockTimestamp {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl BlockTimestamp {
    pub(crate) fn now() -> ServerResult<Self> {
        use std::time::SystemTime;
        let now = SystemTime::now();
        let now_unix_epoch = now.duration_since(SystemTime::UNIX_EPOCH)?;

        Ok(Self(now_unix_epoch.as_secs().try_into()?))
    }
}
