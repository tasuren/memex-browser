use cef::Client;
use uuid::Uuid;

use crate::cef_impl::ClientService;

pub type SessionId = Uuid;

pub struct BrowserSession {
    id: SessionId,
    pub(crate) client: Client,
}

impl Default for BrowserSession {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            client: ClientService::create(),
        }
    }
}

impl BrowserSession {
    pub fn id(&self) -> SessionId {
        self.id
    }
}
