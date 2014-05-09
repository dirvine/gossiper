use std::io::{TcpListener, TcpStream};
use std::io::net::ip::{SocketAddr, IpAddr};

pub struct Connection {
    stream: TcpStream,
    addr: SocketAddr
}

impl Connection {
    pub fn new(ip: IpAddr, port: u16) -> Connection {
        let addr = SocketAddr {
            ip: ip,
            port: port
        };

        let stream = match TcpStream::connect(addr) {
            Ok(stream) => stream,
            Err(err) => fail!("Tcp stream failed to connect: {}", err)
        };

        Connection {
            stream: stream,
            addr: addr
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use std::io::net::ip::{SocketAddr, Ipv4Addr};
    use std::io::{TcpListener, TcpStream, Listener, Acceptor};
    use std::io::net::tcp::TcpAcceptor;

    #[test]
    fn open_stream() {
        let ip = Ipv4Addr(127, 0, 0, 1);
        let port = 5689;

        let addr = SocketAddr { ip: ip, port: port };
        let acceptor = TcpListener::bind(addr).listen().unwrap();

        Connection::new(ip, port);
    }
}