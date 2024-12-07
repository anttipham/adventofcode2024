use std::{ error::Error, io::stdin };

fn main() -> Result<(), Box<dyn Error>> {
    for line in stdin().lines().map(Result::unwrap) {
        if line.is_empty() {
            break;
        }
    }

    Ok(())
}
