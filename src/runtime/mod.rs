use rusty_v8 as v8;
use std::borrow::BorrowMut;
use std::cell::{RefCell, Cell, Ref};

mod core;

pub struct V8Platform {
    platform: v8::UniquePtr<v8::Platform>,
    isolate: v8::OwnedIsolate,
}

impl V8Platform {
    pub fn new() -> Self {
        let mut platform = v8::new_default_platform();
        v8::V8::initialize_platform(platform.take().unwrap());
        v8::V8::initialize();
        let mut isolate = v8::Isolate::new(v8::CreateParams::default());

        V8Platform {
            platform,
            isolate,
        }
    }

    pub fn isolate_scope(&mut self) -> v8::HandleScope<()> {
        v8::HandleScope::new(&mut self.isolate)
    }
}

// impl Drop for V8Platform {
//     fn drop(&mut self) {
//         unsafe {
//             v8::V8::dispose();
//         }
//         v8::V8::shutdown_platform();
//     }
// }

pub struct JsRuntime<'a> {
    global_object: v8::Local<'a, v8::ObjectTemplate>,
}

impl<'a> JsRuntime<'a>
{
    pub fn new(isolate_scope: &mut v8::HandleScope<'a, ()>) -> Self {
        let global_object = v8::ObjectTemplate::new(isolate_scope);

        // Every instance gets logInfo
        global_object.set(
            v8::String::new(isolate_scope, "logInfo").unwrap().into(),
            v8::FunctionTemplate::new(isolate_scope, core::log_info).into()
        );

        JsRuntime {
            global_object,
        }
    }

    pub fn run_script(&mut self, isolate_scope: &mut v8::HandleScope<'a, ()>, script: &str) -> Result<(), &'static str> {
        let context = v8::Context::new_from_template(isolate_scope, self.global_object);
        let mut context_scope = v8::ContextScope::new(isolate_scope, context);
        let code = v8::String::new(&mut context_scope, script).unwrap();
        let script = v8::Script::compile(&mut context_scope, code, None).unwrap();

        match script.run(&mut context_scope) {
            Some(_) => Ok(()),
            None => {
                Err("Failed to run script")
            }
        }
    }

    pub fn extend_global(&mut self, isolate_scope: & mut v8::HandleScope<'a, ()>, fn_name: &str, op: impl v8::MapFnTo<v8::FunctionCallback>){
        self.global_object.set(
            v8::String::new(isolate_scope, fn_name).unwrap().into(),
            v8::FunctionTemplate::new(isolate_scope, op).into()
        )
    }
}