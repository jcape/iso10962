//! Entitlement category support.

use crate::{Form, NotApplicable, macros};

macros::impl_category! {
    /// Entitlement (rights).
    ///
    /// Financial instruments providing the holder with the privilege to subscribe to or receive
    /// specific assets on terms specified.
    pub enum Right {
        /// Allotment (bonus) rights.
        ///
        /// Privileges allotted to existing security holders, entitling them to receive new
        /// securities free of charge.
        Allotment(Allotment) = b'A', "B";

        /// Subscription rights.
        ///
        /// Privileges allotted to existing security holders, entitling them to subscribe to new
        /// securities at a price normally lower than the prevailing market price.
        Subscription(Subscription) = b'S', "S";

        /// Purchase rights.
        ///
        /// Anti-takeover device that gives a prospective acquiree's shareholders the right to buy
        /// shares of the firm or shares of anyone who acquires the firm at a deep discount to
        /// their fair market value.
        Purchase(Purchase) = b'P', "P";

        /// Warrants.
        ///
        /// Financial instruments which permit the holder to purchase a specified amount of a
        /// financial instrument, commodity, currency or other during a specified period at a
        /// specified price.
        Warrant(Warrant) = b'W', "W";

        /// Mini-future certificates, constant leverage certificates.
        ///
        /// Mini-futures combine the structure of open-end certificates with leverage option.
        /// Mini-futures have no fixed term. The leverage is therefore available without a term
        /// restriction. The price of a mini-future always corresponds to its intrinsic value, i.e.
        /// the capital outlay, plus the bid-ask spread. The financing costs associated with
        /// building up the leverage effect are offset against the capital outlay on a daily basis,
        /// thereby eliminating the need for a premium. Investors have to pay only financing costs
        /// they actually utilize. In contrast to options, factors like volatility have no
        /// influence at all on the price of mini-futures.
        MiniFuture(MiniFuture) = b'F', "F";

        /// Depositary receipts on entitlements.
        ///
        /// Depository receipts are securities that facilitate the ownership of instruments traded
        /// in other jurisdictions. Depository receipts are widely used in order to allow the
        /// trading of entitlements in jurisdictions other than the one where the original
        /// entitlements were issued.
        DepositoryReceipt(DepositoryReceipt) = b'D', "D";

        /// Others (miscellaneous).
        ///
        /// Entitlements (rights) that do not fit into any of the other Groups of entitlements
        /// rights.
        Other(Other) = b'M', "M";
    }
}

macros::impl_group! {
    /// Allotment (bonus) rights.
    pub struct Allotment {
        /// Not applicable/undefined.
        pub unassigned1: NotApplicable, 1;

        /// Not applicable/undefined.
        pub unassigned2: NotApplicable, 2;

        /// Not applicable/undefined.
        pub unassigned3: NotApplicable, 3;

        /// Form (negotiability, transmission).
        pub form: Form, 4;
    }
}

macros::impl_group! {
    /// Subscription rights.
    ///
    /// Privileges allotted to existing security holders, entitling them to subscribe to new
    /// securities at a price normally lower than the prevailing market price.
    pub struct Subscription {
        /// Assets (indicates the type of assets that the rights holder is entitled to acquire).
        pub assets: Assets, 1;

        /// Not applicable/undefined.
        pub unassigned2: NotApplicable, 2;

        /// Not applicable/undefined.
        pub unassigned3: NotApplicable, 3;

        /// Form (negotiability, transmission).
        pub form: Form, 4;
    }
}

macros::impl_group! {
    /// Purchase rights.
    ///
    /// Anti-takeover device that gives a prospective acquiree's shareholders the right to buy
    /// shares of the firm or shares of anyone who acquires the firm at a deep discount to their
    /// fair market value.
    pub struct Purchase {
        /// Assets (indicates the type of assets that the rights holder is entitled to acquire).
        pub assets: Assets, 1;

        /// Not applicable/undefined.
        pub undefined2: NotApplicable, 2;

        /// Not applicable/undefined.
        pub undefined3: NotApplicable, 3;

        /// Form (negotiability, transmission).
        pub form: Form, 4;
    }
}

