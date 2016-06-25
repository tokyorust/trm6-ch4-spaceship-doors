#![feature(plugin)]
#![plugin(clippy)]

#[macro_use]
extern crate log;
extern crate env_logger;

extern crate rand;
extern crate trm6_ch4_spaceship_doors;

use std::sync::Arc;
use std::thread;
use std::time::Duration;

use trm6_ch4_spaceship_doors::airlock::{Controller, Door};
use trm6_ch4_spaceship_doors::airlock::Error;

struct Operator<'a> {
    door: &'a mut Door,
}

impl<'a> Operator<'a> {
    fn new(door: &'a mut Door) -> Operator {
        Operator { door: door }
    }

    fn operate(&mut self) {
        if rand::random() {
            match self.door.open() {
                Ok(_) => info!("Opened {}", self.door),
                Err(Error::UnchangedState) => {}
                Err(err) => error!("Unable to open {}: {}", self.door, err),
            }
        } else {
            match self.door.close() {
                Ok(_) => info!("Closed {}", self.door),
                Err(Error::UnchangedState) => {}
                Err(err) => error!("Unable to close {}: {}", self.door, err),
            }
        }
    }
}

fn main() {
    env_logger::init().unwrap();

    let controller = Arc::new(Controller::new());

    let mut inner_door = Door::new("inner door", controller.clone());
    let mut outer_door = Door::new("outer door", controller.clone());

    let mut threads = Vec::new();

    threads.push(thread::spawn(move || {
        let mut operator = Operator::new(&mut inner_door);
        loop {
            operator.operate();
            thread::sleep(Duration::from_millis(50));
        }
    }));

    threads.push(thread::spawn(move || {
        let mut operator = Operator::new(&mut outer_door);
        loop {
            operator.operate();
            thread::sleep(Duration::from_millis(50));
        }
    }));

    for t in threads {
        t.join().unwrap();
    }
}
