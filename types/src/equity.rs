//! Equities-category CFI details.

use crate::Form;

/// CFI Groups for Equities.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum Group {
    /// Common/ordinary shares.
    Common(CommonAttributes) = b'S',
    /// Preferred shares
    Preferred(PreferredAttributes) = b'P',
    /// Convertible shares.
    Convertible(ConvertibleAttributes) = b'C',
    /// Preferred convertible shares.
    PreferedConvertible(PreferredConvertibleAttributes) = b'F',
    /// Limited partnership units.
    LlpUnit(LlpAttributes) = b'L',
    /// Depository receipts on equities.
    DepositoryReceipt(ReceiptAttributes) = b'D',
    /// Structured instruments (participation).
    Structured(StructuredAttributes) = b'Y',
    /// Preference shares.
    Preference(PreferenceAttributes) = b'R',
    /// Preference convertible shares.
    PreferenceConvertible(PreferenceConvertibleAttributes) = b'V',
    /// Units (from Unit trusts, Mutual funds, OPCVM or OICVM).
    Unit(UnitAttributes) = b'U',
    /// Others (misc).
    Other(OtherAttributes) = b'M',
}

impl Group {
    /// If the group is common/ordinary shares.
    #[must_use]
    pub const fn is_common(&self) -> bool {
        matches!(self, Group::Common(_))
    }

    /// If the group is preferred shares.
    #[must_use]
    pub const fn is_preferred(&self) -> bool {
        matches!(self, Group::Preferred(_))
    }

    /// If the group is convertible shares.
    #[must_use]
    pub const fn is_convertible(&self) -> bool {
        matches!(self, Group::Convertible(_))
    }

    /// If the group is convertible shares.
    #[must_use]
    pub const fn is_prefered_convertible(&self) -> bool {
        matches!(self, Group::PreferedConvertible(_))
    }

    /// If the group is a limited partnership unit.
    #[must_use]
    pub const fn is_llp_unit(&self) -> bool {
        matches!(self, Group::LlpUnit(_))
    }

    /// If the group is a depository receipt.
    #[must_use]
    pub const fn is_depository_receipt(&self) -> bool {
        matches!(self, Group::DepositoryReceipt(_))
    }

    /// If the group is a structured instrument (participation).
    #[must_use]
    pub const fn is_structured(&self) -> bool {
        matches!(self, Group::Structured(_))
    }

    /// If the group is a preference share.
    #[must_use]
    pub const fn is_preference(&self) -> bool {
        matches!(self, Group::Preference(_))
    }

    /// If the group is a preference share.
    #[must_use]
    pub const fn is_preference_convertible(&self) -> bool {
        matches!(self, Group::PreferenceConvertible(_))
    }

    /// If the group is a unit (from a unit trust, mutual fund, OPCVM, or OICVM).
    #[must_use]
    pub const fn is_unit(&self) -> bool {
        matches!(self, Group::Unit(_))
    }

    /// If the group is a miscellaneous share.
    #[must_use]
    pub const fn is_other(&self) -> bool {
        matches!(self, Group::Other(_))
    }
}

/// The attributes available to common equities.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "zerocopy",
    derive(
        zerocopy::Immutable,
        zerocopy::IntoBytes,
        zerocopy::KnownLayout,
        zerocopy::TryFromBytes,
    )
)]
pub struct CommonAttributes {
    /// Voting rights (`ES_XXX`).
    pub voting_right: VotingRight,
    /// Ownership (`ESX_XX`).
    pub ownership: Ownership,
    /// Payment status (`ESXX_X`).
    pub payment_status: PaymentStatus,
    /// The form of equity (ESXXX_).
    pub form: Form,
}

/// The attributes available to preferred equities.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "zerocopy",
    derive(
        zerocopy::Immutable,
        zerocopy::IntoBytes,
        zerocopy::KnownLayout,
        zerocopy::TryFromBytes,
    )
)]
pub struct PreferredAttributes {
    /// Voting rights (`EP_XXX`).
    pub voting_right: VotingRight,
    /// Redemption (`EPX_XX`).
    pub redemption: Redemption,
    /// Income (`EPXX_X`).
    pub income: Income,
    /// The form of share (EPXXX_).
    pub form: Form,
}

/// The attributes available to convertible equities.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "zerocopy",
    derive(
        zerocopy::Immutable,
        zerocopy::IntoBytes,
        zerocopy::KnownLayout,
        zerocopy::TryFromBytes,
    )
)]
pub struct ConvertibleAttributes {
    /// Voting rights (`EC_XXX`).
    pub voting_right: VotingRight,
    /// Ownership (`ECX_XXX`).
    pub ownership: Ownership,
    /// Payment status (`ECXX_X`).
    pub payment_status: PaymentStatus,
    /// The form of share (ECXXX_).
    pub form: Form,
}

