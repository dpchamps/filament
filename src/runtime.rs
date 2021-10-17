use std::fmt::Error;

use rusty_v8 as v8;
use std::panic::resume_unwind;

fn says_hello(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut _retval: v8::ReturnValue,
) {
    println!("Hello From Native")
}

pub fn run_shallow_scope(script: &str) -> Result<String, Error> {
    // Initialize V8.
    let platform = v8::new_default_platform().take().unwrap();
    v8::V8::initialize_platform(platform);
    v8::V8::initialize();

    let result = {
        // Create a new Isolate and make it the current one.
        let isolate = &mut v8::Isolate::new(v8::CreateParams::default());
        let isolate_scope = &mut v8::HandleScope::new(isolate);

        let global_object = v8::ObjectTemplate::new(isolate_scope);
        global_object.set(
            v8::String::new(isolate_scope, "saysHello").unwrap().into(),
            v8::FunctionTemplate::new(isolate_scope, says_hello).into()
        );

        // Create a new context.
        let context = v8::Context::new_from_template(isolate_scope, global_object);
        // Enter the context for compiling and running the hello world script.
        let ctx_scope = &mut v8::ContextScope::new(isolate_scope, context);

        // Create a string containing the JavaScript source code.
        let code = v8::String::new(ctx_scope, script).unwrap();
        // Compile the source code.
        let script = v8::Script::compile(ctx_scope, code, None).unwrap();
        // Run the script to get the result.
        let result = script.run(ctx_scope).unwrap();

        // Convert the result to a string and print it.
        result.to_string(ctx_scope).unwrap().to_rust_string_lossy(ctx_scope)
    };

    unsafe {
        v8::V8::dispose();
    }

    v8::V8::shutdown_platform();

    return Ok(result)
}