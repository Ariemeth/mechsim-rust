use std::fmt;

use crate::world::{Entity, Position, Velocity};

#[derive(Debug)]
pub struct Mech {
    name: String,
    location: Position,
    velocity: Velocity,
}

impl fmt::Display for Mech {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(name:{})", self.name)
    }
}

impl Entity for Mech {
    fn cycle(&mut self) {
        self.location.update(&self.velocity);
        self.print_status();
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn print_status(&self) {
        println!(
            "{} @ {}:{} with velocity {}:{} cycled!",
            self.name,
            self.location.x(),
            self.location.y(),
            self.velocity.x(),
            self.velocity.y(),
        )
    }
}

impl Mech {
    pub fn new(name: String, location: Option<Position>, velocity: Option<Velocity>) -> Self {
        Self {
            name,
            location: match location {
                Some(pos) => pos,
                None => Position::new(0, 0),
            },
            velocity: match velocity {
                Some(value) => value,
                None => Velocity::new(0, 0),
            },
        }
    }
}
