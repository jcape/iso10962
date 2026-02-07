//! Equities category support.

use crate::{Form, NotApplicable, macros};

macros::impl_category! {
    /// Financial instruments representing an ownership interest in an entity or pool of assets.
    pub enum Equity {
        /// Common/ordinary shares.
        ///
        /// Holders are typically entitled to vote and receive dividends. In the event of
        /// liquidation, holders of shares usually rank behind the entity's creditors and holders
        /// of preferred/preference shares.
        Common(Common) = b'S', "S";

        /// Preferred/preference shares.
        ///
        /// Payment of dividends to holders normally takes preference over the payment of dividends
        /// to other classes of shares. In the event of liquidation, preferred/preference shares
        /// normally rank above ordinary shares but behind creditors of the company.
        Preferred(Preferred) = b'P', "P";

        /// Common/ordinary convertible shares.
        ///
        /// Shares (common/ordinary) which, at the discretion of the holder, are convertible into
        /// other securities, at a designated rate. The conversion privilege may be perpetual or
        /// limited to a specific period.
        Convertible(Convertible) = b'C', "C";

        /// Preferred/preference convertible shares.
        ///
        /// Preferred/preference shares which, at the discretion of the holder, are convertible
        /// into other securities, usually common/ordinary shares, at a designated rate. The
        /// conversion privilege may be perpetual or limited to a specified period.
        PreferedConvertible(PreferredConvertible) = b'F', "F";

        /// Limited partnership units.
        ///
        /// A limited partnership is a form of partnership similar to a general partnership, except
        /// that in addition to one or more general partners (GPs), there are one or more limited
        /// partners (LPs).
        ///
        /// Like shareholders in a corporation, the LPs have limited liability, i.e. they are only
        /// liable on debts incurred by the firm to the extent of their registered investment and
        /// they have no management authority. The GPs pay the LPs the equivalent of a dividend on
        /// their investment, the nature and extent of which is usually defined in the partnership
        /// agreement.
        LlpUnit(LlpUnit) = b'L', "F";

        /// Depository receipts on equities.
        ///
        /// Depository receipts are securities that facilitate the ownership of securities traded
        /// in other jurisdictions. Depository receipts are widely used in order to allow the
        /// trading of shares in jurisdictions other than the one where the original shares were
        /// issued.
        DepositoryReceipt(DepositoryReceipt) = b'D', "D";

        /// Structured instruments (participation).
        ///
        /// The construction is generally based on a low exercise price option (LEPO) (base value
        /// less discounted future dividends) which in some cases might be comparable to a direct
        /// investment in the underlying asset(s) or a LEPO combined with other options, which
        /// together provide the desired disbursement profile.
        Structured(Structured) = b'Y', "Y";

        /// Others (miscelaneous).
        Other(Other) = b'M', "M";
    }
}

macros::impl_group! {
    /// Attributes applicable to common stock.
    ///
    /// Holders are typically entitled to vote and receive dividends. In the event of liquidation,
    /// holders of shares usually rank behind the entity's creditors and holders of
    /// preferred/preference shares.
    pub struct Common {
        /// Voting right (indicates the kind of voting power conferred to the shareholder).
        pub voting_right: VotingRight, 1;

        /// Ownership/transfer/sales restrictions (the ownership or transfer of the security is
        /// subject to special conditions including country-specific restrictions).
        pub ownership: Ownership, 2;

        /// Payment status.
        pub payment_status: PaymentStatus, 3;

        /// Form (negotiability, transmission).
        pub form: Form, 4;
    }
}

macros::impl_group! {
    /// Attributes applicable to prefered shares.
    ///
    /// Payment of dividends to holders normally takes preference over the payment of dividends to
    /// other classes of shares. In the event of liquidation, preferred/preference shares normally
    /// rank above ordinary shares but behind creditors of the company.
    pub struct Preferred {
        /// Voting right (indicates the kind of voting power conferred to the shareholder).
        pub voting_right: VotingRight, 1;

        /// Redemption (indicates the retirement provisions made for the shares).
        pub redemption: Redemption, 2;

        /// Income (indicates the kind of dividend income the shareholders are entitled to).
        pub income: Income, 3;

        /// Form (negotiability, transmission).
        pub form: Form, 4;
    }
}

