# oxide-api

Public API for [Oxide Game Engine](https://github.com/Enix50/oxide-engine-api).

This crate provides the traits and interfaces needed to write plugins for Oxide Game Engine.

## Example

```rust
use oxide_engine_api::*;

create_script!(MyScript);
struct MyScript{
	name: String
}

impl Script for MyScript {
	fn new() -> Self {
		name: String::from("MyScript")
	}
    fn init(&mut self, ctx: &dyn Context) {
        ctx.log("Hello from plugin!");
    }
	fn update(&mut self, ctx: &dyn Context, delta: f32) {
        ctx.log("Updated!");
	}
}