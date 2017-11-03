/// SCTP message, composite type made of header + N many data chunks + state data
pub struct Message {
    header: MessageHeader,
    chunks: Vec<MessageChunk>
}

impl Message {
    pub fn new() -> Message {
        Message {
            header: MessageHeader::new(),
            chunks: Vec::new()
        }
    }
}

/// SCTP message header. https://en.wikipedia.org/wiki/SCTP_packet_structure#Common_header
/// Note: src_port and dst_port might not be necessary to serialize if UDP is used, as the UDP header replicates them.
pub struct MessageHeader {
    src_port: u16,
    dst_port: u16,

    verification: u32,
    checksum: u32
}

impl MessageHeader {
    pub fn new() -> MessageHeader {
        MessageHeader {
            src_port: 9899,
            dst_port: 9899,

            verification: 0,
            checksum: 0,
        }
    }
}

/// SCTP chunk types. https://en.wikipedia.org/wiki/SCTP_packet_structure#List_of_chunk_types
/// Note: Length might not be necessary to serialize if UDP is used since the UDP header replicates it.
pub struct MessageChunk {
    chunk_type: u8,
    flags: u8,
    length: u16,

    data: MessageChunkData
}

enum MessageChunkData {
     Data {
         tsn: u32,
         stream_id: u16,
         seq_number: u16,
         protocol_id: u32
    },
    Init {
        init_tag: u32,
        /// "Advertised receiver window credit - Amount of dedicated buffer space for this association that should never be reduced"
        a_rwnd: u32,
        /// Number of inbound streams this association is allowed to use.
        out_streams_n: u16,
        /// Number of outbound streams this association is allowed to use.
        in_streams_n: u16,
        /// "Initial TSN - Initial transmission sequence number to be used and may be any value"
        init_tsn: u32,
    },
    /// Init acknowldgement
    InitAck {
        init_tag: u32,
        /// "Advertised receiver window credit - Amount of dedicated buffer space for this association that should never be reduced"
        a_rwnd: u32,
        /// Number of inbound streams this association is allowed to use.
        out_streams_n: u16,
        /// Number of outbound streams this association is allowed to use.
        in_streams_n: u16,
        /// "Initial TSN - Initial transmission sequence number to be used and may be any value"
        init_tsn: u32,
        /// State cookie for recreating Transmission Control Block. Used for security purposes & can be signed with a private key.
        state_cookie: Vec<u8>
    },
    /// Echo the cookie sent by InitAck
    CookieEcho {
        state_cookie: Vec<u8>
    },
    /// Cookie acknowledgement
    CookieAck {

    }
}

impl MessageChunk {
    pub fn serialize() -> Vec<u8>
    {
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_data_payload_message() {
        let message: Message = Message::new();
    }
}