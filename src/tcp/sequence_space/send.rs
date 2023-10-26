/// Send Sequence Space (RFC 793 Section 3.2 Figure 4)
///
///```
/// 1         2          3          4
/// ----------|----------|----------|----------
///   SND.UNA    SND.NXT    SND.UNA
///                        +SND.WND
///
/// 1 - old sequence numbers which have been acknowledged
/// 2 - sequence numbers of unacknowledged data
/// 3 - sequence numbers allowed for new data transmission
/// 4 - future sequence numbers which are not yet allowed
///```
pub struct SendSequenceSpace {
    /// Send unacknowledged
    una: usize,

    /// Send next
    nxt: usize,

    /// Send window
    wnd: usize,

    /// Send urgent pointer
    up: bool,

    /// Segment sequence number used for last window update
    wl1: usize,

    /// Segment acknowledgment number used for last window update
    wl2: usize,

    /// Initial send sequence number
    iss: usize,
}

impl Default for SendSequenceSpace {
    fn default() -> Self {
        Self {
            una: 0,
            nxt: 0,
            wnd: 0,
            up: false,
            wl1: 0,
            wl2: 0,
            iss: 0,
        }
    }
}