macros::impl_group! {
    /// Warrants.
    ///
    /// Financial instruments which permit the holder to purchase a specified amount of a financial
    /// instrument, commodity, currency or other during a specified period at a specified price.
    pub struct Warrant {
        /// Underlying assets (indicates the type of underlying assets that the warrant holder is
        /// entitled to acquire).
        pub asset: Assets, 1;

        /// Type (indicates whether the warrant is issued by the issuer of the underlying
        /// instrument or by a third party).
        pub kind: Kind, 2;

        /// Call/put (indicates whether the warrant entitles the holder to acquire assets at
        /// specified terms or to acquire cash in exchange for specific underlying assets).
        pub call_put: CallPut, 3;

        /// Exercise option style.
        pub exercise_style: ExerciseStyle, 4;
    }
}

macros::impl_group! {
    /// Mini-future certificates, constant leverage certificates.
    ///
    /// Mini-futures combine the structure of open-end certificates with leverage option.
    /// Mini-futures have no fixed term. The leverage is therefore available without a term
    /// restriction. The price of a mini-future always corresponds to its intrinsic value, i.e. the
    /// capital outlay, plus the bid-ask spread. The financing costs associated with building up
    /// the leverage effect are offset against the capital outlay on a daily basis, thereby
    /// eliminating the need for a premium. Investors have to pay only financing costs they
    /// actually utilize. In contrast to options, factors like volatility have no influence at all
    /// on the price of mini-futures.
    pub struct MiniFuture {
        /// Underlying assets (indicates the type of underlying assets that the warrant holder is
        /// entitled to acquire).
        pub asset: FutureAsset, 1;

        /// Barrier dependency type (indicates whether the instrument barrier depends on the
        /// underlying level or on the instrument trading price level).
        pub barrier: Barrier, 2;

        /// Long/short (indicates whether the instrument entitles the holder to acquire assets at
        /// specified terms or to acquire cash in exchange for specific underlying assets).
        pub long_short: LongShort, 3;

        /// Exercise option style
        pub exercise_style: ExerciseStyle, 4;
    }
}

macros::impl_group! {
    /// Depositary receipts on entitlements.
    ///
    /// Depository receipts are securities that facilitate the ownership of instruments traded in
    /// other jurisdictions. Depository receipts are widely used in order to allow the trading of
    /// entitlements in jurisdictions other than the one where the original entitlements were
    /// issued.
    pub struct DepositoryReceipt {
        /// Instrument dependency.
        pub dependency: Dependency, 1;

        /// Not applicable/undefined.
        pub undefined2: NotApplicable, 2;

        /// Not applicable/undefined.
        pub undefined3: NotApplicable, 3;

        /// Form (negotiability, transmission).
        pub form: Form, 4;
    }
}

macros::impl_group! {
    /// Other (miscellaneous).
    ///
    /// Entitlements (rights) that do not fit into any of the other Groups of entitlements rights.
    pub struct Other {
        /// Not applicable/undefined.
        pub undefined1: NotApplicable, 1;

        /// Not applicable/undefined.
        pub undefined2: NotApplicable, 2;

        /// Not applicable/undefined.
        pub undefined3: NotApplicable, 3;

        /// Not applicable/undefined.
        pub undefined4: NotApplicable, 4;
    }
}

macros::impl_attr! {
    /// Assets (indicates the type of assets that the rights holder is entitled to acquire).
    pub enum Assets[2] {
        /// Common/ordinary shares.
        Common = b'S', "S";

        /// Preferred/preference shares.
        Preferred = b'P', "P";

        /// Common/ordinary convertible shares.
        Convertible = b'C', "C";

        /// Preferred/preference convertible shares.
        PreferredConvertible = b'F', "F";

        /// Bonds.
        Bonds = b'B', "B";

        /// Combined instruments.
        Combined = b'I', "I";

        /// Other (miscellaneous).
        Other = b'M', "M";
    }
}

macros::impl_attr! {
    /// Type.
    ///
    /// Indicates whether the warrant is issued by the issuer of the underlying instrument or by a
    /// third party.
    pub enum Kind[3] {
        /// Traditional warrants.
        ///
        /// Issued by the issuer of the underlying instrument.
        Traditional = b'T', "T";

        /// Naked warrants.
        ///
        /// Issued by a third party which is not the issuer of the underlying securities to which
        /// the warrant refers; warrant issuer does not hold as many securities as would be
        /// required if all the warrants are exercised.
        Naked = b'N', "N";

        /// Covered warrants.
        ///
        /// Issued by a third party which is not the issuer of the underlying securities to which
        /// the warrant refers; warrant issuer holds as many securities as would be required if all
        /// the warrants are exercised.
        Covered = b'C', "C";
    }
}

