use filament::runtime;
use rusty_v8 as v8;
use std::fs;

fn main(){
    let file_contents = fs::read_to_string("./src/bin/hello-world.js").unwrap();

    let mut platform = runtime::V8Platform::new();
    let mut isolate_scope = platform.isolate_scope();
    let mut runtime = runtime::JsRuntime::new(&mut isolate_scope);

    runtime.run_script(&mut isolate_scope, &file_contents).unwrap();
}