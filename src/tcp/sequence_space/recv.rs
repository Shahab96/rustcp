/// Receive Sequence Space (RFC 793 Section 3.2 Figure 5)
///
///```
///
///    1          2          3
///    ----------|----------|----------
///           RCV.NXT    RCV.NXT
///                     +RCV.WND
///
/// 1 - old sequence numbers which have been acknowledged
/// 2 - sequence numbers allowed for new reception
/// 3 - future sequence numbers which are not yet allowed
///```
pub struct ReceiveSequenceSpace {
    /// Receive next
    nxt: usize,

    /// Receive window
    wnd: usize,

    /// Receive urgent pointer
    up: bool,

    /// Initial receive sequence number
    irs: usize,
}

impl Default for ReceiveSequenceSpace {
    fn default() -> Self {
        Self {
            nxt: 0,
            wnd: 0,
            up: false,
            irs: 0,
        }
    }
}
