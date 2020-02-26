use std::fs::File;
use std::io::{self, BufRead};
use regex::Regex;

use crate::g_args::GlepArgs;

fn get_lines(args: &GlepArgs) -> Vec<String> {
    let lines: Vec<String> = if args.is_stdin {
        let input = io::stdin();
        input
            .lock()
            .lines()
            .map(|l| l.expect("Couldn't read from stdin"))
            .collect()
    } else {
        let file = File::open(&args.filename).unwrap();
        io::BufReader::new(file)
            .lines()
            .map(|l| l.expect("Couldn't read file line"))
            .collect()
    };

    lines
}

pub fn is_full_match(args: GlepArgs) -> bool {
    let lines = get_lines(&args);

    let re = Regex::new(&args.pattern).unwrap();

    let captures = re.captures(&lines[args.lines[0]]).unwrap();

    dbg!(captures);

    return true;
}
