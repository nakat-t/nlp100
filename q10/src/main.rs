use std::io;

fn main() {
    let mut line_count = 0;
    loop {
        let mut line = String::new();
        match io::stdin().read_line(&mut line) {
            Ok(0) => break,
            Ok(_) => line_count += 1,
            Err(err) => {
                println!("error: {}", err);
                break;
            }
        }
    }
    println!("{}", line_count);
}
