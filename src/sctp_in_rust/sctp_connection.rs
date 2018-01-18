/// See: https://tools.ietf.org/html/rfc4960#page-52
pub enum SctpConnectionState {
    Closed,
    CookieWait,
    CookieEchoed,
    Established
}

pub struct SctpConnection
{
    pub state: SctpConnectionState,
    t1_init_timer: u32,
    t1_cookie_timer: u32,
}

impl SctpConnection
{
    pub fn new() -> SctpConnection {
        SctpConnection {
            state: SctpConnectionState::Closed,
            t1_init_timer: 0,
            t1_cookie_timer: 0
        }
    }
}