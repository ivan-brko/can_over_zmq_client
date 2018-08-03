extern crate zmq;
extern crate libc;
//module for all global variables
//TODO: nothing is thread safe ATM, fix that later

static mut SERVER_IP_ADDRESS: Option<String> = None;
static mut SERVER_PORT : Option<u32> = None;
static mut TX : Option<u32> = None;
static mut RX : Option<u32> = None;
static mut ZMQ_CONTEXT : Option<zmq::Context> = None;
static mut ZMQ_SENDER_SOCKET : Option<zmq::Socket> = None;
static mut ZMQ_LISTENER_SOCKET : Option<zmq::Socket> = None;
static mut RECEIVED_MESSAGE_CALLBACK : Option<&Fn(&[u8])->()> = None;

pub fn get_server_ip_address () -> &'static Option<String> {
    unsafe {
        &SERVER_IP_ADDRESS
    }
}

pub fn set_server_ip_address(address: String) {
    unsafe {
        SERVER_IP_ADDRESS = Some(address);
    }
}

pub fn get_server_port() -> Option<u32> {
    unsafe {
        SERVER_PORT
    }
}

pub fn set_server_port(port:u32) {
    unsafe {
        SERVER_PORT = Some(port);
    }
}

pub fn get_tx() -> Option<u32> {
    unsafe {
        TX
    }
}

pub fn set_tx(tx:u32) {
    unsafe {
        TX = Some(tx);
    }
}

#[no_mangle]
pub extern "C" fn get_rx() -> Option<u32> {
    unsafe {
        RX
    }
}

pub fn set_rx(rx:u32) {
    unsafe {
        RX = Some(rx);
    }
}

pub fn get_zmq_context () -> &'static Option<zmq::Context> {
    unsafe {
        &ZMQ_CONTEXT
    }
}


pub fn set_zmq_context(context: zmq::Context) {
    unsafe {
        ZMQ_CONTEXT = Some(context);
    }
}

pub fn get_zmq_listener_socket () -> &'static Option<zmq::Socket> {
    unsafe {
        &ZMQ_LISTENER_SOCKET
    }
}

pub fn set_zmq_listener_socket(socket: zmq::Socket) {
    unsafe {
        ZMQ_LISTENER_SOCKET = Some(socket);
    }
}

pub fn get_zmq_sender_socket () -> &'static Option<zmq::Socket> {
    unsafe {
        &ZMQ_SENDER_SOCKET
    }
}

pub fn set_zmq_sender_socket(socket: zmq::Socket) {
    unsafe {
        ZMQ_SENDER_SOCKET = Some(socket);
    }
}
//static mut RECEIVED_MESSAGE_CALLBACK : Option<&Fn(&[u8])->()> = None;
pub fn get_received_message_callback() -> &'static Option<&'static Fn(&[u8])->()> {
    unsafe {
        &RECEIVED_MESSAGE_CALLBACK
    }
}

pub fn set_received_message_callback(callback: &'static Fn(&[u8])->()) {
    unsafe {
        RECEIVED_MESSAGE_CALLBACK = Some(callback);
    }
}