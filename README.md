# oxide-api

Public API for [Oxide Game Engine](https://github.com/Enix50/oxide-engine-api).

This crate provides the traits and interfaces needed to write plugins for Oxide.

## Example

```rust
use oxide_api::{Script, WorldContext};

struct MyScript;

impl Script for MyScript {
    fn init(&mut self, ctx: &dyn WorldContext) {
        ctx.log("Hello from plugin!");
    }
}

#[no_mangle]
pub extern "Rust" fn create_script() -> Box<dyn oxide_api::Script> {
    Box::new(MyScript)
}