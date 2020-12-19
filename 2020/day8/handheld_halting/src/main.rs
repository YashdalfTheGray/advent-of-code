use std::process;

mod utils;

fn main() {
    let default_filename = "input".to_string();

    let code_lines =
        utils::read_into_collection(&utils::get_filename().unwrap_or(default_filename))
            .unwrap_or_else(|err| {
                println!("{:?}", err);
                process::exit(1)
            });

    for line in code_lines {
        println!("{}", line);
    }
}
