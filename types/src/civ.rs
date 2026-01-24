//! Collective Investment Vehicles category support.

use crate::{NotApplicable, macros};

macros::impl_category! {
    /// Collective Investment Vehicles.
    ///
    /// Securities representing a portion of assets pooled by investors run by a management company
    /// whose share capital remains separate from such assets and includes issues of shares or
    /// units in the form of, for example, a unit trust, mutual fund, OICVM, OPCVM, SICAV or SICAF.
    pub enum Civ {
        /// Standard (vanilla) investment funds/mutual funds.
        ///
        /// An investment vehicle that is made up of a pool of funds collected from many investors
        /// for the purpose of investing in securities such as stocks, bonds, money market
        /// instruments and similar assets.
        Standard(Standard) = b'I',

        /// Hedge funds.
        ///
        /// Type of investment fund which pursues a total return and is usually open to qualified
        /// investors only.
        Hedge(Hedge) = b'H',

        /// Real estate investment trust (REITs).
        ///
        /// A REIT is a real estate company that offers shares/units to the public and invests in
        /// real estate directly, either through properties or mortgages.
        Reit(Reit) = b'B',

        /// Exchange traded funds (ETFs).
        ///
        /// An ETF is an investment fund traded on stock exchanges, much like stocks. An ETF holds
        /// assets such as stocks, commodities or bonds, and trades close to its net asset value
        /// over the course of the trading day. Most ETFs track an index, such as a stock, bond or
        /// commodity. index.
        Etf(Etf) = b'E',

        /// Pension funds.
        ///
        /// A pension fund is run by a financial intermediary for the company and its employees.
        /// The pension fund is a common asset pool meant to generate stable growth over the long
        /// term.
        Pension(Pension) = b'S',

        /// Funds of funds.
        ///
        /// A fund of funds is a CIV that invests directly in other investment funds rather than
        /// investing in stocks, bonds or other securities.
        FundOfFunds(FundOfFunds) = b'F',

        /// Private equity funds.
        ///
        /// A private equity fund is normally structured as a limited partnership or a limited
        /// liability company (investors are limited partners) managed by a GP.
        PrivateEquity(PrivateEquity) = b'P',

        /// Others (miscellaneous).
        ///
        /// CIVs which do not fit into any of the Groups described between standard (vanilla)
        /// investment funds/mutual funds and private equity funds.
        Other(Other) = b'M',
    }
}

macros::impl_group! {
    /// Standard (vanilla) investment funds/mutual funds.
    ///
    /// An investment vehicle that is made up of a pool of funds collected from many investors for
    /// the purpose of investing in securities such as stocks, bonds, money market instruments and
    /// similar assets.
    pub struct Standard {
        /// Closed/open-end.
        ///
        /// Indicates whether units are traded or whether funds continually stand ready to sell new
        /// units and redeem the outstanding units on demand.
        pub closed_or_open: ClosedOrOpen, 1;

        /// Distribution policy.
        ///
        /// Indicates the fund's normal distribution policy.
        pub distribution: Distribution, 2;

        /// Assets.
        ///
        /// Indicates the underlying assets in which the fund invests.
        pub assets: Assets, 3;

        /// Security type and investor restrictions.
        pub security_kind: KindAndRestrictions, 4;
    }
}

macros::impl_group! {
    /// Hedge funds.
    ///
    /// Type of investment fund which pursues a total return and is usually open to qualified
    /// investors only.
    pub struct Hedge {
        /// Investment strategy.
        ///
        /// The investment process describes core hedge fund strategy characteristics.
        pub strategy: Strategy, 1;

        /// Not applicable/undefined.
        pub attr2: NotApplicable, 2;

        /// Not applicable/undefined.
        pub attr3: NotApplicable, 3;

        /// Not applicable/undefined.
        pub attr4: NotApplicable, 4;
    }
}

macros::impl_group! {
    /// Real estate investment trust (REITs).
    ///
    /// A REIT is a real estate company that offers shares/units to the public and invests in real
    /// estate directly, either through properties or mortgages.
    pub struct Reit {
        /// Closed/open-end.
        ///
        /// Indicates whether units are traded or whether funds continually stand ready to sell new
        /// units and redeem the outstanding units on demand.
        pub closed_or_open: ClosedOrOpen, 1;

        /// Distribution policy.
        ///
        /// Indicates the fund's normal distribution policy.
        pub distribution: Distribution, 2;

        /// Not applicable/undefined.
        pub undefined3: NotApplicable, 3;

        /// Security type and investor restrictions.
        pub security_kind: KindAndRestrictions, 4;
    }
}

