use std::{ error::Error, io::stdin };

fn main() -> Result<(), Box<dyn Error>> {
    let input = stdin().lines().map(Result::unwrap);

    Ok(())
}
