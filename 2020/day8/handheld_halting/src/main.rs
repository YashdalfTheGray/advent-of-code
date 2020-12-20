#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::process;

use code_line::CodeLine;

mod code_line;
mod utils;

fn main() {
    let default_filename = "input".to_string();

    let code_strs = utils::read_into_collection(&utils::get_filename().unwrap_or(default_filename))
        .unwrap_or_else(|err| {
            println!("{:?}", err);
            process::exit(1)
        });

    let code_lines = code_strs
        .iter()
        .map(|s| s.parse::<CodeLine>().unwrap())
        .collect::<Vec<CodeLine>>();

    for l in code_lines {
        println!("{:#?}", l);
    }
}
