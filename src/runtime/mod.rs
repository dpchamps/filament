use rusty_v8 as v8;
mod core;

pub struct JsRuntime<'a, 'b> {
    global_object: v8::Local<'a, v8::ObjectTemplate>,
    context: v8::Local<'a, v8::Context>,
    context_scope: v8::ContextScope<'b, v8::HandleScope<'a>>,
}

impl<'a, 'b> JsRuntime<'a, 'b>
{
    pub fn new(isolate_scope: &'b mut v8::HandleScope<'a, ()>) -> Self {
        let global_object = v8::ObjectTemplate::new(isolate_scope);

        // Every instance gets logInfo
        global_object.set(
            v8::String::new(isolate_scope, "logInfo").unwrap().into(),
            v8::FunctionTemplate::new(isolate_scope, core::log_info).into()
        );

        // Create a new context.
        let context = v8::Context::new_from_template(isolate_scope, global_object);
        let context_scope = v8::ContextScope::new(isolate_scope, context);


        JsRuntime {
            context,
            context_scope,
            global_object
        }
    }

    pub fn run_script(&mut self, script: &str) -> Result<(), &'static str> {
        // Create a string containing the JavaScript source code.
        let code = v8::String::new(&mut self.context_scope, script).unwrap();
        // Compile the source code.
        let script = v8::Script::compile(&mut self.context_scope, code, None).unwrap();

        match script.run(&mut self.context_scope) {
            Some(_) => Ok(()),
            None => {
                Err("Failed to run script")
            }
        }
    }

    pub fn extend_global(&mut self, fn_name: &str, op: impl v8::MapFnTo<v8::FunctionCallback>){
        // todo this is wrong and doesn't work
        self.global_object.set(
            v8::String::new(&mut self.context_scope, fn_name).unwrap().into(),
            v8::FunctionTemplate::new(&mut self.context_scope, op).into()
        )
    }
}