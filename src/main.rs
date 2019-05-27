#[macro_use] extern crate log;
extern crate env_logger;

use log::{info, trace, warn};
use log::Level;

fn main() {

    env_logger::init();

    println!("Hello, world!");

    iterator(10, 5000000);
}


fn iterator (levels: u32, items: u32) {

    for level in 1..levels {
        println!("Hello {}", level);
        info!("Started with vector {}...", level);


        let mut list = vec![];

        for i in 1..items {
            list.push(i);
        }

        info!("Finished with vector {}", level)

    }

}