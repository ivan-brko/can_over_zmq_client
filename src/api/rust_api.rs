extern crate zmq;

use ::global_variables; 
use std::thread;

//TODO: change to better error handling
pub fn initialize_library(server_address : String, server_port_for_listener : u32, server_port_for_sender : u32, tx : u32, rx : u32, callback : Box<Fn(&[u8]) -> ()>) {

    let context = zmq::Context::new();

    let mut full_address = String::from("tcp://");
    full_address += &server_address;
    full_address += ":";
    let full_address_for_listener = full_address.clone() + &server_port_for_listener.to_string();
    let full_address_for_sender = full_address + &server_port_for_sender.to_string();

    //TODO: remove asserts, don't want panic here, but some nice error message

    let listener = context.socket(zmq::SUB).unwrap();
    println!("{}", full_address_for_listener);
    assert!(listener.connect(&full_address_for_listener).is_ok()); 
    let filter : Vec<u8> = Vec::new();
    assert!(listener.set_subscribe(&filter).is_ok());

    let sender = context.socket(zmq::PUSH).unwrap();
    assert!(sender.connect(&full_address_for_sender).is_ok());

    global_variables::set_server_ip_address(server_address);
    global_variables::set_server_port(server_port_for_listener); //TODO: store for sender as well
    global_variables::set_tx(tx);
    global_variables::set_rx(rx);
    global_variables::set_received_message_callback(callback);
    global_variables::set_zmq_context(context);
    global_variables::set_zmq_listener_socket(listener);
    global_variables::set_zmq_sender_socket(sender);

    let _ = start_message_receiveing_thread();

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

fn start_message_receiveing_thread() -> Result<(), ()> {
    thread::spawn(|| {
        println!("Started listener thread");
        let listener_socket = global_variables::get_zmq_listener_socket().as_ref().unwrap();
        println!("Got the listener thread");
        loop {
            let msg = listener_socket.recv_bytes(0).unwrap();
            println!("Got a message here");
            thread::spawn(move || {
               global_variables::get_received_message_callback().as_ref().unwrap()(&msg);   //moved this to a separate thread to allow callbacks that last longer 
                                                                                            //some thread pool, or green threads need to be used here, this will be slow
            });
        }
    });
    Ok(()) //TODO: add proper checking
}