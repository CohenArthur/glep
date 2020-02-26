use std::process;

fn print_help() {
    println!(
    "glep: Simple Line Pattern matching
    Usage:
       glep file pattern line [lines]

       file        filename or '-' for stdin
       pattern     regular expression pattern to match
       line        first line where the pattern shall be matched
       lines       subsequent lines where the matched pattern shall also match

    Example:

        glep - \"*.cc\" 1 2 3

        will return successfully if:
            - The pattern *.cc is matched on line 1
            - The words that matched the pattern are also present on line 2 and 3"
    );
}

pub struct GlepArgs
{
    pub filename: String,
    pub pattern: String,
    pub first_line: u32,
}

pub fn get_args(cli_args: &Vec<String>) -> GlepArgs {
    if cli_args.len() < 4 {
        print_help();
        process::exit(1);
    }

    let args = GlepArgs {
        filename : cli_args[1].clone(),
        pattern : cli_args[2].clone(),
        first_line : cli_args[3].parse::<u32>().unwrap(),
    };

    return args;
}