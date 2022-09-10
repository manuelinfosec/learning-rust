use std::env;

fn main() {
    let key = "PATH";

    match env::var(key) {
        Ok(val) => {
            println!("{key}: {val}")
        }
        Err(e) => {
            println!("{key}: {e}")
        }
    }
}
