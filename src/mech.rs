use std::fmt;

use crate::world::Entity;

#[derive(Debug)]
pub struct Mech {
    name: String,
}

impl fmt::Display for Mech {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(name:{})", self.name)
    }
}

impl Entity for Mech {
    fn cycle(&self) {
        println!("{} cycled!", self.name)
    }

    fn name(&self) -> String {
        self.name.clone()
    }
}

impl Mech {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}
