use std::fmt::Error;

use rusty_v8 as v8;
use std::panic::resume_unwind;

pub fn run_shallow_scope(script: &str) -> Result<String, Error> {
    // Initialize V8.
    let platform = v8::new_default_platform().take().unwrap();
    v8::V8::initialize_platform(platform);
    v8::V8::initialize();

    let result = {
        // Create a new Isolate and make it the current one.
        let isolate = &mut v8::Isolate::new(v8::CreateParams::default());

        // Create a stack-allocated handle scope.
        let handle_scope = &mut v8::HandleScope::new(isolate);

        // Create a new context.
        let context = v8::Context::new(handle_scope);

        // Enter the context for compiling and running the hello world script.
        let scope = &mut v8::ContextScope::new(handle_scope, context);

        // Create a string containing the JavaScript source code.
        let code = v8::String::new(scope, script).unwrap();

        // Compile the source code.
        let script = v8::Script::compile(scope, code, None).unwrap();
        // Run the script to get the result.
        let result = script.run(scope).unwrap();

        // Convert the result to a string and print it.
        result.to_string(scope).unwrap().to_rust_string_lossy(scope)
    };

    unsafe {
        v8::V8::dispose();
    }

    v8::V8::shutdown_platform();

    return Ok(result)
}