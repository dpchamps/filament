Filament is an experimental javascript runtime.

It uses the [rusty_v8 bindings](https://docs.rs/rusty_v8/0.32.0/rusty_v8/) created for Deno.

Project Goals:

1. provide resumable stackful coroutines in javascript
2. show an effect

### Running examples

Examples are implemented as independent binaries in [src/bin](src/bin).

Use `cargo run --bin example-<n>`

### Roadmap

- [x] Run arbitrary javascript with v8 bindings
- [x] Call a native function from within JS
- [x] Pass data to and from native function  
- [ ] Perform an async operation: shove something on the EL
- [ ] Create a stackfull continuation
- [ ] Implement a yieldable fiber that interops with EL
- [ ] Demonstrate an effect