macros::impl_group! {
    /// Attributes applicable to convertible equities.
    ///
    /// Shares (common/ordinary) which, at the discretion of the holder, are convertible into other
    /// securities, at a designated rate. The conversion privilege may be perpetual or limited to a
    /// specific period.
    pub struct Convertible {
        /// Voting right (indicates the kind of voting power conferred to the shareholder).
        pub voting_right: VotingRight, 1;

        /// Ownership/transfer/sales restrictions (the ownership or transfer of the security is
        /// subject to special conditions including country-specific restrictions).
        pub ownership: Ownership, 2;

        /// Payment status.
        pub payment_status: PaymentStatus, 3;

        /// Form (negotiability, transmission).
        pub form: Form, 4;
    }
}

macros::impl_group! {
    /// Attributes applicable to preferred/preference convertible equities.
    ///
    /// Preferred/preference shares which, at the discretion of the holder, are convertible into
    /// other securities, usually common/ordinary shares, at a designated rate. The conversion
    /// privilege may be perpetual or limited to a specified period.
    pub struct PreferredConvertible {
        /// Voting right (indicates the kind of voting power conferred to the shareholder).
        pub voting_right: VotingRight, 1;

        /// Redemption (indicates the retirement provisions made for the shares).
        pub redemption: Redemption, 2;

        /// Income (indicates the kind of dividend income the shareholders are entitled to).
        pub income: Income, 3;

        /// Form (negotiability, transmission).
        pub form: Form, 4;
    }
}

macros::impl_group! {
    /// Attributes applicable to limited partnership units.
    ///
    /// A limited partnership is a form of partnership similar to a general partnership, except
    /// that in addition to one or more general partners (GPs), there are one or more limited
    /// partners (LPs).
    ///
    /// Like shareholders in a corporation, the LPs have limited liability, i.e. they are only
    /// liable on debts incurred by the firm to the extent of their registered investment and they
    /// have no management authority. The GPs pay the LPs the equivalent of a dividend on their
    /// investment, the nature and extent of which is usually defined in the partnership agreement.
    pub struct LlpUnit {
        /// Voting right (indicates the kind of voting power conferred to the shareholder).
        pub voting_right: VotingRight, 1;

        /// Ownership/transfer/sales restrictions (the ownership or transfer of the security is
        /// subject to special conditions including country-specific restrictions).
        pub ownership: Ownership, 2;

        /// Payment status.
        pub payment_status: PaymentStatus, 3;

        /// Form (negotiability, transmission).
        pub form: Form, 4;
    }
}

macros::impl_group! {
    /// Attributes applicable to depository receipts.
    ///
    /// Depository receipts are securities that facilitate the ownership of securities traded in other
    /// jurisdictions. Depository receipts are widely used in order to allow the trading of shares in
    /// jurisdictions other than the one where the original shares were issued.
    pub struct DepositoryReceipt {
        /// Instrument dependency (represents the ownership of an instrument provided in this
        /// table).
        pub dependency: Dependency, 1;

        /// Redemption/conversion of the underlying assets.
        pub redemption: RedemptionConversion, 2;

        /// Income (indicates the kind of dividend income the shareholders are entitled to).
        pub income: Income, 3;

        /// Form (negotiability, transmission).
        pub form: Form, 4;
    }
}

macros::impl_group! {
    /// Attributes applicable to structured equities.
    ///
    /// The construction is generally based on a low exercise price option (LEPO) (base value less
    /// discounted future dividends) which in some cases might be comparable to a direct investment in
    /// the underlying asset(s) or a LEPO combined with other options, which together provide the
    /// desired disbursement profile.
    pub struct Structured {
        /// Type.
        pub kind: Kind, 1;

        /// Distribution (indicates the cash distribution provided by the structured instrument).
        pub distribution: Distribution, 2;

        /// Repayment (indicates the repayment form provided by the structured instrument).
        pub repayment: Repayment, 3;

        /// Underlying assets (indicates the type of underlying assets in which the structured
        /// instrument participates).
        pub underlying: Underlying, 4;
    }
}

macros::impl_group! {
    /// Attributes applicable to other equities.
    ///
    /// Equities that do not fit into any of the other Equity Groups.
    pub struct Other {
        /// Not applicable/undefined.
        pub attr1: NotApplicable, 1;

        /// Not applicable/undefined.
        pub attr2: NotApplicable, 2;

        /// Not applicable/undefined.
        pub attr3: NotApplicable, 3;

        /// Form (negotiability, transmission).
        pub form: Form, 4;
    }
}

macros::impl_attr! {
    /// Voting right.
    ///
    /// Indicates the kind of voting power conferred to the shareholder.
    pub enum VotingRight[2] {
        /// Voting (each share has one vote).
        Voting = b'V', "V";

        /// Non-voting (the shareholder has no voting right).
        NonVoting = b'N', "N";

        /// Restricted voting (the shareholder may be entitled to less than one vote per share).
        Restricted = b'R', "R";

        /// Enhanced voting (the shareholder is entitled to more than one vote per share).
        Enhanced = b'E', "E";
    }
}

