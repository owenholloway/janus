#[cfg(test)]
mod tests {}

// Protocols

/// Test Modbus
#[cfg(test)]
mod modbus;

/// Transport Layers <br/>
/// <br/>
/// Internet Protocol Networks <br/>
/// [TCP/IP](https://en.wikipedia.org/wiki/Internet_protocol_suite) <br/>
/// <br/>
/// Serial Protocol Netwoks <br/>
/// [RS232](https://en.wikipedia.org/wiki/RS-232) <br/>
/// [RS422](https://en.wikipedia.org/wiki/RS-422) <br/>
/// [RS485](https://en.wikipedia.org/wiki/RS-485) <br/>
#[cfg(test)]
mod transport;
