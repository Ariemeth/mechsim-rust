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
            for entity in self.entities.iter_mut() {
                println!("Cycling entity {}", entity.name());
                entity.cycle();
            }
        }
    }
}

pub trait Entity {
    fn cycle(&mut self);
    fn name(&self) -> String;
    fn print_status(&self);
}

impl Debug for dyn Entity + 'static {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(can cycle)")
    }
}

#[derive(Debug)]
pub struct Position {
    x: u32,
    y: u32,
}

impl Position {
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }

    pub fn x(&self) -> u32 {
        self.x
    }
    pub fn y(&self) -> u32 {
        self.y
    }

    pub fn set_x(&mut self, new_value: u32) {
        self.x = new_value
    }
    pub fn set_y(&mut self, new_value: u32) {
        self.y = new_value
    }

    pub fn update(&mut self, velocity: &Velocity) {
        self.x += velocity.x();
        self.y += velocity.y();
    }
}

#[derive(Debug)]
pub struct Velocity {
    x: u32,
    y: u32,
}

impl Velocity {
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }

    pub fn set_x(&mut self, new_value: u32) {
        self.x = new_value
    }
    pub fn set_y(&mut self, new_value: u32) {
        self.y = new_value
    }

    pub fn x(&self) -> u32 {
        self.x
    }
    pub fn y(&self) -> u32 {
        self.y
    }
}
