extern crate serial;
pub mod serlib {

    use std::ffi::OsString;
    use std::time::Duration;
    use serial::prelude::*;
    use std::io;
    use serial::core;

    #[derive(Debug)]
    pub enum SerlibErr<'a> {
        ParitySizeErr(usize),        // 
        CharSizeErr(usize),          // 
        StopBitSizeErr(usize),       // 
        FlowControlTypeErr(&'a str), // 
        InvalidInput,                // 
        NoSerialDevice,              // 
        IoErr(io::ErrorKind),        // 
        InvalidBufSizeErr(&'a str),  // 
    }

    impl SerlibErr<'_> {
        pub fn describe(&self) -> String 
        {
            match self {
                SerlibErr::ParitySizeErr(specified_size) => format!("The size specified for the parity neeeds to be between 0 and 2, specified value is {}.", specified_size),
                SerlibErr::CharSizeErr(specified_size) => format!("The size specified for the character size needs to be between 5 and 8, specified size is {}", specified_size),
                SerlibErr::StopBitSizeErr(specified_size) => format!("The specified stop bit needs to be either 0 or 1, specified size is {}", specified_size),
                SerlibErr::FlowControlTypeErr(specified_val) => format!("The specified value for the control value needs to be either \"None\", \"Software\" or \"Hardware\", specified value {}", specified_val),
                SerlibErr::InvalidInput => String::from("Invalid input for the port name, ensure this is correct."),
                SerlibErr::NoSerialDevice => String::from("No such device is available, either used by another process or does not exist."),
                SerlibErr::IoErr(io_err_kind) => describe_io_err(io_err_kind),
                SerlibErr::InvalidBufSizeErr(specified_val) => format!("The specified value for the receiving buffer needs to be either an integer or \"Unlimmited\", specified value {}", specified_val)  
            }
        }
    }

    fn describe_io_err(err: &io::ErrorKind) -> String {
        match err {
            io::ErrorKind::NotFound => String::from("Port not found, check port name."),
            io::ErrorKind::PermissionDenied => String::from("Permission has been denied."),
            io::ErrorKind::ConnectionRefused => String::from("Connection has been refused."),
            io::ErrorKind::ConnectionReset => String::from("Connection has been reset."),
            io::ErrorKind::ConnectionAborted => String::from("Connection has been aborted."),
            io::ErrorKind::NotConnected => String::from("Not connected."),
            io::ErrorKind::AddrInUse => String::from("Port is in use."),
            io::ErrorKind::AddrNotAvailable => String::from("Port not found, check port name."),
            io::ErrorKind::BrokenPipe => String::from("Pipe has been broken, one process cannot pass to another."),
            io::ErrorKind::AlreadyExists => String::from("Connection already exists."),
            io::ErrorKind::WouldBlock => String::from("This operation needs the parent thread to block, however a block was not requested."),
            io::ErrorKind::InvalidInput => String::from("Input parameter is invalid."),
            io::ErrorKind::InvalidData => String::from("The data used for the operation is invalid."),
            io::ErrorKind::TimedOut => String::from("Port timed out."),
            io::ErrorKind::WriteZero => String::from("A write operation returned an Ok(0) indicating data was not written."),
            io::ErrorKind::Interrupted => String::from("The connection or operation was interupted."),
            io::ErrorKind::Other => String::from("Unknown IO level error."),
            io::ErrorKind::UnexpectedEof => String::from("An Eof (end of file) character was found."),
            _ => String::from("Unknown IO level error."),
        }
    }