macros::impl_group! {
    /// Exchange traded funds (ETFs).
    ///
    /// An ETF is an investment fund traded on stock exchanges, much like stocks. An ETF holds
    /// assets such as stocks, commodities or bonds, and trades close to its net asset value over
    /// the course of the trading day. Most ETFs track an index, such as a stock, bond or
    /// commodity index.
    pub struct Etf {
        /// Closed/open-end.
        ///
        /// Indicates whether units are traded or whether funds continually stand ready to sell new
        /// units and redeem the outstanding units on demand.
        pub closed_or_open: ClosedOrOpen, 1;

        /// Distribution policy.
        ///
        /// Indicates the fund's normal distribution policy.
        pub distribution: Distribution, 2;

        /// Assets.
        ///
        /// Indicates the underlying assets in which the fund invests.
        pub underlying: Assets, 3;

        /// Security type.
        pub security_kind: Kind, 4;
    }
}

macros::impl_group! {
    /// Pension funds.
    ///
    /// A pension fund is run by a financial intermediary for the company and its employees. The
    /// pension fund is a common asset pool meant to generate stable growth over the long term.
    pub struct Pension {
        /// Closed/open.
        pub closed_or_open: ClosedOrOpen, 1;

        /// Strategy/style.
        pub style: Style, 2;

        /// Pension type.
        pub kind: PensionKind, 3;

        /// Security type.
        pub security_kind: Kind, 4;
    }
}

macros::impl_group! {
    /// Funds of funds.
    ///
    /// A fund of funds is a CIV that invests directly in other investment funds rather than
    /// investing in stocks, bonds or other securities.
    pub struct FundOfFunds {
        /// Closed/open-end.
        ///
        /// Indicates whether units are traded or whether funds continually stand ready to sell new
        /// units and redeem the outstanding units on demand.
        pub closed_or_open: ClosedOrOpen, 1;

        /// Distribution policy.
        ///
        /// Indicates the fund's normal distribution policy.
        pub distribution: Distribution, 2;

        /// Type of funds.
        ///
        /// Indicates the type of funds in which the fund invests.
        pub funds_kind: FundsKind, 3;

        /// Security type and investor restrictions.
        pub kind: KindAndRestrictions, 4;
    }
}

macros::impl_group! {
    /// Private equity funds.
    ///
    /// A private equity fund is normally structured as a limited partnership or a limited
    /// liability company (investors are limited partners) managed by a GP.
    pub struct PrivateEquity {
        /// Closed/open-end.
        ///
        /// Indicates whether units are traded or whether funds continually stand ready to sell new
        /// units and redeem the outstanding units on demand.
        pub closed_or_open: ClosedOrOpen, 1;

        /// Distribution policy.
        ///
        /// Indicates the fund's normal distribution policy.
        pub distribution: Distribution, 2;

        /// Assets.
        ///
        /// Indicates the underlying assets in which the fund invests.
        pub assets: Assets, 3;

        /// Security type and investor restrictions.
        pub kind: KindAndRestrictions, 4;
    }
}

macros::impl_group! {
    /// Others (miscellaneous).
    ///
    /// CIVs which do not fit into any of the Groups described between standard (vanilla)
    /// investment funds/mutual funds and private equity funds.
    pub struct Other {
        /// Not applicable/undefined.
        pub undefined1: NotApplicable, 1;

        /// Not applicable/undefined.
        pub undefined2: NotApplicable, 2;

        /// Not applicable/undefined.
        pub undefined3: NotApplicable, 3;

        /// Security type and investor restrictions.
        pub kind: KindAndRestrictions, 4;
    }
}

macros::impl_attr! {
    /// Closed/open-end.
    ///
    /// Indicates whether units are traded or whether funds continually stand ready to sell new
    /// units and redeem the outstanding units on demand.
    pub enum ClosedOrOpen[2] InvalidClosedOrOpen {
        /// Closed-end.
        ///
        /// Units are sold on either an organized exchange or in the over-the-counter (OTC) market
        /// and are usually not redeemed.
        Closed = b'C', "C";

        /// Open-end.
        ///
        /// Funds permanently sell new units to the public and redeem outstanding units on demand,
        /// resulting in an increase or decrease of outstanding capital.
        Open = b'O', "O";

        /// Others (miscellaneous).
        Others = b'M', "M";
    }
}

macros::impl_attr! {
    /// Distribution policy.
    ///
    /// Indicates the fund's normal distribution policy.
    pub enum Distribution[3] InvalidDistribution {
        /// Income funds.
        ///
        /// The fund regularly distributes its investment profits.
        Income = b'I', "I";

        /// Accumulation funds.
        ///
        /// The fund normally reinvests its investment profits.
        Accumulation = b'G', "G";

        /// Mixed funds.
        ///
        /// Investment profits are partly distributed, partly reinvested.
        Mixed = b'J', "J";
    }
}

