use std::net::UdpSocket;
use super::sctp_connection;

pub struct SctpOverUdp
{
    sctp_conn: sctp_connection::SctpConnection,
    socket: UdpSocket,
}

impl SctpOverUdp
{
    pub fn new() -> SctpOverUdp {
        let socket = UdpSocket::bind("127.0.0.1:34254").unwrap();

        SctpOverUdp {
            sctp_conn: sctp_connection::SctpConnection::new(),
            socket: socket,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_udp_conn() {
        let sctp_over_udp = SctpOverUdp::new();
    }
}