/*
    sctp_bindx
    sctp_connectx
    sctp_freepaddrs
    sctp_freeladdrs
    sctp_getaddrlen
    sctp_getassocid
    sctp_getpaddrs
    sctp_getladdrs
    sctp_opt_info
    sctp_peeloff
    sctp_recvmsg
    sctp_send
    sctp_sendx
    sctp_sendmsg
    sctp_sendmsgx
*/
use std::net::{SocketAddr};
use std::io::Error;

pub enum IPV {
    IPV4,
    IVP6
}

/// This is a loose adoption of the common one-to-one sockets API, provided to keep the use of this library familiar.
/// Note that some things are done in a SCTP- or Rust-specific manner and as such function signatures differ from
/// both the Unix and Windows API. I hope that rather than being an inconvenience, this makes using the library more fluent.
pub trait SocketsApi
{
    fn socket(&self, ipv: IPV) -> Result<(), &'static str>;
    /// Binds the socket to an address. You can give either an ipv6 or ipv4 address.
    fn bind(&mut self, addr: SocketAddr) -> Result<(), Error>;
    /// Starts to listen for connections. Will be unimplemented on the non-native, non-threaded API.
    /// On the threaded UDP API this will start the background threads.
    /// On the native implentation, it will map to native listen() call.
    fn listen(&self) -> Result<(), &'static str>;
    /// Connects a client
    fn connect(&self) -> Result<(), &'static str>;
    /// Accept an incoming connection.
    fn accept(&self) -> Result<(), &'static str>;
    /// Send data over the socket.
    fn send(&self) -> Result<(), &'static str>;
    /// Receive data over the socket.
    fn recv(&self) -> Result<(), &'static str>;
    /// Close the socket.
    fn close(&self) -> Result<(), &'static str>;

    fn set_nonblocking(&mut self, block: bool) -> Result<(), Error>;
}