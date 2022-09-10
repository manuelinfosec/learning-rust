/*
This piece fetches the environment variable from the current process.
*/

use std::env;

fn main() {
    // declare environment variable key
    let key = "PATH";

    // check for the returning variants
    match env::var(key) {
        // check for the `Ok` variant
        Ok(val) => {
            println!("{key}: {val}")
        }

        // if key doesn't exist
        Err(e) => {
            println!("{key}: {e}")
        }
    }
}
