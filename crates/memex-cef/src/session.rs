use cef::Client;

use crate::cef_impl::ClientService;

pub struct BrowserSession {
    pub(crate) client: Client,
}

impl Default for BrowserSession {
    fn default() -> Self {
        Self {
            client: ClientService::create(),
        }
    }
}
