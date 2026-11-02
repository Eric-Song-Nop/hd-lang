//! Writes the host glue of a module `hd_web` built, for a page that binds a
//! registered package's host traits: `host_glue IN.wasm OUT.mjs`. Registered
//! identities are fixed, so one generated file serves every compile.

use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let [input, output] = args.as_slice() else {
        eprintln!("usage: host_glue IN.wasm OUT.mjs");
        return ExitCode::FAILURE;
    };
    let result = std::fs::read(input)
        .map_err(|e| e.to_string())
        .and_then(|wasm| Ok(hd_driver::host_artifacts(&wasm)?))
        .and_then(|artifacts| artifacts.ok_or_else(|| "the module has no host traits".to_owned()))
        .and_then(|artifacts| {
            std::fs::write(output, artifacts.javascript).map_err(|e| e.to_string())
        });
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(why) => {
            eprintln!("host_glue: {why}");
            ExitCode::FAILURE
        }
    }
}
