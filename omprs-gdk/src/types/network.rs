#[repr(C)]
pub enum PeerDisconnectReason {
    PeerDisconnectReasonTimeout,
    PeerDisconnectReasonQuit,
    PeerDisconnectReasonKicked,
    // Never used by us, reserved for some existing libraries.
    PeerDisconnectReasonCustom,
    // 4 to match fixes.inc, which wasn't 3 because some other libraries already used it.
    PeerDisconnectReasonModeEnd,
}
