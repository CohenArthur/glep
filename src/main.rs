use std::env;

mod g_args;
mod g_match;

fn main() {
    let cli_args: Vec<String> = env::args().collect();
    let args = g_args::get_args(&cli_args);

    dbg!(g_match::is_full_match(args));
}
