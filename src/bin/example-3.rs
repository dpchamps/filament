use filament::runtime;
use rusty_v8 as v8;
use std::fs;
use rusty_v8::Local;

fn main(){
    let file_contents = fs::read_to_string("./src/bin/native-fns.js").unwrap();

    let mut platform = runtime::V8Platform::new();
    let mut isolate_scope = platform.isolate_scope();
    let mut runtime = runtime::JsRuntime::new(&mut isolate_scope);

    runtime.extend_global(&mut isolate_scope, "passCallback", pass_callback);
    runtime.extend_global(&mut isolate_scope, "callCbFromNative", call_callback_from_native);

    runtime.run_script(&mut isolate_scope, &file_contents).unwrap();
}

fn pass_callback(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut retval: v8::ReturnValue,
) {
    let arg1 = args.get(0);

    if !arg1.is_function() {
        let exn_str = v8::String::new(scope, "passCallback expects a string").unwrap();
        let exn = v8::Exception::error(scope, exn_str);
        scope.throw_exception(exn);
        return
    }

    retval.set(arg1);
}

fn call_callback_from_native(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut retval: v8::ReturnValue,
) {
    let arg1 = args.get(0);

    if !arg1.is_function() {
        let exn_str = v8::String::new(scope, "passCallback expects a string").unwrap();
        let exn = v8::Exception::error(scope, exn_str);
        scope.throw_exception(exn);
        return
    }

    let undefined = v8::undefined(scope);
    let down_cast: v8::Local<v8::Function> = unsafe {Local::cast(arg1)};
    let return_val = down_cast.call(scope, undefined.into(), &[]).unwrap();

    retval.set(return_val);
}


// TODO: Revisit this
// let arg1_ptr = Box::new(RefCell::new(arg1)).as_ptr() as *mut c_void;
// let arg1_ref = v8::External::new(scope, arg1_ptr);
// println!("Arg: {:?}", arg1);
// println!("Ptr: {:?}", arg1_ptr);
// unsafe{println!("Ptr Deref: {:?}", *(arg1_ptr as *mut Local<Value>))};
// println!("Ref: {:?}", arg1_ref);
//
// let ret_fn: FunctionBuilder<Function>  = v8::FunctionBuilder::new(|inner_scope: &mut v8::HandleScope, inner_args: v8::FunctionCallbackArguments, mut inner_return: v8::ReturnValue|{
// let external: Local<External> = Local::try_from(inner_args.data().unwrap()).unwrap();
// let undefined = v8::undefined(inner_scope);
// println!("External: {:?}", external);
// println!("Value: {:?}", external.value());
// unsafe { println!("Deref: {:?}", *(external.value() as * mut Local<Value>) ); }
// let pass_through = unsafe {*(external.value() as * mut Local<Value>)};
// inner_return.set(pass_through);
// })
// .data(arg1_ref.into());