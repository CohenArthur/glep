use std::env;

mod args;

fn main() {
    let cli_args: Vec<String>= env::args().collect();
    let args = args::get_args(&cli_args);

    dbg!(args.filename);
    dbg!(args.pattern);
    dbg!(args.first_line);
}
