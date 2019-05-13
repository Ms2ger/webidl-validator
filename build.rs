use std::env;
use std::fs::{self, File};
use std::io;
use std::io::Write;
use std::path::Path;

fn escape(path: &Path) -> Option<String> {
    let file_name = path.file_stem()?.to_str()?;
    Some(file_name.replace("-", "_"))
}

fn write(path: &Path) -> io::Result<()> {
    let mut file = File::create(&path)?;
    for kind in &["valid", "invalid"] {
        let path = format!("tests/wpt/WebIDL/{}/idl", kind);
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let test_path = entry.path();
            let name = escape(&test_path).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidData, "Bad path")
            })?;
            let test = format!("
    #[test]
    fn parse_{}_idl_{}() -> io::Result<()> {{
        parse_{}_idl(\"{}\")
    }}
    ", kind, name, kind, test_path.to_str().unwrap());
            file.write_all(test.as_bytes())?;
        }
    }
    Ok(())
}

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("wpt.rs");
    write(&dest_path).unwrap();
}
