mod error_handling;
mod parse_settings;
pub mod serlib {

    use std::ffi::OsString;
    use std::time::Duration;
    use serial::prelude::*;
    use std::io;
    use serial::core;
    use super::error_handling::SerlibErr;
    use super::parse_settings::*;
    use std::io::Read;
    use std::io::Write;

    pub struct Port<'a> {
        receiving_buffer_size: RecBufferSize,
        port: serial::SystemPort, 
        end_write_byte: &'a[u8],
        end_read_byte: &'a i8,
    }

    impl Port<'_> {
        pub fn open<'a>(port: String, baud_rate: usize) -> Result<Port<'a>, SerlibErr<'a>> {
            Port::open_with_settings(
                port,    
                0,       
                baud_rate,    
                8,       
                1,       
                &"None", 
                &([0x1C][0] as i8),
                60,      
                &"S".as_bytes(),
                0,       
                &"64"    
            )
        }
        pub fn open_with_settings<'a>
        (
            port: String,                   // e.g "COM1"
            parity: usize,                  // Between 0 and 2
            baud_rate: usize,               // Any, make sure it matches you devices
            char_size: usize,               // Between 5 and 8
            stop_bits: usize,               // Either 1 or 2
            flow_control: &'a str,          // Either "None", "Software" or "Hardware"
            end_read_byte: &'a i8,          // The byte that signifies the end of a read
            timeout_seconds: usize,         // Any integer, make sure it's appropriate
            end_write_byte: &'a[u8],        // The byte that signifies the end of a write 
            timeout_nanoseconds: usize,     // Any integer, make sure it's appropriate
            receiving_buffer_size: &'a str, // Either "unlimmited" or a string slice of an integer  
        ) -> 
            Result<Port<'a>, SerlibErr<'a>>
        {
            // Converts the timeout seconds and nanoseconds into a suration struct 
            let timeout: Duration = Duration::new(timeout_seconds as u64, timeout_nanoseconds as u32);
    
            // Creates the PortSettings struct to be used later on, uses helper functions to make 
            // each required struct, checking the values  
            let settings: serial::PortSettings = serial::PortSettings {
                baud_rate: core::BaudRate::from_speed(baud_rate),
                char_size: get_char_size(char_size)?,
                parity: get_parity(parity)?,
                stop_bits: get_stop_bits(stop_bits)?,
                flow_control: get_flow_control(flow_control)?,
            };
    
            let mut port: serial::SystemPort = 
            match serial::open(&OsString::from(&port)) {
                Ok(serial_port) => serial_port,
                Err(core_err_kind) => return Err(handle_core_errors(core_err_kind.kind())),
            };
            match port.configure(&settings) {
                Ok(()) => {  },
                Err(core_err_kind) => return Err(handle_core_errors(core_err_kind.kind())),
            }
            match port.set_timeout(timeout) {
                Ok(()) => {  },
                Err(core_err_kind) => return Err(handle_core_errors(core_err_kind.kind()))
            } 
            Ok(Port {
                receiving_buffer_size: new_rec_buffer_size(receiving_buffer_size)?,
                end_write_byte: end_write_byte,
                end_read_byte: end_read_byte,
                port: port,
            })
        }
        pub fn serial_write_segments_read<'a, T: io::Write + io::Read>(&mut self, data: Vec<&'a str>) -> 
            Result<Vec<i8>, io::Error> 
        {
            let mut result_vector = Vec::new();
            for segment in data {
                self.port.write(segment.as_bytes())?;
                self.port.write(self.end_write_byte)?;
                for byte in self.read_until_eof_char()? {
                    result_vector.push(byte);
                }
            }
            Ok(result_vector)
        }
    
        pub fn read_until_eof_char(&mut self) -> Result<Vec<i8>, io::Error> {
            let mut result_vector = Vec::new();
            loop {
                let byte = self.read_i8().expect("Port read error.");
                if &byte == self.end_read_byte { break; }
                if byte != 0 { result_vector.push(byte); }
            }
            Ok(result_vector)
        }
    
        pub fn read_i8(&mut self) -> Result<i8, io::Error>
        {
            let mut read_buffer = [0u8; 1];
            self.port.read(&mut read_buffer)?;
            Ok(read_buffer[0] as i8)
        }
    }
}
