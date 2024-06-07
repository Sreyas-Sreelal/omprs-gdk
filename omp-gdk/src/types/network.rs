use std::{
    fmt::{Debug, Display},
    net::{Ipv4Addr, Ipv6Addr},
};

#[repr(C)]
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum PeerDisconnectReason {
    PeerDisconnectReasonTimeout,
    PeerDisconnectReasonQuit,
    PeerDisconnectReasonKicked,
    /// Never used, reserved for some existing libraries.
    PeerDisconnectReasonCustom,
    /// to match fixes because some other libraries already used it.
    PeerDisconnectReasonModeEnd,
}

#[repr(C)]
#[derive(PartialEq, Clone, Copy, Debug)]
pub struct NetworkStats {
    pub connectionStartTime: u32,
    pub messageSendBuffer: u32,
    pub messagesSent: u32,
    pub totalBytesSent: u32,
    pub acknowlegementsSent: u32,
    pub acknowlegementsPending: u32,
    pub messagesOnResendQueue: u32,
    pub messageResends: u32,
    pub messagesTotalBytesResent: u32,
    pub packetloss: f32,
    pub messagesReceived: u32,
    pub messagesReceivedPerSecond: u32,
    pub bytesReceived: u32,
    pub acknowlegementsReceived: u32,
    pub duplicateAcknowlegementsReceived: u32,
    pub bitsPerSecond: f64,
    pub bpsSent: f64,
    pub bpsReceived: f64,
    /// only for player network statistics
    pub isActive: bool,
    /// only for player network statistics
    pub connectMode: i32,
    pub connectionElapsedTime: u32,
}

#[repr(C)]
#[derive(PartialEq, Clone, Copy, Debug)]
struct IpV6Data {
    /// The IPv6 address segments
    segments: [u16; 8],
    /// The IPv6 address bytes
    bytes: [u8; 16],
}

#[repr(C)]
#[derive(Clone, Copy)]
union PeerAddresssData {
    v4: u32,
    v6: std::mem::ManuallyDrop<IpV6Data>,
}

impl PartialEq for PeerAddresssData {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            self.v4 == other.v4
                && self.v6.segments[2] == other.v6.segments[2]
                && self.v6.segments[3] == other.v6.segments[3]
                && self.v6.segments[4] == other.v6.segments[4]
                && self.v6.segments[5] == other.v6.segments[5]
                && self.v6.segments[6] == other.v6.segments[6]
                && self.v6.segments[7] == other.v6.segments[7]
        }
    }
}

impl Debug for PeerAddresssData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PeerAddresssData")
            .field("v4", unsafe { &self.v4 })
            .field("v6", unsafe { &self.v6 })
            .finish()
    }
}

#[repr(C)]
#[derive(PartialEq, Clone, Copy, Debug)]
pub struct PeerAddress {
    /// True if IPv6 is used, false otherwise
    ipv6: bool,
    data: PeerAddresssData,
}

impl Display for PeerAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.ipv6 {
            write!(f, "{}", Ipv6Addr::from(unsafe { self.data.v6.bytes }))
        } else {
            write!(f, "{}", Ipv4Addr::from(unsafe { self.data.v4.to_be() }))
        }
    }
}
/// Peer network ID
#[repr(C)]
#[derive(PartialEq, Clone, Copy, Debug)]
pub struct NetworkID {
    /// The peer's address
    pub address: PeerAddress,
    /// The peer's port
    pub port: u16,
}

impl Display for NetworkID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.address, self.port)
    }
}
