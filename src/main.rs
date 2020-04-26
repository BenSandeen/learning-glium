#[macro_use]
use glium;

mod tutorial_01;
mod tutorial_02;
mod tutorial_03;

fn main() {
    // It looks like we can only have one of these uncommented and running at a time, otherwise only the first one will
    // actually work :/

    // tutorial_01::tutorial_01::tutorial_01();
    // tutorial_02::tutorial_02::tutorial_02();
    tutorial_03::tutorial_03::tutorial_03();
}
