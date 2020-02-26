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

pub struct GlepArgs {
    pub filename: String,
    pub is_stdin: bool,
    pub pattern: String,
    pub lines: Vec<usize>,
}

pub fn get_args(cli_args: &Vec<String>) -> GlepArgs {
    if cli_args.len() < 4 {
        print_help();
        process::exit(1);
    }

    let mut extra_lines = Vec::new();

    cli_args[3..].iter().for_each(|line| {
        extra_lines.push(line.parse::<usize>().unwrap());
        // FIXME: Add better error handling
    });

    let args = GlepArgs {
        filename: cli_args[1].clone(),
        is_stdin: if cli_args[1] == "-" { true } else { false },
        pattern: cli_args[2].clone(),
        lines: extra_lines,
    };

    return args;
}
