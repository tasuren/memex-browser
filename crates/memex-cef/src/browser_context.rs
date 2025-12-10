use std::rc::Rc;

use uuid::Uuid;

use crate::EventHandler;

pub type BrowserId = Uuid;

#[derive(Clone)]
pub struct BrowserContext {
    pub(crate) id: BrowserId,
    event_handler: Rc<dyn EventHandler>,
}

impl BrowserContext {
    pub fn new(event_handler: impl EventHandler + 'static) -> Self {
        Self {
            id: Uuid::new_v4(),
            event_handler: Rc::new(event_handler),
        }
    }

    pub fn id(&self) -> BrowserId {
        self.id
    }

    pub fn event_handler(&self) -> &dyn EventHandler {
        &*self.event_handler
    }
}