    pub struct Port<'a> {
        receiving_buffer_size: RecBufferSize,
        port: serial::SystemPort, 
        end_write_byte: &'a[u8],
        end_read_byte: &'a i8,
    }

    enum RecBufferSize {
        Unlimmited,
        Limmit(usize)
    }

    fn new_rec_buffer_size<'a>(size: &'a str) -> Result<RecBufferSize, SerlibErr<'a>> 
    {
        match size {
            "Unlimmited" => Ok(RecBufferSize::Unlimmited),
            _ => {
                let ret_int = size.parse::<usize>();
                match ret_int {
                    Ok(ret_int) => Ok(RecBufferSize::Limmit(ret_int)),
                    Err(_) => Err(SerlibErr::InvalidBufSizeErr(size))
                } 
            }
        }
    }

    pub fn open_default<'a>(port: String) -> Result<Port<'a>, SerlibErr<'a>>
    {
        open_with_settings(
            port,    
            0,       
            9600,    
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
        receiving_buffer_size: &'a str, // Either "Unlimmited" or a string slice of an integer  
    ) -> 
        Result<Port<'a>, SerlibErr<'a>>
    {

        let settings: serial::PortSettings = serial::PortSettings {
            baud_rate: core::BaudRate::from_speed(baud_rate),
            char_size: get_char_size(char_size)?,
            parity: get_parity(parity)?,
            stop_bits: get_stop_bits(stop_bits)?,
            flow_control: get_flow_control(flow_control)?,
        };

        let port_open_success = serial::open(&OsString::from(&port));
        let mut port: serial::SystemPort = 
        match port_open_success {
            Ok(serial_port) => serial_port,
            Err(core_err_kind) => return Err(handle_core_errors(core_err_kind.kind())),
        };
        let port_config_sucess = port.configure(&settings);
        match port_config_sucess {
            Ok(()) => {  },
            Err(core_err_kind) => return Err(handle_core_errors(core_err_kind.kind())),
        }
        let set_timeout_res = port.set_timeout(Duration::new(timeout_seconds as u64, timeout_nanoseconds as u32));
        match set_timeout_res {
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

    fn get_char_size<'a>(size: usize) -> Result<core::CharSize, SerlibErr<'a>> {
        match size {
            5 => Ok(core::CharSize::Bits5),
            6 => Ok(core::CharSize::Bits6),
            7 => Ok(core::CharSize::Bits7),
            8 => Ok(core::CharSize::Bits8),
            _ => Err(SerlibErr::CharSizeErr(size)),
        }
    }

    fn get_parity<'a>(parity: usize) -> Result<core::Parity, SerlibErr<'a>> {
        match parity {
            0 => Ok(core::Parity::ParityNone),
            1 => Ok(core::Parity::ParityOdd),
            2 => Ok(core::Parity::ParityEven),
            _ => Err(SerlibErr::ParitySizeErr(parity)),
        }
    }

    fn get_stop_bits<'a>(size: usize) -> Result<core::StopBits, SerlibErr<'a>> {
        match size {
            1 => Ok(core::StopBits::Stop1),
            2 => Ok(core::StopBits::Stop2),
            _ => Err(SerlibErr::StopBitSizeErr(size)),
        }
    }

    fn get_flow_control<'a>(flow_control: &'a str) -> Result<core::FlowControl, SerlibErr<'a>> 
    {
        match flow_control {
            "None" => Ok(core::FlowControl::FlowNone),
            "Software" => Ok(core::FlowControl::FlowNone),
            "Hardware" => Ok(core::FlowControl::FlowNone),
            _ => Err(SerlibErr::FlowControlTypeErr(flow_control))
        }
    }

    fn handle_core_errors<'a>(kind: core::ErrorKind) -> SerlibErr<'a> {
        match kind {
            core::ErrorKind::NoDevice => SerlibErr::NoSerialDevice,
            core::ErrorKind::InvalidInput => SerlibErr::InvalidInput,
            core::ErrorKind::Io(sub_kind) => SerlibErr::IoErr(sub_kind),
        }
    }

    pub fn serial_write_segments_read<'a, T: io::Write + io::Read>(port: &'a mut T, data: Vec<&'a str>, end_write_byte: &'a[u8], end_read_byte: &'a i8) -> 
        Result<Vec<i8>, io::Error> 
    {
        let mut result_vector = Vec::new();
        for segment in data {
            port.write(segment.as_bytes())?;
            port.write(end_write_byte)?;
            for byte in read_until_eof_char(port, end_read_byte)? {
                result_vector.push(byte);
            }
        }
        Ok(result_vector)
    }

    pub fn parse_data<'a>(data: &'a String, parse_string: &'a str, max: &'a usize) -> Result<Vec<&'a str>, ()> {
        let return_vec: Vec<&str>;
        if data.len() < *max {
            return_vec = vec![data];
        }
        else {
            return_vec = data.split(parse_string).collect();
            for item in &return_vec {
                if item.len() > *max { return Err(()) }
            }
        }
        Ok(return_vec)
    }

    pub fn read_until_eof_char<'a, T: io::Read>(mut port: &'a mut T, eof_char: &'a i8) -> Result<Vec<i8>, io::Error> {
        let mut result_vector = Vec::new();
        loop {
            let byte = read_i8(&mut port).expect("Port read error.");
            if &byte == eof_char { break; }
            if byte != 0 { result_vector.push(byte); }
        }
        Ok(result_vector)
    }

    pub fn read_i8<T: io::Read>(file: &mut T) -> Result<i8, io::Error>
    {
        let mut read_buffer = [0u8; 1];
        file.read(&mut read_buffer)?;
        Ok(read_buffer[0] as i8)
    }

}
