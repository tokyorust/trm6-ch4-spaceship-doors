use std::sync::Mutex;

use airlock::{Result, Error, Door, Config};

pub struct Controller {
    config: Config,
    counter: Mutex<u8>,
}

impl Controller {
    pub fn new(config: Config) -> Controller {
        Controller {
            config: config,
            counter: Mutex::new(0),
        }
    }

    pub fn open(&self, door: &mut Door) -> Result<()> {
        let mut counter = self.counter.lock().unwrap();

        if door.open {
            return Err(Error::UnchangedState);
        }

        if *counter >= self.config.max_open {
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
