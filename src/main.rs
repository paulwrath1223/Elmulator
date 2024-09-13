mod commands;


use std::io;
use std::io::Write;

const DELIMETER: char = '\r';
const SERIAL_BUFFER_LEN: usize = 1024;

fn main() {
    println!("Select port:");
    
    match serialport::available_ports(){
        Ok(port_list) => {
            for port in port_list {
                println!("{}, {:?}", port.port_name, port.port_type);
                match port.port_type {
                    serialport::SerialPortType::UsbPort(usb_port_info) => {
                        if usb_port_info.manufacturer == Some(String::from("Silicon Labs")) {
                            println!("Found the right port at {}", port.port_name)
                        }
                    }
                    serialport::SerialPortType::PciPort => {println!("ignoring PCI serial")}
                    serialport::SerialPortType::BluetoothPort => {println!("ignoring Bluetooth Serial")}
                    serialport::SerialPortType::Unknown => {println!("weird unknown type, ignoring")}
                }
            }
        }
        Err(e) => {
            println!("No ports found. See exact error {}", e);
        }
    }

    let mut input_string = String::new();
    io::stdin().read_line(&mut input_string).unwrap();
    
    let mut elmulator = serialport::new(
        input_string.trim(), 
        115_200
    ).open().expect(&format!("Failed to open serial port: {}", input_string.as_str()));

    let mut intermediate_serial_buf: [u8; SERIAL_BUFFER_LEN] = [0u8; SERIAL_BUFFER_LEN];
    let mut buffer_index: usize;
    
    println!("opened successfully");
    
    let default_response_chars: [char; 5] = ['?', '\n', '\n', '>', '\n'];
    let response_as_u8: [u8; 5] = default_response_chars.map(|c| c as u8);
    
    loop{
        buffer_index = 0;
        while elmulator.bytes_to_read().unwrap() == 0{

        }

        while !intermediate_serial_buf[..buffer_index].contains(&(DELIMETER as u8)) && buffer_index < intermediate_serial_buf.len() {
            while elmulator.bytes_to_read().unwrap() > 0 {
                buffer_index += elmulator.read(&mut intermediate_serial_buf[buffer_index..]).expect("Found no data despite `bytes_to_read > 0`!");
            }
        }
        if buffer_index >= SERIAL_BUFFER_LEN {
            println!("Buffer overflow, received over {SERIAL_BUFFER_LEN} bytes with no {:?}", DELIMETER);
        }

        let response_slice: &[u8] = &intermediate_serial_buf[..buffer_index];
        let response_string = response_slice.iter().map(|&b| char::from(b)).collect::<String>();
        println!("Received response: {}", response_string);
        
        elmulator.write(&response_as_u8).expect("Failed to write to serial port");
    }

    

    // commands::STATIC_COMMAND_LUT
    
    
    



    
    
    
}

