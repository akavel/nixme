use std::error::Error;
use std::io::{Read, Write};

pub fn serve(mut stream: &(impl Read + Write)) -> Result<(), Box<dyn Error>> {
    Ok(())
}
