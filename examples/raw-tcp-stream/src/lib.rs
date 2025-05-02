/* Shared code between client & server */
use std::io::{self, BufRead};

pub const DEFAULT_SERVER_ADDR: &str = "127.0.0.1:4000";
const MESSAGE_BUFFER_SIZE: usize = 32;

/// Given a buffer (in this case, TcpStream), write the bytes
/// to be transmitted via TCP
pub fn write_data(stream: &mut impl io::Write, data: &[u8]) -> io::Result<()> {
    // Here, `write_all()` attempts to write the entire slice, raising an error if it cannot do so
    stream.write_all(data)?;

    // An alternative is `write()` which will return the number of bytes
    // that *could* be sent. This can be used if your app has a mechanism
    // to handle this scenario.
    // E.g. TCP backpressure for high-bandwidth data
    //
    // This is an example of what `write_all()` does:
    // let bytes_to_write = data.len();
    // let bytes_written = stream.write(data)?;
    // if bytes_written < bytes_to_write {
    //     return Err(Error::new(ErrorKind::Interrupted, "Could not write all data"));
    // }

    // Signal that we're done writing and the data should be sent (with TCP PSH bit)
    stream.flush()
}

/// Given a buffer (in this case, TcpStream), attempt to read
/// an unknown stream of bytes and decode to a String
pub fn extract_string_unbuffered(buf: &mut impl io::Read) -> io::Result<String> {
    let mut received: Vec<u8> = vec![];

    // Use a statically sized array buffer
    // Picking a size is tricky:
    // - A large array can waste stack space for bytes we may never need
    // - A small array results in more syscalls (for this unbuffered approach)
    let mut rx_bytes = [0u8; MESSAGE_BUFFER_SIZE];
    loop {
        // Read from the current data in the TcpStream
        // NOTE: Each time this is called it can be a syscall
        let bytes_read = buf.read(&mut rx_bytes)?;

        // However many bytes we read, extend the `received` string bytes
        received.extend_from_slice(&rx_bytes[..bytes_read]);

        // And if we didn't fill the array
        // stop reading because there's no more data (we hope!)
        if bytes_read < MESSAGE_BUFFER_SIZE {
            break;
        }
    }

    String::from_utf8(received).map_err(|_| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            "Couldn't parse received string as utf8",
        )
    })
}

/// Given a buffer (in this case, TcpStream)
/// Use `BufReader` and `BufRead` trait to read
/// the pending bytes in the stream

#[cfg(test)]
mod tests {
    use super::*;
}
