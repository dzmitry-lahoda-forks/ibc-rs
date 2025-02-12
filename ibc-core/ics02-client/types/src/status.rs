use core::fmt::{Debug, Display, Formatter};

/// `UpdateKind` represents the 2 ways that a client can be updated
/// in IBC: either through a `MsgUpdateClient`, or a `MsgSubmitMisbehaviour`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum UpdateKind {
    /// this is the typical scenario where a new header is submitted to the client
    /// to update the client. Note that light clients are free to define the type
    /// of the object used to update them (e.g. could be a list of headers).
    UpdateClient,
    /// this is the scenario where misbehaviour is submitted to the client
    /// (e.g 2 headers with the same height in Tendermint)
    SubmitMisbehaviour,
}

/// Represents the status of a client
#[derive(Debug, PartialEq, Eq)]
pub enum Status {
    /// The client is active and allowed to be used
    Active,
    /// The client is frozen and not allowed to be used
    Frozen,
    /// The client is expired and not allowed to be used
    Expired,
    /// Unauthorized indicates that the client type is not registered as an allowed client type.
    Unauthorized,
}

impl Status {
    pub fn is_active(&self) -> bool {
        *self == Status::Active
    }

    pub fn is_frozen(&self) -> bool {
        *self == Status::Frozen
    }

    pub fn is_expired(&self) -> bool {
        *self == Status::Expired
    }
}

impl Display for Status {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{self:?}")
    }
}
