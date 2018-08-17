extern crate libc;

use std;
use self::libc::c_char;
use std::ffi::CStr;
use super::rust_api;


//TODO: check that no nulls are passed from the C layer to the lib

//TODO: memory handling for pointers coming from C-code? who needs to delete what?
#[no_mangle]
pub extern "C" fn initialize_library(server_address: *const c_char, server_port_for_listener : u32, server_port_for_sender : u32, tx : u32, rx : u32, callback: extern "C" fn(*const libc::uint8_t, libc::size_t)) {
        let server_address = unsafe {CStr::from_ptr(server_address)};
        let server_address = String::from(server_address.to_str().unwrap());
        rust_api::initialize_library(server_address, server_port_for_listener, server_port_for_sender, tx, rx, Box::new(|data: &[u8]| {
           println!("adasdsa");
        }));
}

#[no_mangle]
pub extern "C" fn send_can_message(array_pointer: *const libc::uint8_t, size: libc::size_t) {
    unsafe {
        let data_as_slice = std::slice::from_raw_parts(array_pointer as *const u8, size as usize);
        let _ = rust_api::send_message(data_as_slice);
    }
}

// #[no_mangle]
// pub extern fn rust_multiply(size: libc::size_t, array_pointer: *const libc::uint32_t) -> libc::uint32_t {
//     internal_rust_multiply(unsafe { std::slice::from_raw_parts(array_pointer as *const i32, size as usize) }) as libc::uint32_t
// }

// fn internal_rust_multiply(array: &[i32]) -> i32
// {
//     assert!(!array.is_empty());
//     array[0]
// }