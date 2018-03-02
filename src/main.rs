extern crate libc;
use libc::{c_char, c_int, uint64_t};
use std::ffi::CString;

#[link(name = "pHash")]
extern "C" {
    fn ph_dct_imagehash(file: *const c_char, hash: *mut uint64_t) -> c_int;
}

fn image_hash(file: &str) -> Option<u64> {
    let mut hash: u64 = 0;
    let cstr = CString::new(file).unwrap();
    unsafe {
        let ptr = cstr.as_ptr();
        match ph_dct_imagehash(ptr, &mut hash) {
            0 => Some(hash),
            _ => None,
        }
    }
}

fn main() {
    println!("Hello, world!");
}
