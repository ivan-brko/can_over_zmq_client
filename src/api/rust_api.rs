use ::global_variables; 

//TODO: change to better error handling

pub fn send_message(data: &[u8]) -> Result<(), ()> {
    let tx = global_variables::get_tx();
    //TODO add message sending logic
    //TODO discard messages longer than 8 bytes
    Ok(())
}

pub fn set_received_message_callback(callback : &Fn(&[u8])->()) {
    
}