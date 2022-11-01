use crate::modules::race;

mod domain;
mod modules;

fn main() {
    let r = race::controller::handle_get();
    println!("{:?}", r);
}
