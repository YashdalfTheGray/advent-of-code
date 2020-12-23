#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::process;

use code_line::CodeLine;
use code_processor::CodeProcessor;

mod code_line;
mod code_processor;
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

    let mut processor = CodeProcessor::new();
    processor.execute(code_lines, false);
}
