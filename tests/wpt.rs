extern crate webidl_validator;

use std::fs;
use std::io::{self, Read};

use webidl_validator::validate;

#[test]
fn parse_invalid_idl() -> io::Result<()> {
    let path = "tests/wpt/WebIDL/invalid/idl";
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let mut file = fs::File::open(entry.path())?;
        let mut webidl = String::new();
        file.read_to_string(&mut webidl)?;
        let result = validate(&*webidl);
        if let Ok(ref r) = result {
            println!("{:#?}", r);
        }
        assert!(result.is_err(), "{:?}", entry.path());
    }
    Ok(())
}


#[test]
fn parse_valid_idl() -> io::Result<()> {
    let path = "tests/wpt/WebIDL/valid/idl";
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let mut file = fs::File::open(entry.path())?;
        let mut webidl = String::new();
        file.read_to_string(&mut webidl)?;
        let result = validate(&*webidl);
        if let Err(ref r) = result {
            println!("{:#?}", r);
        }
        assert!(result.is_ok(), "{:?}", entry.path());
    }
    Ok(())
}


