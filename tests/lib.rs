#![allow(unstable)]
extern crate atomicwrites;

use std::old_io;
use atomicwrites::{AtomicFile,AllowOverwrite,DisallowOverwrite};

fn get_tmp() -> Path {
    old_io::TempDir::new("atomicwrites-test").unwrap().into_inner()
}

#[test]
fn test_simple_allow_override() {
    let tmpdir = get_tmp();
    let path = tmpdir.join(Path::new("haha"));

    let af = AtomicFile::new(&path, AllowOverwrite, None);
    af.write(|&: f| f.write_str("HELLO")).unwrap();

    let mut testfd = old_io::File::open(&path);
    let rv = testfd.read_to_string().unwrap();
    assert_eq!(rv.as_slice(), "HELLO");
}

#[test]
fn test_simple_disallow_override() {
    let tmpdir = get_tmp();
    let path = tmpdir.join(Path::new("haha"));

    let af = AtomicFile::new(&path, DisallowOverwrite, None);
    af.write(|&: f| f.write_str("HELLO")).unwrap();

    let mut testfd = old_io::File::open(&path);
    let rv = testfd.read_to_string().unwrap();
    assert_eq!(rv.as_slice(), "HELLO");
}
