use clap::*;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufRead;

fn main() {
    let app = app_from_crate!()
        .arg(Arg::from_usage("[NUM] 'split file into NUM pieces'").default_value("3"));
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
    let p = lines.len() / num;
    for i in 0..num {
        let filename = format!("xa{}", std::char::from_u32(0x61 + i as u32).unwrap());
        let mut f = io::BufWriter::new(File::create(filename).unwrap());
        let begin = p * i;
        let end = if i < num-1 { p * (i+1) } else { lines.len() };
        for l in &lines[begin..end] {
            writeln!(f, "{}", l).unwrap();
        }
    }
}
