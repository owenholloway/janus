pub enum FunctionCode {
    Confirm = 0x00,
    Read = 0x02,
    Write = 0x03,
    Select = 0x04,
    Operate = 0x05,
    DirOperate = 0x06,
    DirOperateNoResp = 0x07,
    Response = 0x81,
    UnsolicitedResponse = 0x82,
}