macros::impl_attr! {
    /// Call/put.
    ///
    /// Indicates whether the warrant entitles the holder to acquire assets at specified terms or
    /// to acquire cash in exchange for specific underlying assets.
    pub enum CallPut[4] {
        /// Call.
        ///
        /// In most cases, the warrant entitles the holder to acquire specific underlying assets
        /// during a specified period at a specified price.
        Call = b'C', "C";

        /// Put.
        ///
        /// The warrant entitles the holder to acquire cash in exchange for specific underlying assets.
        Put = b'P', "P";

        /// Call and put.
        ///
        /// Warrants with neither call nor put feature or warrants with call and put feature.
        CallAndPut = b'B', "B";
    }
}

macros::impl_attr! {
    /// Exercise option style.
    pub enum ExerciseStyle[5] {
        /// European.
        ///
        /// Warrant that can only be exercised for a short, specified period of time just prior to
        /// its expiration, usually a single day.
        European = b'E', "E";

        /// American.
        ///
        /// Warrant that can be exercised at any time between the purchase date and the expiration
        /// date.
        American = b'A', "A";

        /// Bermudan.
        ///
        /// Warrant that can only be exercised on predetermined dates, usually every month.
        Bermudan = b'B', "B";

        /// Others (miscellaneous).
        Other = b'M', "M";
    }
}

macros::impl_attr! {
    /// Underlying assets.
    ///
    /// Indicates the type of underlying assets that the warrant holder is entitled to acquire.
    pub enum FutureAsset[2] {
        /// Baskets (the warrant holder is entitled to acquire a package or group of assets).
        Basket = b'B', "B";

        /// Equities (the warrant holder is entitled to acquire equity).
        Equity = b'S', "S";

        /// Debt instruments/interest rates (the warrant holder is entitled to acquire debt
        /// instruments).
        Debt = b'D', "D";

        /// Commodities (the warrant holder is entitled to acquire a specific commodity).
        Commodity = b'T', "T";

        /// Currencies (the warrant holder is entitled to acquire a specified amount in a certain
        /// currency at a specified exchange rate).
        Currency = b'C', "C";

        /// Indices (the warrant holder is entitled to acquire a specified amount based on the
        /// performance of an index).
        Index = b'I', "I";

        /// Others (miscellaneous).
        Other = b'M', "M";
    }
}

macros::impl_attr! {
    /// Barrier dependency type.
    ///
    /// Indicates whether the instrument barrier depends on the underlying level or on the
    /// instrument trading price level.
    pub enum Barrier[3] {
        /// Barrier underlying based (the instrument immediately expires if the barrier underlying
        /// level is breached during product lifetime).
        Underlying = b'T', "T";

        /// Barrier instrument based (the instrument immediately expires if the barrier instrument
        /// trading price level is breached during product lifetime).
        Instrument = b'N', "N";

        /// Others (miscellaneous).
        Other = b'M', "M";
    }
}

macros::impl_attr! {
    /// Long/short.
    ///
    /// Indicates whether the instrument entitles the holder to acquire assets at specified terms
    /// or to acquire cash in exchange for specific underlying assets.
    pub enum LongShort[4] {
        /// Long (in most cases, the instrument entitles the holder to acquire specific underlying
        /// assets during a specified period at a specified price).
        Long = b'C', "C";

        /// Short (the instrument entitles the holder to acquire cash in exchange for specific
        /// underlying assets).
        Short = b'P', "P";

        /// Others (miscellaneous).
        Other = b'M', "M";
    }
}

macros::impl_attr! {
    /// Instrument dependency.
    pub enum Dependency[2] {
        /// Allotment (bonus) rights.
        Allotment = b'A', "A";

        /// Subscription rights.
        Subscription = b'S', "S";

        /// Purchase rights.
        Purchase = b'P', "P";

        /// Warrants.
        Warrant = b'W', "W";

        /// Others (miscellaneous).
        Other = b'M', "M";
    }
}
