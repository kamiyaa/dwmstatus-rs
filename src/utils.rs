use std::{fs, io::{self, Read}, path};

pub fn read_to_usize(p: &path::Path) -> io::Result<usize> {
    let mut file = fs::File::open(p)?;

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Ok(_) => {
            let stripped = s.trim();
            match stripped.parse::<usize>() {
                Ok(u) => Ok(u),
                Err(_) => Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Failed to parse value".to_string(),
                )),
            }
        }
        Err(_) => Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Failed to read value".to_string(),
        )),
    }
}
