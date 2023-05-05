use std::fs::{self, File};
use std::io::{self, Read, ErrorKind};

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(error) => panic!("Problem creating the file: {:?}", error),
            },

            other_error => panic!("Problem opening the file: {:?}", other_error),
        }
    };

    // Using closures
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem with creating the file: {:?}", error);
            })
        } else {
            panic!("Problem with opening the file: {:?}", error);
        }
    });
}

// error propagation
fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
    // let mut s = String::new();
    // File::open("hello.txt")?.read_to_string(&mut s)?;
    // match f.read_to_string(&mut s) {
    //     Ok(_) => Ok(s),
    //     Err(e) => Err(e),
    // };

    // Ok(s)
}
