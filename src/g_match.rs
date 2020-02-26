use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};

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

pub fn is_full_match(mut args: GlepArgs) -> bool {
    let lines = get_lines(&args);

    let re = Regex::new(&args.pattern).unwrap();

    let captures = re.captures(&lines[args.lines[0]]).unwrap();

    args.lines.remove(0);

    for idx in args.lines {
        if !lines[idx].contains(&captures[0]) {
            return false;
        }
    }

    return true;
}
