extern crate webidl_validator;

use std::fs;
use std::io::{self, Read};

use webidl_validator::validate;

fn parse_invalid_idl(path: &str) -> io::Result<()> {
    let mut file = fs::File::open(path)?;
    let mut webidl = String::new();
    file.read_to_string(&mut webidl)?;
    let result = validate(&*webidl);
    if let Ok(ref r) = result {
        println!("{:#?}", r);
    }
    assert!(result.is_err(), "{}", path);
    Ok(())
}

fn parse_valid_idl(path: &str) -> io::Result<()> {
    let mut file = fs::File::open(path)?;
    let mut webidl = String::new();
    file.read_to_string(&mut webidl)?;
    let result = validate(&*webidl);
    if let Err(ref r) = result {
        println!("{:#?}", r);
    }
    assert!(result.is_ok(), "{}", path);
    Ok(())
}

include!(concat!(env!("OUT_DIR"), "/wpt.rs"));
