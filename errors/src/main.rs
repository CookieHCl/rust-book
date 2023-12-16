use std::fs::File;
use std::io::{self, ErrorKind, Read};

fn main() {
    let greeting_file_result = File::open("hello.txt");

    // handling with match
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };

    // handling with unwrap
    let greeting_file = File::open("hello.txt").unwrap();
    let greeting_file = File::open("hello.txt").expect("Panic message!");

    // propagate errors
    fn read_username_from_file() -> Result<String, io::Error> {
        let username_file_result = File::open("hello.txt");

        let mut username_file = match username_file_result {
            Ok(file) => file,
            Err(e) => return Err(e),
        };

        let mut username = String::new();

        match username_file.read_to_string(&mut username) {
            Ok(_) => Ok(username),
            Err(e) => Err(e),
        }
    }

    // propagate errors with ?
    fn read_username_from_file2() -> Result<String, io::Error> {
        let mut username = String::new();

        let mut username_file = File::open("hello.txt")?;
        username_file.read_to_string(&mut username)?;

        // or oneline
        // File::open("hello.txt")?.read_to_string(&mut username)?;

        Ok(username)
    }

    // directly calling panic
    panic!("crash and burn");

    // from library
    let v = vec![1, 2, 3];
    v[99];
}

// main can return Result
// If error is returned, exit code will be non-zero

/*
use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    let greeting_file = File::open("hello.txt")?;

    Ok(())
}
*/
