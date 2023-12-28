single-instance
===

[![Crates.io](https://img.shields.io/crates/v/single-instance.svg)](https://crates.io/crates/single-instance)
[![Build Status](https://travis-ci.org/WLBF/single-instance.svg?branch=master)](https://travis-ci.org/WLBF/single-instance)

single-instance provides a single API to check if there are any other running instance. 

## Detail
On windows, init `SingleInstance` will create a mutex named by given `&str` then check error code by calling `GetLastError`. On linux init will bind abstract unix domain socket with given name . On macos, init will create or open a file which path is given `&str`, then call `flock` to apply an advisory lock on the open file.

```toml
[dependencies]
single-instance = "0.3"
```

### Examples
```rust
extern crate single_instance;

use single_instance::SingleInstance;

fn main() {
    {
        let instance_a = SingleInstance::new("whatever").unwrap();
        assert!(instance_a.is_single());
        let instance_b = SingleInstance::new("whatever").unwrap();
        assert!(!instance_b.is_single());
    }
    let instance_c = SingleInstance::new("whatever").unwrap();
    assert!(instance_c.is_single());
}
```

### Ensuring the SingleInstance stays during lifetime of the process
Users should ensure that instance of SingleInstace should lives in lifetime of the calling process. 
This could be achieved in multiple ways. Few ways are:
 * Using std::sync::Once
```rust
static mut SINGLE_INSTANCE_VAL : Option<SingleInstance> = None;
static SINGLE_INSTANCE_VAL_LOCK: Once = Once::new();

pub fn ensure_single_instance(uniq_id: &str) -> bool {
    let instance =  SingleInstance::new(&uniq_id);
    match  instance{
        Ok(inst) => {
            let single = inst.is_single();
            if single {
                unsafe {
                    SINGLE_INSTANCE_VAL_LOCK.call_once(|| {
                        SINGLE_INSTANCE_VAL = Some(inst);
                    })
                }
            }
            single
        },
        Err(e) => {
            false
        }
    }
}

```
 * Using Box::leak. This example is in examples/multi_instance_server.rs
 ```rust
pub fn ensure_single_instance(uniq_id: &str) -> bool {
    let instance = Box::new(SingleInstance::new(uniq_id).unwrap());
    if instance.is_single() {
        Box::leak(instance);
        true
    }else {
        false
    }
}
 ```

