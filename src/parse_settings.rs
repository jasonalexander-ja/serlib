
use serial::core;
use super::error_handling::SerlibErr;

pub enum RecBufferSize {
    Unlimmited,
    Limmit(usize)
}

pub fn new_rec_buffer_size<'a>(size: &'a str) -> Result<RecBufferSize, SerlibErr<'a>> 
{
    let size_lower = size.to_ascii_lowercase();
    match size_lower.as_str() {
        "unlimmited" => Ok(RecBufferSize::Unlimmited),
        _ => {
            let ret_int = size.parse::<usize>();
            match ret_int {
                Ok(ret_int) => Ok(RecBufferSize::Limmit(ret_int)),
                Err(_) => Err(SerlibErr::InvalidBufSizeErr(size))
            } 
        }
    }
}

pub fn get_flow_control<'a>(flow_control: &'a str) -> Result<core::FlowControl, SerlibErr<'a>> 
{
    let flow_control_lower = flow_control.to_ascii_lowercase();
    match flow_control_lower.as_str() {
        "none" => Ok(core::FlowControl::FlowNone),
        "software" => Ok(core::FlowControl::FlowNone),
        "hardware" => Ok(core::FlowControl::FlowNone),
        _ => Err(SerlibErr::FlowControlTypeErr(flow_control))
    }
}

pub fn get_char_size<'a>(size: usize) -> Result<core::CharSize, SerlibErr<'a>> {
    match size {
        5 => Ok(core::CharSize::Bits5),
        6 => Ok(core::CharSize::Bits6),
        7 => Ok(core::CharSize::Bits7),
        8 => Ok(core::CharSize::Bits8),
        _ => Err(SerlibErr::CharSizeErr(size)),
    }
}

pub fn get_parity<'a>(parity: usize) -> Result<core::Parity, SerlibErr<'a>> {
    match parity {
        0 => Ok(core::Parity::ParityNone),
        1 => Ok(core::Parity::ParityOdd),
        2 => Ok(core::Parity::ParityEven),
        _ => Err(SerlibErr::ParitySizeErr(parity)),
    }
}

pub fn get_stop_bits<'a>(size: usize) -> Result<core::StopBits, SerlibErr<'a>> {
    match size {
        1 => Ok(core::StopBits::Stop1),
        2 => Ok(core::StopBits::Stop2),
        _ => Err(SerlibErr::StopBitSizeErr(size)),
    }
}

pub fn handle_core_errors<'a>(kind: core::ErrorKind) -> SerlibErr<'a> {
    match kind {
        core::ErrorKind::NoDevice => SerlibErr::NoSerialDevice,
        core::ErrorKind::InvalidInput => SerlibErr::InvalidInput,
        core::ErrorKind::Io(sub_kind) => SerlibErr::IoErr(sub_kind),
    }
}