macros::impl_attr! {
    /// Ownership/transfer/sales restrictions.
    ///
    /// The ownership or transfer of the security is subject to special conditions including
    /// country-specific restrictions.
    pub enum Ownership[3] {
        /// Restrictions.
        Restricted = b'T', "T";

        /// Free (unrestricted).
        Free = b'U', "U";
    }
}

macros::impl_attr! {
    /// The payment status.
    pub enum PaymentStatus[4] {
        /// Fully paid.
        Fully = b'F', "F";

        /// Nil paid.
        Nil = b'O', "O";

        /// Partially paid.
        Partial = b'P', "P";
    }
}

macros::impl_attr! {
    /// Redemption.
    ///
    /// Indicates the retirement provisions made for the shares.
    pub enum Redemption[3] {
        /// Redeemable.
        ///
        /// The shares may be redeemed at the option of the issuer and/or of the shareholder.
        Redeemable = b'R', "R";

        /// Extendible.
        ///
        /// The redemption date can be extended at the issuer or holder option.
        Extendible = b'E', "E";

        /// Redeemable/extendible.
        ///
        /// The issuer and/or holders of redeemable shares with a fixed maturity date have the
        /// option to extend the maturity date.
        RedeemableExtendible = b'T', "T";

        /// Exchangeable.
        ///
        /// The shares may be exchanged for securities of another issuer.
        Exchangeable = b'G', "G";

        /// Redeemable/exchangeable/extendible.
        ///
        /// The issuer and/or holders of redeemable shares with a fixed maturity date have the
        /// option to extend the maturity date and the shares may be exchanged for securities of
        /// another issuer.
        RedeemableExchangeableExtendible = b'A', "A";

        /// Redeemable/exchangeable.
        ///
        /// The shares may be redeemed at the option of the issuer and/or of the shareholder and
        /// may be exchanged for securities of another issuer.
        RedeemableExchangeable = b'C', "C";

        /// Perpetual.
        ///
        /// The share has no fixed maturity date.
        Perpetual = b'N', "N";
    }
}

macros::impl_attr! {
    /// Income.
    ///
    /// Indicates the kind of dividend income the shareholders are entitled to.
    pub enum Income[4] {
        /// Fixed rate income.
        ///
        /// The shareholder periodically receives a stated income.
        FixedRate = b'F', "F";

        /// Cumulative, fixed rate income.
        ///
        /// The shareholder periodically receives a stated amount; dividends not paid in any year
        /// accumulate and shall be paid at a later date before dividends can be paid on the
        /// common/ordinary shares.
        CumulativeFixedRate = b'C', "C";

        /// Participating income.
        ///
        /// Preferred/preference shareholders, in addition to receiving their fixed rate of prior
        /// dividend, share with the common shareholders in further dividend distributions and in
        /// capital distributions.
        Participating = b'P', "P";

        /// Cumulative, participating income.
        ///
        /// Shareholders are entitled to dividends in excess of the stipulated preferential rate
        /// under specified conditions; dividends not paid in any year accumulate and shall be paid
        /// at a later date before dividends can be paid on the common/ordinary shares.
        CumulativeParticipating = b'Q', "Q";

        /// Adjustable/variable rate income.
        ///
        /// The dividend rate is set periodically, usually based on a certain yield.
        AdjustableRate = b'A', "A";

        /// Normal rate income.
        ///
        /// Shareholders are entitled to the same dividends as common/ordinary shareholders, but
        /// have other privileges, for example as regards distribution of assets upon dissolution.
        NormalRate = b'N', "N";

        /// Auction rate income.
        ///
        /// Dividend is adjusted through an auction, such as the Dutch auction.
        AuctionRate = b'U', "U";
    }
}

macros::impl_attr! {
    /// Instrument dependency.
    ///
    /// Represents the ownership of an instrument provided in this table.
    pub enum Dependency[2] {
        /// Common/ordinary shares.
        Common = b'S', "S";

        /// Preferred/preference shares.
        Preferred = b'P', "P";

        /// Common/Ordinary convertible shares.
        CommonConvertible = b'C', "C";

        /// Preferred/preference convertible shares.
        PreferredConvertible = b'F', "F";

        /// Limited partnership units.
        LlpUnit = b'L', "L";

        /// Other (miscellaneous).
        Other = b'M', "M";
    }
}

