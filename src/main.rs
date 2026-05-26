use llrt_modules::{
    assert::AssertModule,
    fs::FsModule,
    os::OsModule,
    path::PathModule,
    url::{init, UrlModule},

        child_process::ChildProcessModule,
};
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
        .with_module("fs", FsModule)
        .with_module("os", OsModule)
        .with_module("child-process", ChildProcessModule)
        .with_module("assert", AssertModule),);

    let resolver = (BuiltinResolver::default()
        .with_module("path")
        .with_module("url")
        .with_module("fs")
        .with_module("assert")
        .with_module("child-process")
        .with_module("os"),);
    runtime.set_loader(resolver, loader);

    context.with(|ctx| {
        let global = ctx.globals();

        init(&ctx).unwrap();

        global
            .set("print", Function::new(ctx.clone(), print))
            .unwrap();

        let name = "osvg.js";
        let code = r#"
import os from 'os'
print(typeof os.EOL)

import url from 'url'
print(typeof url)

import path from 'path'
print(typeof path)
const cwd = path.resolve(".")
print(cwd)

import fs from 'fs'
print(typeof fs)

const p = path.join(cwd, 'Cargo.toml')
print(fs.readFileSync(p, 'utf-8'))

import {   spawn } from 'child-process'

// print(execSync('echo hello world').toString())

spawn('echo', ['hello world'] )

"#;
        Module::evaluate(ctx.clone(), name, code)
            .unwrap()
            .finish::<Value>()
            .unwrap();
    });
}
