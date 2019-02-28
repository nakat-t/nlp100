use clap::*;
use std::io::BufRead;

fn main() {
    let app = app_from_crate!()
        .arg(Arg::from_usage("[NUM] 'print the tail NUM lines'").default_value("10"));
    let matches = app.get_matches();

    let num = matches.value_of("NUM").unwrap();
    let num = match num.parse::<usize>() {
        Ok(n) => n,
        Err(_) => {
            println!("error: NUM must be an integer");
            std::process::exit(1);
        }
    };

    let stdin = std::io::stdin();
    let handle = stdin.lock();
    let lines: Vec<String> = handle.lines().map(|l| l.unwrap()).collect();
    for line in lines.iter().skip(lines.len() - num) {
        println!("{}", line);
    }
}
