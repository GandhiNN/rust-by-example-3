use std::io::{self, Write};
use std::net::{SocketAddr, TcpStream};

use structopt::StructOpt;

use crate::{DEFAULT_SERVER_ADDR, extract_string_unbuffered, write_data};

#[derive(Debug, StructOpt)]
#[structopt(name = "client")]
struct Args {
    message: String,
    /// Server destination address
    #[structopt(long, default_value = DEFAULT_SERVER_ADDR, global = true)]
    addr: SocketAddr,
}

fn main() -> io::Result<()> {
    let args = Args::from_args();

    // Establish a TCP connection with the far end
    let mut stream = TcpStream::connect(args.addr)?;
    write_data(&mut stream, &args.message.as_bytes())?;

    // Now read & print the response
    // (this will block until all data has been received)
    extract_string_unbuffered(&mut stream).map(|resp| println!("{}", resp))
}
