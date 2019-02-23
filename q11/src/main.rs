use std::io;

fn main() {
    loop {
        let mut line = String::new();
        match io::stdin().read_line(&mut line) {
            Ok(0) => break,
            Ok(_) => {
                print!("{}", line.replace("\t", " "));
            }
            Err(err) => {
                println!("error: {}", err);
                break;
            }
        }
    }
}
