use clap::*;
use std::io;

fn main() {
    let app = app_from_crate!()
        .arg(Arg::from_usage("[NUM] 'print the first NUM lines'").default_value("10"));
    let matches = app.get_matches();

    let num = matches.value_of("NUM").unwrap().parse::<u32>();
    let num = match num {
        Ok(n) => n,
        Err(_) => {
            println!("error: NUM must be an integer");
            std::process::exit(1);
        }
    };

    for _ in 0..num {
        let mut line = String::new();
        match io::stdin().read_line(&mut line) {
            Ok(0) => break,
            Ok(_) => print!("{}", line),
            Err(err) => {
                println!("error: {}", err);
                break;
            }
        }
    }
}
