extern crate single_instance;

use single_instance::SingleInstance;

fn main() {
    {
        let instance_a = SingleInstance::new("aa2d0258-ffe9-11e7-ba89-0ed5f89f718b").unwrap();
        println!("instance a is single: {}", instance_a.is_single());
        let instance_b = SingleInstance::new("aa2d0258-ffe9-11e7-ba89-0ed5f89f718b").unwrap();
        println!("instance b is single: {}", instance_b.is_single());
    }
    let instance_c = SingleInstance::new("aa2d0258-ffe9-11e7-ba89-0ed5f89f718b").unwrap();
    println!("instance c is single: {}", instance_c.is_single());
}
