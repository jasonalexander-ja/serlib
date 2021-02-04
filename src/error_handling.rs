use std::io;

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