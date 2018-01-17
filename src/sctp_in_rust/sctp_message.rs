use bincode::{serialize, deserialize, Infinite};
use rand;
use rand::Rng;

/// SCTP message, composite type made of header + N many data chunks + state data
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Message {
    pub header: MessageHeader,
    pub chunks: Vec<MessageChunk>
}

impl Message {
    pub fn new() -> Message {
        Message {
            header: MessageHeader::new(),
            chunks: Vec::new()
        }
    }

    pub fn add_chunk(&mut self, chunk: MessageChunk)
    {
        self.chunks.push(chunk);
    }

    pub fn create_init_msg() -> Message {
        let mut rng = rand::thread_rng();

        let chunk_data = MessageChunkData::Init {
            init_tag: rng.gen::<u32>(),
            a_rwnd: 0,
            out_streams_n: 0,
            in_streams_n: 0,
            init_tsn: 0
        };

        let message_chunk = MessageChunk {
            chunk_type: 1,
            flags: 0,
            length: 4 + 4 + 2 + 2 + 4,

            data: chunk_data
        };

        let mut message = Message::new();
        message.add_chunk(message_chunk);

        message
    }

    pub fn create_init_ack_msg(verification: u32) -> Message {
        let mut rng = rand::thread_rng();

        let chunk_data = MessageChunkData::InitAck {
            init_tag: rng.gen::<u32>(),
            a_rwnd: 0,
            out_streams_n: 0,
            in_streams_n: 0,
            init_tsn: 0,
            state_cookie: Vec::new()
        };

        let message_chunk = MessageChunk {
            chunk_type: 1,
            flags: 0,
            length: 4 + 4 + 2 + 2 + 4,

            data: chunk_data
        };

        let mut message = Message::new();
        message.add_chunk(message_chunk);
        message.header.verification = verification;

        message
    }
}

/// SCTP message header. https://en.wikipedia.org/wiki/SCTP_packet_structure#Common_header
/// Note: src_port and dst_port might not be necessary to serialize if UDP is used, as the UDP header replicates them.
#[derive(Serialize, Deserialize, PartialEq, Debug)]
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
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct MessageChunk {
    pub chunk_type: u8,
    flags: u8,
    length: u16,

    pub data: MessageChunkData
}

impl MessageChunk
{

}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum MessageChunkData {
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

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_data_payload_message() {
        let message: Message = Message::new();
    }
}