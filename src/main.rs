use llrt_modules::{os::OsModule, path::PathModule, url::UrlModule};
use rquickjs::{
    loader::{BuiltinResolver, ModuleLoader},
    Context, Function, Module, Runtime, Value,
};

fn print(s: String) {
    println!("{s}")
}

fn main() {
    let runtime = Runtime::new().unwrap();
    let context = Context::full(&runtime).unwrap();
    let loader = (ModuleLoader::default()
        .with_module("path", PathModule)
        .with_module("url", UrlModule)
        .with_module("os", OsModule),);
    let resolver = (
      BuiltinResolver::default()
      .with_module("path")
      .with_module("url")
      .with_module("os"),
    );
    runtime.set_loader(resolver, loader);

    context.with(|ctx| {
        let global = ctx.globals();
        global
            .set("print", Function::new(ctx.clone(), print))
            .unwrap();

        let name = "osvg.js";
        let code = r#"
// import os from 'os'
// print(typeof os.EOL)

// import url from 'url'
// print(typeof url)

import path from 'path'
print(typeof path)

"#;
        Module::evaluate(ctx.clone(), name, code)
            .unwrap()
            .finish::<Value>()
            .unwrap();
    });
}
