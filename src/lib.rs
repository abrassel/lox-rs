#![feature(try_blocks)]
#![feature(option_result_contains)]

use std::io::{self, Write};

use crate::scanning::scan_tokens;

pub mod scanning;

pub fn exec(source: String) {
    let tokens = scan_tokens(&source);
    print!("{:?}", tokens.collect::<Vec<_>>());
}

pub fn exec_repl() -> io::Result<()> {
    fn print_prompt() -> io::Result<()> {
        print!("> ");
        std::io::stdout().flush()
    }
    let mut string = String::new();
    print_prompt()?;
    while io::stdin().read_line(&mut string)? != 0 {
        let tokens = scan_tokens(&string);
        println!("{:?}", tokens.collect::<Vec<_>>());
        string.clear();
        print_prompt()?;
    }

    Ok(())
}
