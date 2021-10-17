mod runtime;
use rusty_v8 as v8;
use std::env;
use std::fs;
use std::path::{Path};

fn main() {
    let args : Vec<String> = env::args().collect();
    let filepath = Path::new(
        env::current_dir().unwrap().to_str().unwrap()
    ).join(&args[1]);
    let file_contents = fs::read_to_string(filepath).unwrap();

    let mut platform = runtime::V8Platform::new();
    let mut isolate_scope = platform.isolate_scope();
    let mut runtime = runtime::JsRuntime::new(&mut isolate_scope);

    runtime.run_script(&mut isolate_scope, &file_contents).unwrap();
}
