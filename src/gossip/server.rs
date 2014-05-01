use std::io::net::ip::IpAddr;
use std::io::net::ip::SocketAddr;
use cluster::Cluster;
use metadata::Metadata;
use transport::Transport;
use tcp::TcpTransport;

/// A server/node within a single gossip cluster. Each server has
/// a fast knowledge of it's cluster, which is all stored here.
#[deriving(Show,Eq)]
pub struct Server {
    addr: SocketAddr,
    cluster: Option<Cluster>,
    metadata: Metadata,
    transport: TcpTransport
}

impl Server {
    /// Create a new server given an address (ipv4 or ipv6) and a port.
    /// This function will **not** do any connection initializations. This
    /// is handled by further methods.
    pub fn new(ip: IpAddr, port: u16) -> Server {
        Server {
            // We're handling the creation of the SocketAddr to allow
            // for a more friendly API.
            addr: SocketAddr {
                ip: ip,
                port: port
            },

            // By default, we aren't joining a cluster yet.
            cluster: None,
            metadata: Metadata::new(),
            transport: TcpTransport::new()
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use std::io::net::ip::Ipv4Addr;
    use tcp::TcpTransport;

    #[test]
    fn new_server() {
        let server = Server::new(Ipv4Addr(127, 0, 0, 1), 4989);

        assert_eq!(server.addr.ip, Ipv4Addr(127, 0, 0, 1));
        assert_eq!(server.addr.port, 4989);
        match server.cluster {
            Some(_) => fail!("Expected a new server without joining a cluster."),
            None => {}
        }
    }
}