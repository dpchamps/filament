use rusty_v8 as v8;

pub fn log_info(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut _retval: v8::ReturnValue,
) {
    for i in 0..args.length(){
        let arg = args.get(i);
        match arg.to_string(scope) {
            Some(message) => {
                println!("{}", message.to_rust_string_lossy(scope))
            },
            _ => {
                println!("Failed to log arg: {:?}", arg);
            }
        }
    }
}