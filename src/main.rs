mod world;

use std::{thread, time};
use world::*;

fn main() {
    let mut world = World::new(60, 180);
    world.populate_random();
    for _i in 0..100 {
        world.step();
        world.print();
        thread::sleep(time::Duration::from_millis(1000));
    }
}
