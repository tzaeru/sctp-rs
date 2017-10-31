extern crate libc;
extern crate ws2_32;
extern crate winapi;
extern crate byteorder;

use std::ffi::CString;
use std::ptr;
use std::mem;
use std::net;

//#[cfg(all(target_os = "win32", target_arch = "x64"))]
#[link(name = "sctpsp")]
extern "C" {
    fn internal_sctp_getaddrlen(family: libc::c_int) -> libc::c_int;
    fn internal_sctp_bindx(socket: u64, address: *const winapi::ws2def::SOCKADDR, num: u64, con_type: u64) -> libc::c_int;
}

pub fn get_family_address_size(family: i32) -> i32
{
    unsafe { internal_sctp_getaddrlen(family) }
}

pub fn bind_stcp_socket(socket: winapi::SOCKET, address: winapi::ws2def::SOCKADDR_IN) -> i32
{
    println!("Socket 2: {:?}", socket);
    unsafe {
    internal_sctp_bindx(socket,
        &address as *const _ as *const _,
        1,
        0x00008001)
    }
}

pub fn create_stcp_socket() -> winapi::winsock2::SOCKET
{
    unsafe { ws2_32::socket(2, 1, 132) }
}

pub fn socket_addr_ipv4() -> winapi::ws2def::SOCKADDR_IN
{
    let addr_bytes = [127, 0, 0, 1];
    let mut addr_buf = &addr_bytes[..];
    let in_addr = winapi::in_addr{S_un: u32::from(net::Ipv4Addr::new(127, 0, 0, 1)).to_be()};

    let sockaddr_in = winapi::ws2def::SOCKADDR_IN {sin_family: winapi::ws2def::AF_INET as u16,
        sin_addr: in_addr,
        sin_port: 7777,
        sin_zero: unsafe { mem::zeroed() }
    };

    return sockaddr_in

    //ws2_32::socket(0, 0, 0)
    //unsafe { internal_sctp_bindx(socket)}
}

pub fn open_and_bind_socket()
{
    unsafe {
        let mut data = mem::zeroed();
        let start_err = ws2_32::WSAStartup(winapi::minwindef::MAKEWORD(2, 2), &mut data);
        let addr = socket_addr_ipv4();
        let socket = create_stcp_socket();
        println!("Socket created: {:?}", socket);
        println!("Win err: {:?}", ws2_32::WSAGetLastError());
        let err = bind_stcp_socket(socket, addr);
        println!("STCP error: {:?}", err);
        println!("Win err: {:?}", ws2_32::WSAGetLastError());
    }
}