macros::impl_attr! {
    /// Assets.
    ///
    /// Indicates the underlying assets in which the fund invests.
    pub enum Assets[4] InvalidAsset {
        /// Real estate.
        RealEstate = b'R', "R";

        /// Debt instruments.
        ///
        /// Fund invests in debt instrument regardless of maturity.
        Debt = b'B', "B";

        /// Equities.
        Equities = b'E', "E";

        /// Convertible securities.
        Convertibles = b'V', "V";

        /// Mixed.
        ///
        /// Fund invests in different assets.
        Mixed = b'L', "L";

        /// Commodities.
        Commodities = b'C', "C";

        /// Derivatives.
        Derivatives = b'D', "D";

        /// Referential instruments excluding commodities.
        Referential = b'F', "F";

        /// Credits.
        ///
        /// Contractual agreement in which a borrower receives something of value (good, service or
        /// money) now and agrees to repay the lender at some date in the future, generally with
        /// interest; CIVs normally invest in credits originated by third parties; credits are not
        /// freely transferable like debt securities.
        Credits = b'K', "K";

        /// Others (miscellaneous).
        Others = b'M', "M";
    }
}

macros::impl_attr! {
    /// Security type and investor restrictions.
    pub enum KindAndRestrictions[5] InvalidKindAndRestriction {
        /// Shares for either retail and/or qualified/institutional/professional investors.
        Shares = b'S', "S";

        /// Shares for Qualified/institutional/professional investors only.
        SharesForQualified = b'Q', "Q";

        /// Units for retail and/or qualified/institutional/professional investors.
        Utils = b'U', "U";

        /// Units for qualified/institutional/professional investors only.
        UnitsForQualizfied = b'Y', "Y";
    }
}

macros::impl_attr! {
    /// Investment strategy.
    ///
    /// The investment process describes core hedge fund strategy characteristics.
    pub enum Strategy[2] InvalidStrategy {
        /// Directional.
        ///
        /// The two biggest constituents of directional are macro and commodity trading advisor
        /// (CTA)/managed futures; macro describes directional strategies that are based upon the
        /// direction of market prices of currencies, commodities, equities, fixed income and
        /// includes futures and cash markets; CTA/managed futures describe strategies that are
        /// based upon futures contracts across all asset classes only.
        Directional = b'D', "D";

        /// Relative value.
        ///
        /// Strategies focusing on the spread relationships across various financial assets or
        /// commodities; they often utilize leverage and avoid market risk, although spread risk
        /// may often be large.
        Relative = b'R', "R";

        /// Security selection.
        ///
        /// Strategies typically equity-based and including long/short equity; the manager attempts
        /// to make money from superior stock selection by building some combination of long and
        /// short positions in such a way to mitigate systematic market risks.
        Selection = b'S', "S";

        /// Event-driven.
        ///
        /// Combination of investment strategies focusing on securities that are expected to
        /// experience a change in valuation due to corporate transactions or events such as
        /// bankruptcies.
        EventDriven = b'E', "E";

        /// Arbitrage.
        ///
        /// In economics and finance, arbitrage is the practice of taking advantage of a price
        /// difference between two or more markets, striking a combination of matching deals that
        /// capitalize upon the imbalance, the profit being the difference between the market
        /// prices.
        Arbitrage = b'A', "A";

        /// Multi-strategy.
        ///
        /// Multi-strategy as a separate set of investment strategies is broad and by it the
        /// manager is expected to maintain approximately 25 % of portfolio exposure in two or more
        /// strategies that are distinct from one another.
        Multi = b'N', "N";

        /// Asset-based lending.
        ///
        /// Strategy based on providing loans against assets to companies, including the ones
        /// viewed as not being creditworthy by commercial banks; the amount of the loan is secured
        /// by claims against the borrower’s assets and as such it is directly determined by the
        /// assets' value.
        Lending = b'L', "L";

        /// Others (miscellaneous).
        Others = b'M', "M";
    }
}

macros::impl_attr! {
    /// Security type.
    pub enum Kind[5] InvalidKind {
        /// Shares.
        Shares = b'S', "S";

        /// Units.
        Units = b'U', "U";
    }
}

macros::impl_attr! {
    /// Strategy/style.
    pub enum Style[3] InvalidStyle {
        /// Balanced/conservative.
        Balanced = b'B', "B";

        /// Growth.
        Growth = b'G', "G";

        /// Life style.
        ///
        /// Strategy changes depending on age group of members.
        Lifestyle = b'L', "L";

        /// Others (miscellaneous).
        Other = b'M', "M";
    }
}

macros::impl_attr! {
    /// Pension type.
    pub enum PensionKind[4] InvalidKind {
        /// Defined benefit.
        Benefit = b'R', "R";

        /// Defined contribution.
        Contribution = b'B', "B";

        /// Others (miscellaneous).
        Other = b'M', "M";
    }
}

macros::impl_attr! {
    /// Type of funds.
    ///
    /// Indicates the type of funds in which the fund invests.
    pub enum FundsKind[4] InvalidKind {
        /// Standard (vanilla) investment funds/mutual funds.
        Standard = b'I', "I";

        /// Hedge funds.
        Hedge = b'H', "H";

        /// REITs.
        Reit = b'B', "B";

        /// ETFs.
        Etf = b'E', "E";

        /// Private equity funds.
        PrivateEquity = b'P', "P";

        /// Others (miscellaneous).
        Other = b'M', "M";
    }
}
