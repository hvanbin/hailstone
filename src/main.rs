use clap::{Arg, Command};
use hailstone::{hail, unhail};
use std::process;

/// Prints a number of '*' characters to represent the value
fn put_hsval(val: i16) {
    for _ in 0..val {
        print!("*");
    }
}

/// Main execution logic
fn run(verbose: bool, start: i16, counter: i16, loop_mode: bool) -> i32 {
    let mut hsval = start;
    println!("counter: {}", counter);
    
    loop {
        for _ in 0..counter.abs() {
            hsval = if counter > 0 {
                hail(hsval)
            } else {
                unhail(hsval)
            };
            
            print!("{:02}", hsval);
            if verbose {
                print!("\t");
                put_hsval(hsval);
            }
            println!();
        }
        
        if !loop_mode {
            break;
        }
    }
    
    0
}

fn main() {
    let matches = Command::new("hailstone")
        .version("0.0.1")
        .about("Print hailstone sequence values")
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .help("Enable verbose output with visual representation")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("start")
                .short('s')
                .long("start")
                .value_name("NUMBER")
                .help("Starting value for the sequence")
                .default_value("1")
                .value_parser(clap::value_parser!(i16)),
        )
        .arg(
            Arg::new("counter")
                .short('n')
                .long("number")
                .value_name("COUNT")
                .help("Number of iterations to perform")
                .default_value("1")
                .value_parser(clap::value_parser!(i16)),
        )
        .arg(
            Arg::new("loop")
                .short('l')
                .long("loop")
                .help("Run in continuous loop mode")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    let verbose = matches.get_flag("verbose");
    let start = *matches.get_one::<i16>("start").unwrap();
    let counter = *matches.get_one::<i16>("counter").unwrap();
    let loop_mode = matches.get_flag("loop");

    let exit_code = run(verbose, start, counter, loop_mode);
    process::exit(exit_code);
}