macros::impl_attr! {
    /// Redemption/conversion of the underlying assets.
    ///
    /// # Guidelines
    ///
    /// For common/ordinary shares and limited partnership units, only the values `N`
    /// ([`Perpetual`](RedemptionConversion::Perpetual)) `X` (not
    /// applicable/undefined) may be used. All values apply for other underlying instruments.
    pub enum RedemptionConversion[3] {
        /// Redeemable.
        Redeemable = b'R', "R";

        /// Perpetual.
        Perpetual = b'N', "N";

        /// Convertible.
        Convertible = b'B', "B";

        /// Convertible/redeemable.
        ConvertibleRedeemable = b'D', "D";
    }
}

macros::impl_attr! {
    /// The structured instrument type.
    pub enum Kind[2] {
        /// Tracker certificate.
        ///
        /// Participation in development of the underlying asset(s); reflects underlying price
        /// moves 1:1 (adjusted by conversion ratio and any related fees); risk is comparable to
        /// direct investment in the underlying asset(s).
        Tracker = b'A', "A";

        /// Outperformance certificate.
        ///
        /// Participation in development of the underlying asset(s); disproportionate participation
        /// (outperformance) in positive performance above the strike; reflects underlying price
        /// moves 1:1 (adjusted by conversion ratio and any related fees); risk is comparable to
        /// direct investment in the underlying asset(s).
        Outperforming = b'B', "B";

        /// Bonus certificate.
        ///
        /// Participation in development of the underlying asset(s); minimum redemption is equal to
        /// the nominal value provided the barrier has not been breached; if the barrier is
        /// breached the product changes into a tracker certificate; with greater risk multiple
        /// underlying asset(s) (worst-of) allow for a higher bonus level or lower barrier; reduced
        /// risk compared to a direct investment into the underlying asset(s).
        Bonus = b'C', "C";

        /// Outperformance bonus certificate.
        ///
        /// Participation in development of the underlying asset(s); disproportionate participation
        /// (outperformance) in positive performance above the strike; minimum redemption is equal
        /// to the nominal value provided the barrier has not been breached; if the barrier is
        /// breached the product changes into an outperformance certificate; with greater risk
        /// multiple underlying asset(s) (worst-of) allow for a higher bonus level or lower
        /// barrier; reduced risk compared to a direct investment into the underlying asset(s).
        OutperformanceBonus = b'D', "D";

        /// Twin-win-certificate.
        ///
        /// Participation in development of the underlying asset(s); profits possible with rising
        /// and falling underlying asset values; falling underlying asset price converts into
        /// profit up to the barrier; minimum redemption is equal to the nominal value provided the
        /// barrier has not been breached; if the barrier is breached the product changes into a
        /// tracker certificate; with higher risk levels, multiple underlying asset(s) (worst-of)
        /// allow for a higher bonus level or lower barrier; reduced risk compared to a direct
        /// investment into the underlying asset(s).
        TwinWin = b'E', "E";

        /// Other (miscellaneous).
        Other = b'M', "M";
    }
}

macros::impl_attr! {
    /// Distribution.
    ///
    /// Indicates the cash distribution provided by the structured instrument.
    pub enum Distribution[3] {
        /// Dividend payments.
        ///
        /// This depends on strategy of the structured instrument.
        Dividend = b'D', "D";
        /// No payments.
        None = b'Y', "Y";
        /// Others (miscellaneous).
        Other = b'M', "M";
    }
}

macros::impl_attr! {
    /// Repayment.
    ///
    /// Indicates the repayment form provided by the structured instrument.
    pub enum Repayment[4] {
        /// Cash repayment.
        Cash = b'F', "F";

        /// Physical repayment.
        Physical = b'V', "V";

        /// Elect at settlement (determined at the time of settlement).
        Elect = b'E', "E";

        /// Others (miscellaneous).
        Other = b'M', "M";
    }
}

macros::impl_attr! {
    /// Underlying assets.
    ///
    /// Indicates the type of underlying assets in which the structured instrument participates.
    pub enum Underlying[5] {
        /// Baskets.
        ///
        /// Group of securities that have been put together for a specific investment purpose.
        Baskets = b'B', "B";

        /// Equities.
        Equities = b'S', "S";

        /// Debt instruments.
        Debt = b'D', "D";

        /// Derivatives (options, futures, swaps, spot, forwards, strategies, financing).
        Derivatives = b'G', "G";

        /// Commodities.
        Commodities = b'T', "T";

        /// Currencies (specified exchange rate).
        Currencies = b'C', "C";

        /// Indices (the performance of an index).
        Indices = b'I', "I";

        /// Interest rates (specified amount based on the future level of interest rates).
        Rates = b'N', "N";

        /// Others (miscellaneous).
        Other = b'M', "M";
    }
}