/// The attributes available to convertible preferred equities.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "zerocopy",
    derive(
        zerocopy::Immutable,
        zerocopy::IntoBytes,
        zerocopy::KnownLayout,
        zerocopy::TryFromBytes,
    )
)]
pub struct PreferredConvertibleAttributes {
    /// Voting rights (`EF_XXX`).
    pub voting_right: VotingRight,
    /// Redemption (`EFX_XXX`).
    pub redemption: Redemption,
    /// Income (`EFXX_X`).
    pub income: Income,
    /// The form of share (EFXXX_).
    pub form: Form,
}

/// The attributes available to limited partnership units.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "zerocopy",
    derive(
        zerocopy::Immutable,
        zerocopy::IntoBytes,
        zerocopy::KnownLayout,
        zerocopy::TryFromBytes,
    )
)]
pub struct LlpAttributes {
    /// Voting rights (`EL_XXX`).
    pub voting_right: VotingRight,
    /// Ownership (`ELX_XXX`).
    pub ownership: Ownership,
    /// Payment status (`ELXX_XX`).
    pub payment_status: PaymentStatus,
    /// The form of unit (ELXXX_).
    pub form: Form,
}

/// The attributes available to common equities.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "zerocopy",
    derive(
        zerocopy::Immutable,
        zerocopy::IntoBytes,
        zerocopy::KnownLayout,
        zerocopy::TryFromBytes,
    )
)]
pub struct ReceiptAttributes {
    /// Instrument dependency (`ED_XXX`).
    pub dependency: Dependency,
    /// Redemption/Conversion style (`EDX_XXX`).
    pub redemption: RedemptionConversion,
    /// Income (`EDXX_XX`)
    pub income: Income,
    /// The form of the receipt (EDXXX_).
    pub form: Form,
}

/// The attributes available to structured equities.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "zerocopy",
    derive(
        zerocopy::Immutable,
        zerocopy::IntoBytes,
        zerocopy::KnownLayout,
        zerocopy::TryFromBytes,
    )
)]
pub struct StructuredAttributes {
    /// Structured instrument type (`ER_XXX`)
    pub kind: Kind,
    /// Distribution style (`ERX_XX`)
    pub distribution: Distribution,
    /// Repayment style (`ERXX_X`)
    pub repayment: Repayment,
    /// Underlying asset type (ERXXX_)
    pub underlying: Underlying,
}

/// The attributes available to preference equities.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "zerocopy",
    derive(
        zerocopy::Immutable,
        zerocopy::IntoBytes,
        zerocopy::KnownLayout,
        zerocopy::TryFromBytes,
    )
)]
pub struct PreferenceAttributes {
    /// Voting rights (`ER_XXX`).
    pub voting_right: VotingRight,
    /// Redemption (`ERX_XX`).
    pub redemption: Redemption,
    /// Income (`ERXX_X`).
    pub income: Income,
    /// The form of the receipt (ERXXX_)
    pub form: Form,
}

/// The attributes available to preference equities.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "zerocopy",
    derive(
        zerocopy::Immutable,
        zerocopy::IntoBytes,
        zerocopy::KnownLayout,
        zerocopy::TryFromBytes,
    )
)]
pub struct PreferenceConvertibleAttributes {
    /// Voting rights (`EV_XXX`).
    pub voting_right: VotingRight,
    /// Redemption (`EVX_XX`).
    pub redemption: Redemption,
    /// Income (`EVXX_X`).
    pub income: Income,
    /// The form of the receipt (EVXXX_)
    pub form: Form,
}

/// The attributes available to preference equities.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "zerocopy",
    derive(
        zerocopy::Immutable,
        zerocopy::IntoBytes,
        zerocopy::KnownLayout,
        zerocopy::TryFromBytes,
    )
)]
pub struct UnitAttributes {
    /// Closed/open end (`EU_XXX`).
    pub open_closed: OpenClosed,
    /// Redemption (`EUX_XX`).
    pub distribution: DistributionPolicy,
    /// Income (`EUXX_X`).
    pub assets: Asset,
    /// The form of the receipt (EVXXX_)
    pub form: UnitForm,
}

/// The attributes available to other equities.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "zerocopy",
    derive(
        zerocopy::Immutable,
        zerocopy::IntoBytes,
        zerocopy::KnownLayout,
        zerocopy::TryFromBytes,
    )
)]
pub struct OtherAttributes {
    /// An un-used attribute.
    pub attr1: NotApplicable,
    /// An un-used attribute.
    pub attr2: NotApplicable,
    /// An un-used attribute.
    pub attr3: NotApplicable,
    /// The form of this equity.
    pub form: Form,
}

/// The attributes available to other equities.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
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
pub enum NotApplicable {
    /// Not applicable / undefined.
    #[default]
    Undefined = b'X',
}

/// The attributes available to common equities.
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
pub enum VotingRight {
    /// The shares are voting.
    Voting = b'V',
    /// The shares are non-voting.
    NonVoting = b'N',
    /// The shares are restricted-voting.
    Restricted = b'R',
    /// The shares have enhanced voting.
    Enhanced = b'E',
}

