use std::net::UdpSocket;
use super::sctp_connection;
use super::sctp_message;
use sockets_api;
use sockets_api::SocketsApi;
use std::net::{SocketAddr, IpAddr, Ipv4Addr, Ipv6Addr};
use std::io::Error;
use std::thread;
use bincode::{serialize, deserialize, Infinite};


pub struct SctpOverUdpThreaded
{
    sctp_conn: sctp_connection::SctpConnection,
    socket:UdpSocket,
}

impl SctpOverUdpThreaded
{
    pub fn new(addr: SocketAddr) -> SctpOverUdpThreaded {
        let socket = UdpSocket::bind(addr).unwrap();

        SctpOverUdpThreaded {
            sctp_conn: sctp_connection::SctpConnection::new(),
            socket: socket,
        }
    }
}

impl sockets_api::SocketsApi for SctpOverUdpThreaded
{
    /// Doesn't actually create a socket, since Rust std's UDP socket needs to be bound at creation.
    fn socket(&self, ipv: sockets_api::IPV) -> Result<(), &'static str>
    {
        Ok(())
    }
    /// Binds the socket to an address. You can give either an ipv6 or ipv4 address.
    fn bind(&mut self, addr: SocketAddr) -> Result<(), Error>
    {
        self.socket = UdpSocket::bind(addr)?;
        
        Ok(())
    }
    /// Starts to listen for connections. Will be unimplemented on the non-native, non-threaded API.
    /// On the threaded UDP API this will start the background thread(s).
    /// On the native implentation, it will map to native listen() call.
    fn listen(&self) -> Result<(), &'static str>
    {
        thread::spawn(move || {
            // TODO
            // Read messages in a loop to detect corrupted/missing messages.
        });

        Ok(())
    }
    /// Accept an incoming connection.
    fn accept(&self) -> Result<(), Error>
    {
        let mut buf = [0; 200];
        let (amt, src) = self.socket.recv_from(&mut buf)?;
        println!("Buf: {:?}\n", &buf[..]);
        println!("Read: {}\n", amt);
        println!("From: {:?}\n", src);

        let message: sctp_message::Message = deserialize(&buf[..]).unwrap();
        println!("Chunk amount: {:?}\n", message.chunks.len());
        println!("Chunk 0 type (should be init): {:?}", message.chunks[0].chunk_type);
 
        //This is an init message, so answer with an InitAck right away
        
        match message.chunks[0].data
        {
            sctp_message::MessageChunkData::Init { init_tag, .. } => {
                let mut init_ack_msg = sctp_message::Message::create_init_ack_msg(init_tag);
                self.socket.send_to(&serialize(&init_ack_msg, Infinite).unwrap(), src);
            }
            _ => {}
        }

        //This is cookie echo, so answer with cookie ack
        let (amt, src) = self.socket.recv_from(&mut buf)?;
        let message: sctp_message::Message = deserialize(&buf[..]).unwrap();
        println!("Chunk 0 type (should be cookie echo): {:?}", message.chunks[0].chunk_type);
        match message.chunks[0].data
        {
            sctp_message::MessageChunkData::CookieEcho {..} => {
                let mut cookie_ack_msg = sctp_message::Message::create_cookie_ack_msg();
                self.socket.send_to(&serialize(&cookie_ack_msg, Infinite).unwrap(), src);
            }
            _ => {}
        }

        Ok(())
    }
    /// Connects a client
    fn connect(&mut self, addr: SocketAddr) -> Result<(), Error>
    {
        let mut init_msg = sctp_message::Message::create_init_msg();
        self.socket.send_to(&serialize(&init_msg, Infinite).unwrap(), addr);
        self.sctp_conn.state = sctp_connection::SctpConnectionState::CookieWait;

        let mut buf = [0; 200];
        let (amt, src) = self.socket.recv_from(&mut buf)?;

        //TODO: We should initialize t1_init_timer in sctp_conn and start updating it.
        //This message should be init ack, so respond with cookie echo
        let message: sctp_message::Message = deserialize(&buf[..]).unwrap();
        println!("Chunk 0 type (should be init ack): {:?}", message.chunks[0].chunk_type);
        //TODO: Stop the t1_init_timer.
        match message.chunks[0].data
        {
            sctp_message::MessageChunkData::InitAck { ref state_cookie, .. } => {
                let mut cookie_echo_msg = sctp_message::Message::create_cookie_echo_msg(state_cookie);
                self.socket.send_to(&serialize(&cookie_echo_msg, Infinite).unwrap(), src);
            }
            _ => {}
        }

        //This sould be cookie ack, no need to respond anymore.
        let (amt, src) = self.socket.recv_from(&mut buf)?;
        let message: sctp_message::Message = deserialize(&buf[..]).unwrap();
        println!("Chunk 0 type (should be cookie ack): {:?}", message.chunks[0].chunk_type);


        Ok(())
    }
    /// Send data over the socket.
    fn send(&self) -> Result<(), &'static str>
    {
        Ok(())
    }
    /// Receive data over the socket.
    fn recv(&self) -> Result<(), &'static str>
    {
        Ok(())
    }
    /// Close the socket.
    fn close(&self) -> Result<(), &'static str>
    {
        Ok(())
    }

    fn set_nonblocking(&mut self, block: bool) -> Result<(), Error>
    {
        self.socket.set_nonblocking(block)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_server() {
        let sctp_over_udp_server = SctpOverUdpThreaded::new(SocketAddr::new(IpAddr::V4(<Ipv4Addr>::new(127, 0, 0, 1)), 34254));
        sctp_over_udp_server.listen();
        //sctp_over_udp_server.set_nonblocking(true);
        let result = sctp_over_udp_server.accept();
        print!("Server result: {:?}\n", result);
    }

    #[test]
    fn test_client() {
        let mut sctp_over_udp_client = SctpOverUdpThreaded::new(SocketAddr::new(IpAddr::V4(<Ipv4Addr>::new(0, 0, 0, 0)), 0));
        let result = sctp_over_udp_client.connect(SocketAddr::new(IpAddr::V4(<Ipv4Addr>::new(127, 0, 0, 1)), 34254));
        print!("Client result: {:?}\n", result);
    }

    #[test]
    fn create_udp_conn() {
        //let sctp_over_udp = SctpOverUdpThreaded::new();
    }
}