Filament is an experimental javascript runtime.

It uses the [rusty_v8 bindings](https://docs.rs/rusty_v8/0.32.0/rusty_v8/) created for Deno.
The goal is two fold:

1. provide resumable stackful coroutines on an event loop
2. implement an effect system

Roadmap

- [x] Run arbitrary javascript with v8 bindings
- [ ] Perform an async operation: shove something on the EL
- [ ] Create a stackfull continuation
- [ ] Implement a yieldable fiber that interops with EL
- [ ] Demonstrate an effect