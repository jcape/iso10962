//! ISO 10962 Types for Classification of Financial Instruments

#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![no_std]

pub mod civ;
pub mod debt;
pub mod equities;
pub mod futures;
pub mod options;
pub mod rights;
pub mod swaps;

mod error;
mod macros;

pub use crate::error::{Error, Result};

/// The length of a CFI code, in bytes.
pub const CFI_LENGTH: usize = 6;

/// The byte index of the category character.
const CATEGORY_IDX: usize = 0;

/// The byte index of the group character.
const GROUP_IDX: usize = 1;

/// A trait implemented by CFI code attributes used to parse a code.
pub trait Attr: Sized {
    /// Attempt to parse the given ASCII byte as this attribute.
    ///
    /// # Errors
    ///
    /// - A specific value if a given character position contains an invald value.
    fn from_code_byte(value: u8) -> Result<Self>;
}

/// A trait implemented by CFI code attributes used to parse a code out of a byte slice.
pub trait AttrPos<const INDEX: usize>: Sized + Attr {
    /// Attempt to parse the given ASCII byte from the given 6-byte slice.
    ///
    /// # Errors
    ///
    /// - [`Error::InvalidLength`] if the length of the byte string is not [`CFI_LENGTH`].
    /// - A more specific error if a given character position contains an invalid value.
    #[inline]
    fn from_cfi_bytes(value: &[u8]) -> Result<Self> {
        if value.len() != CFI_LENGTH {
            return Err(Error::InvalidLength);
        }

        Self::from_code_byte(value[INDEX])
    }
}

/// A CFI group is a collection of attributes.
pub trait CfiGroup: Sized {
    /// The type of the first attribute.
    type Attr1: AttrPos<2>;

    /// The type of the second attribute.
    type Attr2: AttrPos<3>;

    /// The type of the third attribute.
    type Attr3: AttrPos<4>;

    /// The type of the fourth attribute.
    type Attr4: AttrPos<5>;

    /// Retrieve the first attribute.
    fn attr1(&self) -> Self::Attr1;

    /// Retrieve the second attribute.
    fn attr2(&self) -> Self::Attr2;

    /// Retrieve the third attribute.
    fn attr3(&self) -> Self::Attr3;

    /// Retrieve the fourth attribute.
    fn attr4(&self) -> Self::Attr4;

    /// Parse the given value into bytes.
    ///
    /// # Errors
    ///
    /// - [`Error::InvalidLength`] if the length of the byte string is not [`CFI_LENGTH`].
    /// - A more specific error if a given character position contains an invalid value.
    fn from_cfi_bytes(value: &[u8]) -> Result<Self>;
}

/// A hierarchical enumeration of CFI Codes.
///
/// # Examples
///
/// ```rust
/// use iso10962_types::{CFI_LENGTH, Code};
/// use std::mem;
///
/// assert_eq!(CFI_LENGTH, mem::size_of::<Code>());
/// ```
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum Code {
    /// `E`: Equities.
    ///
    /// Financial instruments representing an ownership interest in an entity or pool of assets.
    Equity(equities::Equity) = b'E',

    /// `D`: Debt instruments.
    ///
    /// Financial instruments evidencing monies owed by the issuer to the holder on terms as
    /// specified.
    Debt(debt::Debt) = b'D',

    /// `C`: Collective investment vehicles.
    ///
    /// Securities representing a portion of assets pooled by investors run by a management company
    /// whose share capital remains separate from such assets and includes issues of shares or
    /// units in the form of, for example, a unit trust, mutual fund, OICVM, OPCVM, SICAV or SICAF.
    Civ(civ::Civ) = b'C',

    /// `R`: Entitlement (rights).
    ///
    /// Financial instruments providing the holder with the privilege to subscribe to or receive
    /// specific assets on terms specified.
    Right(rights::Right) = b'R',

    /// `O`: Listed options.
    ///
    /// This Category classifies listed options, which are contracts that grant to the holder
    /// either the privilege to purchase or the privilege to sell the assets specified at a
    /// predetermined price or formula at or within a time in the future. Where a listed option
    /// cannot be classified within this Category, refer to non-listed and complex listed options.
    ListedOption(options::Listed) = b'O',

    /// `F`: Futures.
    ///
    /// Contracts, listed on an exchange or regulated market, which obligate the buyer to receive
    /// and the seller to deliver in the future the assets specified at an agreed price. This
    /// includes forwards on regulated markets.
    Future(futures::Future) = b'F',

    /// `S`: Swaps.
    ///
    /// A swap is an agreement or contract where two counterparties agree to exchange periodic
    /// streams of cash flows with each other. Swaps can be executed with a variety of asset
    /// classes, as listed below.
    Swap(swaps::Swap) = b'S',

    /// `H`: Non-listed and complex listed options.
    ///
    /// This category includes OTC or unlisted options and also includes any listed option which is
    /// not captured by the listed options category. An option grants the holder either the
    /// privilege to purchase or the privilege to sell the assets specified at a predetermined
    /// price or formula at or within a time in the future.
    UnlistedOption(()) = b'H',

    /// `I`: Spot.
    ///
    /// Contracts conducted on the spot market which are bought and sold for cash with immediate
    /// delivery based on market convention for the asset.
    Spot(()) = b'I',

    /// `J`: Forwards.
    ///
    /// Contracts, which are not exchange traded or listed, entered between two parties to buy or
    /// sell the underlying asset at a specified future date at the price specified at the outset
    /// of the contract.
    Forward(()) = b'J',

    /// `K`: Strategies.
    ///
    /// This subclause defines a classification of derivative strategies. Strategies are the
    /// simultaneous trading of two or more derivative instruments.
    Strategy(()) = b'K',

    /// `L`: Financing.
    ///
    /// Financing is a collateralized loan agreement entered into between two parties where one
    /// party, the lender, lends (temporarily) the underlying asset which is secured with cash or
    /// other acceptable collateral (securities or other assets) provided by the borrower.
    /// Depending on the exact type of financing transaction, a simultaneous agreement to reverse
    /// the agreement may be entered into at the same time with an agreed-upon future date for the
    /// reverse transaction to take place.
    Financing(()) = b'L',

    /// `T`: Referential Instruments.
    ///
    /// Indicators that are used as a reference for other financial instruments.
    Referential(()) = b'T',

    /// `M`: Misc / Other Instruments.
    ///
    /// Financial instruments that do not fit the above categories as defined.
    Misc(()) = b'M',
}

