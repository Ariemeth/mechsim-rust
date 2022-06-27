use std::str::FromStr;

mod mech;
mod world;

use mech::Mech;

fn main() {
    println!("Starting Sim!");
    let mut new_world = world::World::new(
        128,
        128,
        vec![
            Box::new(Mech::new(String::from_str("mech1").unwrap())),
            Box::new(Mech::new(String::from_str("mech2").unwrap())),
            Box::new(Mech::new(String::from_str("mech3").unwrap())),
        ],
    );
    new_world.run_cycle(Some(2));
    println!("{}", new_world)
}
