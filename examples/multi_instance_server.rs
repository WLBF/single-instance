extern crate single_instance;
use std::{thread::sleep, time::Duration};
use single_instance::SingleInstance;

static UNIQ_ID : &'static str = "multi_instance_server";
static SLEEP_SECS : u64 = 100;

/// Run in one terminal (this should be the first instance of this program) :
///     cargo run --example multi_instance_server  
/// 
/// Run in another terminal(this should fail, provieded above program is still running) :
///     cargo run --example multi_instance_server  
fn main() { 
    let instance =Box::new(SingleInstance::new(UNIQ_ID).unwrap());
    
    println!("server is single: {}\n", instance.is_single());
    
    if instance.is_single() {
        Box::leak(instance);
     
        println!("Sleeping for secs:{}, press ^C for exit.
Run another instance of the same process to see the single instance is false(within sleep seconds)", SLEEP_SECS);

        sleep(Duration::from_secs(SLEEP_SECS));
        // once is sleep is over, other proces can claim the single instance
    }
}
