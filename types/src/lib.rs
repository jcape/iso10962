//! ISO 10962 Types for Classification of Financial Instruments

#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![no_std]

pub mod equity;

mod error;

pub use crate::error::{Error, Result};

/// A hierarchical enumeration of CFI Codes.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum Code {
    /// Equities.
    Equity(equity::Group) = b'E',
    /// Debt Instruments.
    DebtInstrument = b'D',
    /// Collective Investment Vehicles.
    CollectiveInvestmentVehicle = b'C',
    /// Entitlements (Rights).
    Entitlement = b'R',
    /// Listed Options.
    ListedOption = b'O',
    /// Futures Contracts.
    Future = b'F',
    /// Swaps.
    Swap = b'S',
    /// Unlisted and Complex Listed Options.
    UnlistedOption = b'H',
    /// Spots.
    Spot = b'I',
    /// Forwards
    Forward = b'J',
    /// Strategies
    Strategy = b'K',
    /// Financing
    Financing = b'L',
    /// Referential Instruments.
    Referential = b'T',
    /// Misc / Other Instruments.
    Misc = b'M',
}

impl Code {
    /// Whether this instance is an equity.
    #[must_use]
    pub const fn is_equity(&self) -> bool {
        matches!(self, Self::Equity(_))
    }
}

/// The form of the instrument.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "zerocopy",
    derive(
        zerocopy::Immutable,
        zerocopy::IntoBytes,
        zerocopy::KnownLayout,
        zerocopy::TryFromBytes,
        zerocopy::Unaligned,
    )
)]
#[repr(u8)]
pub enum Form {
    /// Bearer shares.
    Bearer = b'B',
    /// Registered shares.
    Registered = b'R',
    /// Bearer/Registered shares.
    BearerRegistered = b'N',
    /// Others (Misc).
    Other = b'M',
}

impl Form {
    /// The instance refers to bearer shares.
    #[must_use]
    pub const fn is_bearer(&self) -> bool {
        matches!(self, Self::Bearer)
    }

    /// The instance refers to registered shares.
    #[must_use]
    pub const fn is_registered(&self) -> bool {
        matches!(self, Self::Registered)
    }

    /// The instance refers to bearer/registered shares.
    #[must_use]
    pub const fn is_bearer_registered(&self) -> bool {
        matches!(self, Self::BearerRegistered)
    }

    /// The instance refers to miscelaneous shares.
    #[must_use]
    pub const fn is_other(&self) -> bool {
        matches!(self, Self::Other)
    }

    /// Construct a new instance from the given byte value.
    pub const fn from_byte(value: u8) -> Result<Self> {
        match value {
            b'B' => Ok(Self::Bearer),
            b'R' => Ok(Self::Registered),
            b'N' => Ok(Self::BearerRegistered),
            b'M' => Ok(Self::Other),
            _ => Err(Error::InvalidByte(value)),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[yare::parameterized(
        bearer = {Form::Bearer, Form::is_bearer},
        registered = {Form::Registered, Form::is_registered},
        bearer_registered = {Form::BearerRegistered, Form::is_bearer_registered},
        other = {Form::Other, Form::is_other},
    )]
    fn form_is(form: Form, func: fn(&Form) -> bool) {
        assert!(func(&form));
    }
}
