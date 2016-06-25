use std::fmt;
use std::sync::Arc;

use airlock::{Result, Controller};

pub struct Door {
    name: String,
    controller: Arc<Controller>,
    pub open: bool,
}

impl Door {
    pub fn new<S: Into<String>>(name: S, controller: Arc<Controller>) -> Door {
        Door {
            name: name.into(),
            controller: controller,
            open: false,
        }
    }

    pub fn open(&mut self) -> Result<()> {
        self.controller.clone().open(self)
    }

    pub fn close(&mut self) -> Result<()> {
        self.controller.clone().close(self)
    }
}

impl fmt::Display for Door {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}
