single-instance
===

A rust library for single instance application.

[![Crates.io](https://img.shields.io/crates/v/single-instance.svg)](https://crates.io/crates/single-instance)
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
