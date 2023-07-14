use super::x01_read::ReadResponse;

pub struct ResponseMessage {
    pub response: Response,
    pub indicators: Indicators,
}

pub enum Response {
    //4.4.1
    Confirm,
    //4.4.2
    Read(ReadResponse),
}

//As per 4.5 IIN bits
pub struct Indicators {
    /// LSB IIN1.0 <br> Broadcast Message Received
    pub broadcast: bool,
    /// LSB IIN1.1 <br> Addditional Class 1 Event Data Is Available
    pub class_1_events: bool,
    /// LSB IIN1.2 <br> Addditional Class 2 Event Data Is Available
    pub class_2_events: bool,
    /// LSB IIN1.3 <br> Addditional Class 3 Event Data Is Available
    pub class_3_events: bool,
    /// LSB IIN1.4 <br> Time Synchronization Required
    pub need_time: bool,
    /// LSB IIN1.5 <br> Some Output Points Are In Local Mode
    pub local_control: bool,
    /// LSB IIN1.6 <br> Device Trouble
    pub device_trouble: bool,
    /// LSB IIN1.7 <br> Device Restart
    pub device_restart: bool,
    /// MSB IIN2.0 <br>  
    pub no_func_code_support: bool,
    /// MSB IIN2.1 <br>
    pub object_unknown: bool,
    /// MSB IIN2.2 <br>
    pub parameter_error: bool,
    /// MSB IIN2.3 <br>
    pub event_buffer_overflow: bool,
    /// MSB IIN2.4 <br>
    pub already_executing: bool,
    /// MSB IIN2.5 <br>
    pub config_corrupt: bool,
    /// MSB IIN2.6 <br> Reserved bit
    pub reservered_bit_2: bool,
    /// MSB IIN2.7 <br> Reserved bit
    pub reservered_bit_1: bool,
}
