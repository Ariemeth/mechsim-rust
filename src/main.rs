use std::str::FromStr;

mod mech;
mod world;

use mech::Mech;

use crate::world::{Position, Velocity};

fn main() {
    println!("Starting Sim!");
    let mut new_world = world::World::new(
        128,
        128,
        vec![
            Box::new(Mech::new(String::from_str("mech1").unwrap(), None, None)),
            Box::new(Mech::new(
                String::from_str("mech2").unwrap(),
                Some(Position::new(1, 1)),
                None,
            )),
            Box::new(Mech::new(
                String::from_str("mech3").unwrap(),
                None,
                Some(Velocity::new(1, 0)),
            )),
        ],
    );
    new_world.run_cycle(Some(2));
    println!("{}", new_world)
}
