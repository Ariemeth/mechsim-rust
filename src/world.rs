use core::fmt::Debug;
use std::fmt;

#[derive(Debug)]
pub struct World {
    pub width: u32,
    pub height: u32,
    entities: Vec<Box<dyn Entity>>,
}

impl fmt::Display for World {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "World: (width: {}, height: {}, Entity Count: {})",
            self.width,
            self.height,
            self.entities.len(),
        )
    }
}

impl World {
    pub fn new(width: u32, height: u32, entities: Vec<Box<dyn Entity>>) -> Self {
        Self {
            width,
            height,
            entities,
        }
    }

    pub fn run_cycle(&mut self, num_cycles: Option<u64>) {
        for i in 1..=num_cycles.unwrap_or(1) {
            println!("Running Cycle {i}");
            for entity in self.entities.iter() {
                println!("Cycling entity {}", entity.name());
                entity.cycle()
            }
        }
    }
}

pub trait Entity {
    fn cycle(&self);
    fn name(&self) -> String;
}

impl Debug for dyn Entity + 'static {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(can cycle)")
    }
}
