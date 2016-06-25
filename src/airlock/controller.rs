use std::sync::Mutex;

use airlock::{Result, Error, Door};

pub struct Controller {
    counter: Mutex<u32>,
}

impl Controller {
    pub fn new() -> Controller {
        Controller { counter: Mutex::new(0) }
    }

    pub fn open(&self, door: &mut Door) -> Result<()> {
        let mut counter = self.counter.lock().unwrap();

        if door.open {
            return Err(Error::UnchangedState);
        }

        if *counter > 0 {
            return Err(Error::IllegalAction);
        }

        *counter += 1;
        door.open = true;

        Ok(())
    }

    pub fn close(&self, door: &mut Door) -> Result<()> {
        let mut counter = self.counter.lock().unwrap();

        if !door.open {
            return Err(Error::UnchangedState);
        }

        *counter -= 1;
        door.open = false;

        Ok(())
    }
}
