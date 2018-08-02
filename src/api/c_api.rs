extern crate libc;

use std;
use super::rust_api;

//TODO: check that no nulls are passed from the C layer to the lib

#[no_mangle]
pub extern "C" fn send_can_message(array_pointer: *const libc::uint32_t, size: libc::size_t) {
    unsafe {
        let data_as_slice = std::slice::from_raw_parts(array_pointer as *const u8, size as usize);
    }

    

}

#[no_mangle]
pub extern fn rust_multiply(size: libc::size_t, array_pointer: *const libc::uint32_t) -> libc::uint32_t {
    internal_rust_multiply(unsafe { std::slice::from_raw_parts(array_pointer as *const i32, size as usize) }) as libc::uint32_t
}

fn internal_rust_multiply(array: &[i32]) -> i32
{
    assert!(!array.is_empty());
    array[0]
}