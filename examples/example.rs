extern crate single_instance;

use std::thread;
use single_instance::SingleInstance;

fn main() {
    let instance = SingleInstance::new("aa2d0258-ffe9-11e7-ba89-0ed5f89f718b").unwrap();
    println!("is single instance: {}", instance.is_single());

    loop {
        thread::park();
    }
}
