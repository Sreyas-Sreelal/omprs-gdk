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
