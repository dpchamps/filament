use filament::runtime;
use rusty_v8 as v8;
use std::fs;

fn main(){
    let file_contents = fs::read_to_string("./src/bin/hello-world.js").unwrap();
    let platform = v8::new_default_platform().take().unwrap();
    v8::V8::initialize_platform(platform);
    v8::V8::initialize();

    let mut isolate = v8::Isolate::new(v8::CreateParams::default());
    let mut isolate_scope = v8::HandleScope::new(&mut isolate);
    let mut rt = runtime::JsRuntime::new(&mut isolate_scope);

    rt.run_script(&file_contents).unwrap();
}