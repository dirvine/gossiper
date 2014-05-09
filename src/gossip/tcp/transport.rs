use std::io::net::ip::{SocketAddr, IpAddr};
use std::io::{TcpListener, TcpStream, Listener, Acceptor};
use std::io::net::tcp::TcpAcceptor;

use transport::Transport;
use util::GossipResult;

pub struct TcpTransport {
    acceptor: TcpAcceptor
}

impl TcpTransport {
    pub fn new(ip: IpAddr, port: u16) -> TcpTransport {
        let addr = SocketAddr {
            ip: ip,
            port: port
        };

        TcpTransport {
            acceptor: TcpListener::bind(addr).listen().unwrap()
        }
    }
}

impl Transport for TcpTransport {
    fn join<T>(&self, ip: IpAddr, port: u16) -> GossipResult<T> {
        unimplemented!()
    }
}

impl Drop for TcpTransport {
    fn drop(&mut self) {
        drop(self)
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use std::io::net::ip::{Ipv4Addr};
    use tcp::connection::TcpConnection;

    #[test]
    fn new_transport() {
        let addr = Ipv4Addr(127, 0, 0, 1);
        let port = 5499;

        let transport = TcpTransport::new(addr, port);
        let connection = TcpConnection::new(addr, port);
    }
}