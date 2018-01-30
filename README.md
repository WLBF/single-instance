single-instance
===

A rust library for single instance application.

[![Build Status](https://travis-ci.org/WLBF/single-instance.svg?branch=master)](https://travis-ci.org/WLBF/single-instance)
[![Build status](https://ci.appveyor.com/api/projects/status/cnmbkhqso04577sr/branch/master?svg=true)](https://ci.appveyor.com/project/WLBF/single-instance)

single-instance provides a single API to check if there are any other running instance. 

## Usage
On Windows, init `SingleInstance` will create a *Mutex* named by given `&str` then check error code by calling `GetLastError`. On Linux, init will create or open a file which path is given `&str`, then call `flock` to apply an advisory lock on the open file.

```toml
[dependencies]
single-instance = "0.1"
```

### Examples
```rust
extern crate single_instance;

use std::thread;
use single_instance::SingleInstance;

fn main() {
    let instance = SingleInstance::new("whatever").unwrap();
    let is_single = instance.is_single();
    assert!(is_single);

    loop {
        thread::park();
    }
}
```