/// The attributes available to common equities.
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
pub enum Ownership {
    /// There are restrictions on ownership.
    Restrictions = b'T',
    /// There are no restrictions.
    Free = b'U',
}

/// The payment status.
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
pub enum PaymentStatus {
    /// Fully paid.
    Fully = b'F',
    /// Nil paid.
    Nil = b'O',
    /// Partially paid.
    Partial = b'P',
}

/// The redemption style.
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
pub enum Redemption {
    /// Redeemable.
    Redeemable = b'R',
    /// Extendible.
    Extendible = b'E',
    /// Redeemable / Extendible.
    RedeemableExtendible = b'T',
    /// Exchangeable.
    Exchangeable = b'G',
    /// Redeemable / Exchangeable / Extendible.
    RedeemableExchangeableExtendible = b'A',
    /// Redeemable / Exchangeable.
    RedeemableExchangeable = b'C',
    /// Perpetual.
    Perpetual = b'N',
}

/// The redemption style.
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
pub enum Income {
    /// Fixed Rate.
    FixedRate = b'F',
    /// Cumulative, Fixed Rate.
    CumulativeFixedRate = b'C',
    /// Participating.
    Participating = b'P',
    /// Cumulative, Participating.
    CumulativeParticipating = b'Q',
    /// Adjustable/Variable Rate.
    AdjustableRate = b'A',
    /// Normal Rate.
    NormalRate = b'N',
    /// Auction Rate.
    AuctionRate = b'U',
    /// Dividends.
    Dividends = b'D',
}

/// The instrument dependency.
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
pub enum Dependency {
    /// Common/Ordinary shares.
    Common = b'S',
    /// Preferred/Preference shares.
    PreferredPreference = b'P',
    /// Common/Ordinary convertible shares.
    CommonConvertible = b'C',
    /// Preferred/Preference convertible shares.
    PreferredPreferenceConvertible = b'F',
    /// Limited partnership units.
    LlpUnit = b'L',
    /// Other (misc).
    Other = b'M',
}

/// The redemption conversion type.
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
pub enum RedemptionConversion {
    /// Redeemable.
    Redeemable = b'R',
    /// Perpetual.
    Perpetual = b'N',
    /// Convertible.
    Convertible = b'B',
    /// Convertible/Redeemable.
    ConvertibleRedeemable = b'D',
    /// Not Applicable/Undefined.
    Undefined = b'X',
}

/// The structured instrument type.
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
pub enum Kind {
    /// Tracker certificate.
    Tracker = b'A',
    /// Outperforming certificate.
    Outperforming = b'B',
    /// Bonus certificate.
    Bonus = b'C',
    /// Outperformance bonus certificate.
    OutperformanceBonus = b'D',
    /// Twin-Win certificate.
    TwinWin = b'E',
    /// Other (misc).
    Other = b'M',
}

/// The structured instrument type.
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
pub enum Distribution {
    /// Divident payments.
    Dividend = b'D',
    /// No payments.
    None = b'Y',
    /// Others (misc).
    Other = b'M',
}

/// The structured instrument type.
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
pub enum Repayment {
    /// Cash repayment.
    Cash,
    /// Physical repayment.
    Physical,
    /// Elect at settlement.
    Elect,
    /// Others (misc).
    Other = b'M',
}

/// The underlier of a structured instrument.
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
pub enum Underlying {
    /// Baskets.
    Baskets = b'B',
    /// Equities.
    Equities = b'S',
    /// Debt instruments.
    Debt = b'D',
    /// Derivatives.
    Derivatives = b'G',
    /// Commodities.
    Commodities = b'T',
    /// Currencies.
    Currencies = b'C',
    /// Indices.
    Indices = b'I',
    /// Interest rates.
    Rates = b'N',
    /// Others (misc).
    Other = b'M',
}

/// The underlier of a structured instrument.
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
pub enum DistributionPolicy {
    /// Income funds.
    Income = b'I',
    /// Growth funds.
    Growth = b'G',
    /// Mixed funds.
    Mixed = b'M',
}

/// The closed/open-end status of the units.
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
pub enum OpenClosed {
    /// Closed-end.
    Closed = b'C',
    /// Open-end.
    Open = b'O',
}

/// The assets of the units.
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
pub enum Asset {
    /// Real-estate.
    RealEstate = b'R',
    /// Securities.
    Securities = b'S',
    /// Commodities.
    Commodities = b'C',
    /// Derivatives.
    Derivatives = b'D',
}

/// The form of the units.
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
pub enum UnitForm {
    /// Bearer shares.
    Bearer = b'B',
    /// Registered shares.
    Registered = b'R',
    /// Bearer/Registered shares.
    BearerRegistered = b'N',
    /// Bearer depository receipt.
    BearerReceipt = b'Z',
    /// Registered depository receipt.
    RegisteredReceipt = b'A',
    /// Others (misc).
    Other = b'M',
}
