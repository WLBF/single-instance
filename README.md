single-instance
===

[![Crates.io](https://img.shields.io/crates/v/single-instance.svg)](https://crates.io/crates/single-instance)
[![Build Status](https://travis-ci.org/WLBF/single-instance.svg?branch=master)](https://travis-ci.org/WLBF/single-instance)

single-instance provides an API to check if there are any other running instances of the same process.

## Detail
On Windows, `SingleInstance` attempts to create a named mutex and checks if it already exists.
On POSIX platforms it creates or opens a file with a given path, then attempts to apply an advisory lock on the opened file.

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
    }
}
```
