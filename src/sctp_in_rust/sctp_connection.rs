/// See: https://tools.ietf.org/html/rfc4960#page-52
enum SctpConnectionState {
    Closed,
    CookieWait,
    CookieEchoed,
    Established
}

pub struct SctpConnection
{
    state: SctpConnectionState,
}

impl SctpConnection
{
    pub fn new() -> SctpConnection {
        SctpConnection {
            state: SctpConnectionState::Closed
        }
    }
}