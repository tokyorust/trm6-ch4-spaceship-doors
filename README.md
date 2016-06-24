# Challenge #4 - Spaceship doors

For [Tokyo Rust Meetup's mini-hackathon on 2016-06-23](http://www.meetup.com/Tokyo-Rust-Meetup/events/231555496/).

## Setup

For this task, any Rust nightly should be fine.

## The challenge

It's year 2045 and humankind has launched its first long range spaceship. Unfortunately, on its maiden voyage the ship mysteriously jettisons all its crew and the mission is lost. It turns out that there was a glaring oversight in the electronic airlock controls, allowing both the inner and outer door to be opened simultaneously if both panels were unlocked at the exact same time. Obviously this was a huge, preventable mistake and the entire development team was fired.

As the new Head Airlock Engineer, it is now your responsibility to design an airlock system that allows both doors to be operated in any order, excepting that they cannot be open at the same time.

You'll have two Operators (threads) operating independently, either closing or opening doors randomly. If two doors would end up being open simultaneously, the operation should not complete, but rather return a `Result` with an appropriate error.
