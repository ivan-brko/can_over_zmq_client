extern crate zmq;

use ::global_variables; 

//TODO: change to better error handling
pub fn initialize_library(server_address : String, server_port : u32, tx : u32, rx : u32, callback : &'static Fn(&[u8]) -> ()) {
    global_variables::set_server_ip_address(server_address);
    global_variables::set_server_port(server_port);
    global_variables::set_tx(tx);
    global_variables::set_rx(rx);
    global_variables::set_received_message_callback(callback);


    let context = zmq::Context::new();
    let listener = context.socket(zmq::SUB).unwrap();
    assert!(listener.connect("tcp://127.0.0.1:5558").is_ok());

    global_variables::set_zmq_listener_socket(listener);

    global_variables::set_zmq_context(context);
}

pub fn send_message(data: &[u8]) -> Result<(), ()> {
    let tx = global_variables::get_tx();
    let rx = global_variables::get_rx();
    //TODO add message sending logic
    //TODO discard messages longer than 8 bytes
    Ok(())
}

pub fn set_received_message_callback(callback : &'static Fn(&[u8])->()) {
    global_variables::set_received_message_callback(callback);
}