extern crate zmq;

use ::global_variables; 
use std::thread;

//TODO: change to better error handling
pub fn initialize_library(server_address : String, server_port : u32, tx : u32, rx : u32, callback : &'static Fn(&[u8]) -> ()) {

    let context = zmq::Context::new();

    let mut full_address = String::from("tcp://");
    full_address += &server_address;
    full_address += ":";
    full_address += &server_port.to_string();

    //TODO: remove asserts, don't want panic here, but some nice error message

    let listener = context.socket(zmq::SUB).unwrap();
    assert!(listener.connect(&full_address).is_ok()); 

    let sender = context.socket(zmq::PUSH).unwrap();
    assert!(sender.connect(&full_address).is_ok());

    start_message_receiveing_thread();

    global_variables::set_server_ip_address(server_address);
    global_variables::set_server_port(server_port);
    global_variables::set_tx(tx);
    global_variables::set_rx(rx);
    global_variables::set_received_message_callback(callback);
    global_variables::set_zmq_context(context);
    global_variables::set_zmq_listener_socket(listener);
    global_variables::set_zmq_sender_socket(sender);
}

pub fn send_message(data: &[u8]) -> Result<(), ()> {
    let tx = global_variables::get_tx();
    let rx = global_variables::get_rx();
    
    if data.len() <= 8 {
        let sender_socket = global_variables::get_zmq_sender_socket().as_ref().unwrap(); //need as_ref here because this whole option is a borrowed reference
                                                                                         //TODO: replace unwrapping with error handling
        return match sender_socket.send(data, 0) {
            Ok(_) => Ok(()),
            Err(_) => Err(())
        }
    }
    Err(())
}

pub fn set_received_message_callback(callback : &'static Fn(&[u8])->()) {
    global_variables::set_received_message_callback(callback);
}

fn start_message_receiveing_thread() -> Result<(), ()> {
    thread::spawn(|| {
        let listener_socket = global_variables::get_zmq_listener_socket().as_ref().unwrap();
        loop {
            let msg = listener_socket.recv_bytes(0).unwrap();
            thread::spawn(move || {
               global_variables::get_received_message_callback().as_ref().unwrap()(&msg);   //moved this to a separate thread to allow callbacks that last longer 
                                                                                            //some thread pool, or green threads need to be used here, this will be slow
            });
        }
    });
    Ok(())
}