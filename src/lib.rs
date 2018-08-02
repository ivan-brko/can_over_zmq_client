extern crate zmq;

//global variables
//TODO: move this to separate module
static mut SERVER_IP_ADDRESS: Option<String> = None;
static mut SERVER_PORT : Option<u32> = None;
static mut TX : Option<u32> = None;
static mut RX : Option<u32> = None;
static mut ZMQ_CONTEXT : Option<zmq::Context> = None;
static mut ZMQ_SOCKET : Option<zmq::Socket> = None;

//TODO: these getters and setters are not thread safe, fix that
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


pub fn get_rx() -> Option<u32> {
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

pub fn get_zmq_socket () -> &'static Option<zmq::Socket> {
    unsafe {
        &ZMQ_SOCKET
    }
}

pub fn set_zmq_socket(socket: zmq::Socket) {
    unsafe {
        ZMQ_SOCKET = Some(socket);
    }
}