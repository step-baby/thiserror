use ref_cast::RefCast;
use std::fmt::Display;
use std::path::{Path, PathBuf};
use thiserror::EnumDisplay;

#[derive(EnumDisplay, Debug)]
#[display("failed to read '{file}'")]
struct StructPathBuf {
    file: PathBuf,
}

#[derive(EnumDisplay, Debug, RefCast)]
#[repr(C)]
#[display("failed to read '{file}'")]
struct StructPath {
    file: Path,
}

#[derive(EnumDisplay, Debug)]
enum EnumPathBuf {
    #[display("failed to read '{0}'")]
    Read(PathBuf),
}

fn assert<T: Display>(expected: &str, value: T) {
    assert_eq!(expected, value.to_string());
}

#[test]
fn test_display() {
    let path = Path::new("/thiserror");
    let file = path.to_owned();
    assert("failed to read '/thiserror'", StructPathBuf { file });
    let file = path.to_owned();
    assert("failed to read '/thiserror'", EnumPathBuf::Read(file));
    assert("failed to read '/thiserror'", StructPath::ref_cast(path));
}
