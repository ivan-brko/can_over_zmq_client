extern crate libc;

use std;
use self::libc::c_char;
use std::ffi::CStr;
use super::rust_api;


//TODO: check that no nulls are passed from the C layer to the lib

//TODO: memory handling for pointers coming from C-code? who needs to delete what?
#[no_mangle]
pub extern "C" fn initialize_library(server_address: *const c_char, server_port_for_listener : u32, server_port_for_sender : u32, rx_list_ptr: *const libc::uint32_t, size: libc::size_t, callback: extern "C" fn(libc::uint32_t, *const libc::uint8_t, libc::size_t)) {
        let server_address = unsafe {CStr::from_ptr(server_address)};
        let server_address = String::from(server_address.to_str().unwrap());
        let rx_list = unsafe {std::slice::from_raw_parts(rx_list_ptr, size)};
        rust_api::initialize_library(server_address, server_port_for_listener, server_port_for_sender, rx_list, Box::new(move |id: u32, data: &[u8]| {
           let data_as_ptr = data.as_ptr() as *const libc::uint8_t;
           let length = data.len() as libc::size_t;
           callback(id, data_as_ptr, length); //TODO: memory leak? who has to delete the data for each message?
        }));
}

#[no_mangle]
pub extern "C" fn send_can_message(array_pointer: *const libc::uint8_t, size: libc::size_t, tx: u32) {
    unsafe {
        let data_as_slice = std::slice::from_raw_parts(array_pointer as *const u8, size as usize);
        let _ = rust_api::send_message(data_as_slice, tx);
    }
}