impl Code {
    /// Whether this instance is an equity.
    #[inline]
    #[must_use]
    pub const fn is_equity(&self) -> bool {
        matches!(self, Self::Equity(_))
    }

    /// Whether this instance is a debt instrument.
    #[inline]
    #[must_use]
    pub const fn is_debt(&self) -> bool {
        matches!(self, Self::Debt(_))
    }

    /// Whether this instance is a collective investment vehicle.
    #[inline]
    #[must_use]
    pub const fn is_civ(&self) -> bool {
        matches!(self, Self::Civ(_))
    }

    /// Whether this instance is a entitlement/right.
    #[inline]
    #[must_use]
    pub const fn is_entitlement(&self) -> bool {
        matches!(self, Self::Right(_))
    }

    /// Whether this instance is a listed option.
    #[inline]
    #[must_use]
    pub const fn is_listed_option(&self) -> bool {
        matches!(self, Self::ListedOption(_))
    }

    /// Whether this instance is a futures contract.
    #[inline]
    #[must_use]
    pub const fn is_future(&self) -> bool {
        matches!(self, Self::Future(_))
    }

    /// Whether this instance is a swap.
    #[inline]
    #[must_use]
    pub const fn is_swap(&self) -> bool {
        matches!(self, Self::Swap(_))
    }

    /// Whether this instance is a non-listed or complex listed option.
    #[inline]
    #[must_use]
    pub const fn is_unlisted_option(&self) -> bool {
        matches!(self, Self::UnlistedOption(()))
    }

    /// Whether this instance is a spot contract.
    #[inline]
    #[must_use]
    pub const fn is_spot(&self) -> bool {
        matches!(self, Self::Spot(()))
    }

    /// Whether this instance is a forward contract.
    #[inline]
    #[must_use]
    pub const fn is_forward(&self) -> bool {
        matches!(self, Self::Forward(()))
    }

    /// Whether this instance is a derivative strategy.
    #[inline]
    #[must_use]
    pub const fn is_strategy(&self) -> bool {
        matches!(self, Self::Strategy(()))
    }

    /// Whether this instance is a financing agreement.
    #[inline]
    #[must_use]
    pub const fn is_financing(&self) -> bool {
        matches!(self, Self::Financing(()))
    }

    /// Whether this instance is a referential instrument.
    #[inline]
    #[must_use]
    pub const fn is_referential(&self) -> bool {
        matches!(self, Self::Referential(()))
    }

    /// Whether this instance does not fit the above categories.
    #[inline]
    #[must_use]
    pub const fn is_misc(&self) -> bool {
        matches!(self, Self::Misc(()))
    }

    /// Parse the given byte slice into a code.
    ///
    /// # Errors
    ///
    /// - [`Error::InvalidLength`] if the bytes given are not [`CFI_LENGTH`].
    #[inline]
    pub const fn from_bytes(src: &[u8]) -> Result<Self> {
        if src.len() != CFI_LENGTH {
            return Err(Error::InvalidLength);
        }

        match src[CATEGORY_IDX] {
            b'E' => match equities::Equity::from_bytes(src) {
                Ok(value) => Ok(Self::Equity(value)),
                Err(error) => Err(error),
            },
            other => Err(Error::InvalidCategory(other as char)),
        }
    }
}

macros::impl_attr! {
    /// Form (negotiability, transmission).
    pub enum Form[5] {
        /// Bearer (the owner is not registered in the books of the issuer or of the
        /// registrar).
        Bearer = b'B', "B";

        /// Registered (securities are recorded in the name of the owner on the books of the
        /// issuer or the issuer's registrar and can only be transferred to another owner when
        /// endorsed by the registered owner).
        Registered = b'R', "R";

        /// Bearer/registered (securities are issued in both bearer and registered form but
        /// with the same identification number).
        BearerRegistered = b'N', "N";

        /// Others (miscellaneous).
        Other = b'M', "M";
    }
}

macros::impl_attr! {
    /// Not applicable/undefined.
    pub enum NotApplicable[2, 3, 4, 5] {}
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

macros::impl_attr! {
    /// Standardized/non-standardized.
    ///
    /// Indicates whether the terms of the contract are standardized or not.
    pub enum Standardized[4, 5] {
        /// Standardized (the underlying instruments, exercise price, expiration date and contract
        /// size of the options are standardized; these options are traded on special option
        /// exchanges).
        Standardized = b'S', "S";

        /// Non-standardized (options traded on option exchanges which have non-standard delivery
        /// or expiry terms).
        NonStandardized = b'N', "N";
    }
}
