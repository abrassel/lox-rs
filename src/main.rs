use std::{io, path::PathBuf};

use lox_rs::{exec, exec_repl};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    script: Option<PathBuf>,
}

fn main() -> io::Result<()> {
    let opt: Opt = Opt::from_args();
    match opt.script {
        Some(script) => {
            let string = std::fs::read_to_string(script)?;
            exec(string);
        }
        None => {
            exec_repl()?;
        }
    };

    Ok(())
}
