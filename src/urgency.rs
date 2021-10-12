/// Specification of urgency of the sending of a message. Typically we'll want to send messages
/// on the next game tick, but the option to send messages immediately is available.
#[derive(Copy, Clone, Debug, PartialOrd, PartialEq, Eq)]
pub enum Urgency {
    /// Message will be sent based on the current configuration of the tick rate and
    /// the message send rate.
    OnTick,
    /// Message will be sent as soon as possible.
    Immediate,
}
