mod native;

#[cfg(target_os = "windows")]
use native::sctp_windows as sctp;
#[cfg(target_os = "linux")]
use native::sctp_linux as sctp;

// TODO: Wrap the return value to a Result and socket's data type should be an Enum.
pub fn bind_stcp_socket(socket: u64, ip: u64, port: u64) -> i32
{
    // TODO: Fix. Need to create wrapper data types.
    //sctp::bind_stcp_socket()
    0
}

// TODO: Wrap the return value to a Result.
pub fn create_stcp_socket() -> u64
{
    sctp::create_stcp_socket()
}

// TODO: Wrap the return value to a Result.
pub fn open_and_bind_socket()
{
    sctp::open_and_bind_socket();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        open_and_bind_socket();
        //println!("Family address: {}", get_family_address_size(4));
    }
}
//2