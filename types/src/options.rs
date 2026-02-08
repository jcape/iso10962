//! Options (listed and unlisted).

use crate::{NotApplicable, Standardized, macros};

macros::impl_category! {
    /// Listed options.
    ///
    /// This Category classifies listed options, which are contracts that grant to the holder
    /// either the privilege to purchase or the privilege to sell the assets specified at a
    /// predetermined price or formula at or within a time in the future. Where a listed option
    /// cannot be classified within this Category, refer to non-listed and complex listed options.
    pub enum Listed {
        /// Call options.
        ///
        /// Contracts between a buyer and a seller giving the buyer (holder) the right, but not the
        /// obligation, to buy the assets specified at a fixed price or formula, on or before a
        /// specified date. The seller of the call option assumes the obligation of delivering the
        /// assets specified should the buyer exercise his or her option.
        Call(Call) = b'C', "C";

        /// Put options.
        ///
        /// Contracts between a buyer and a seller giving the buyer (holder) the right, but not the
        /// obligation, to sell the assets specified at a fixed price or formula, on or before a
        /// specified date. The seller of the put option assumes the obligation of buying the
        /// assets specified should the buyer exercise his or her option.
        Put(Put) = b'P', "P";

        /// Others (miscellaneous).
        Other(OtherListed) = b'M', "M";
    }
}

macros::impl_category! {
    /// Non-listed and complex listed options.
    ///
    /// This category includes OTC or unlisted options and also includes any listed option which is
    /// not captured by the listed options Category (see "Others"). An option grants the holder
    /// either the privilege to purchase or the privilege to sell the assets specified at a
    /// predetermined price or formula at or within a time in the future.
    pub enum Unlisted {
        /// Rates.
        ///
        /// An option where the holder of the option has the right but not the obligation to enter
        /// into the underlying contract, or pay or receive payment related to the underlying rate
        /// on a specified future date based on a specified future rate and term.
        Rate(Rate) = b'R', "R";

        /// Commodities.
        ///
        /// An option where the option buyer has the right to buy or sell specified commodities
        /// assets at a fixed price or formula, on or before a specified date.
        Commodity(Commodity) = b'T', "T";

        /// Equity.
        ///
        /// An option where the underlying asset is an equity-linked instrument (i.e. shares,
        /// depository receipts, ETFs, indices, baskets).
        Equity(Equity) = b'E', "E";

        /// Credit.
        ///
        /// An option to buy or sell a credit product which is a contract in which one party
        /// (protection seller) agrees to provide payment to the other party (protection buyer)
        /// should a credit event occur against the underlying, which could be a specified debt
        /// (the reference obligation), a specific debt issuer (reference entity), a basket of
        /// reference entities and/or reference obligations, or a credit index (reference index).
        Credit(Credit) = b'C', "C";

        /// Foreign exchange.
        ///
        /// An option to buy or sell a foreign exchange agreement between two parties to exchange a
        /// given amount of one currency for another currency for spot delivery or for forward
        /// delivery at an agreed rate after a specified period of time.
        Forex(Forex) = b'F', "F";

        /// Others (miscellaneous).
        ///
        /// Options that do not fit into any of the above Groups.
        Other(OtherUnlisted) = b'M', "M";
    }
}

macros::impl_group! {
    /// Call options.
    ///
    /// Contracts between a buyer and a seller giving the buyer (holder) the right, but not the
    /// obligation, to buy the assets specified at a fixed price or formula, on or before a
    /// specified date. The seller of the call option assumes the obligation of delivering the
    /// assets specified should the buyer exercise his or her option.
    pub struct Call {
        /// Exercise option style.
        pub exercise_style: ExerciseStyle, 1;

        /// Underlying assets (indicates the type of underlying assets that the option holder is
        /// entitled to acquire).
        pub underlying: Underlying, 2;

        /// Delivery (indicates whether the settlement of the option, when exercised, is made in
        /// cash or whether the underlying instruments are delivered).
        pub delivery: Delivery, 3;

        /// Standardized/non-standardized (indicates whether the terms of the contract are
        /// standardized or not).
        pub standardized: Standardized, 4;
    }
}

macros::impl_group! {
    /// Put options.
    ///
    /// Contracts between a buyer and a seller giving the buyer (holder) the right, but not the
    /// obligation, to sell the assets specified at a fixed price or formula, on or before a
    /// specified date. The seller of the put option assumes the obligation of buying the assets
    /// specified should the buyer exercise his or her option.
    pub struct Put {
        /// Exercise option style.
        pub exercise_style: ExerciseStyle, 1;

        /// Underlying assets (indicates the type of underlying assets that the option holder is
        /// entitled to acquire).
        pub underlying: Underlying, 2;

        /// Delivery (indicates whether the settlement of the option, when exercised, is made in
        /// cash or whether the underlying instruments are delivered).
        pub delivery: Delivery, 3;

        /// Standardized/non-standardized (indicates whether the terms of the contract are
        /// standardized or not).
        pub standardized: Standardized, 4;
    }
}

macros::impl_group! {
    /// Others (miscellaneous).
    ///
    /// Options that do not fit into any of the Groups of the listed options.
    pub struct OtherListed {
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

macros::impl_group! {
    /// Rates.
    ///
    /// An option where the holder of the option has the right but not the obligation to enter into
    /// the underlying contract, or pay or receive payment related to the underlying rate on a
    /// specified future date based on a specified future rate and term.
    pub struct Rate {
        /// Underlying assets.
        pub underlying: RateUnderlying, 1;

        /// Option style and type.
        pub style: UnlistedStyle, 2;

        /// Valuation method or trigger.
        pub valuation: RateValuation, 3;

        /// Delivery.
        pub delivery: UnlistedDelivery, 4;
    }
}

macros::impl_group! {
    /// Commodities.
    ///
    /// An option where the option buyer has the right to buy or sell specified commodities assets
    /// at a fixed price or formula, on or before a specified date.
    pub struct Commodity {
        /// Underlying assets.
        pub underlying: CommodityUnderlying, 1;

        /// Option style and type.
        pub style: UnlistedStyle, 2;

        /// Valuation method or trigger.
        pub valuation: UnlistedValuation, 3;

        /// Delivery.
        pub delivery: UnlistedDelivery, 4;
    }
}

macros::impl_group! {
    /// Equity.
    ///
    /// An option where the underlying asset is an equity-linked instrument (i.e. shares,
    /// depository receipts, ETFs, indices, baskets).
    pub struct Equity {
        /// Underlying assets.
        pub underlying: EquityUnderlying, 1;

        /// Option style and type.
        pub style: UnlistedStyle, 2;

        /// Valuation method or trigger.
        pub valuation: UnlistedValuation, 3;

        /// Delivery.
        pub delivery: UnlistedDelivery, 4;
    }
}

macros::impl_group! {
    /// Credit.
    ///
    /// An option to buy or sell a credit product which is a contract in which one party
    /// (protection seller) agrees to provide payment to the other party (protection buyer) should
    /// a credit event occur against the underlying, which could be a specified debt (the reference
    /// obligation), a specific debt issuer (reference entity), a basket of reference entities
    /// and/or reference obligations, or a credit index (reference index).
    pub struct Credit {
        /// Underlying assets.
        pub underlying: CreditUnderlying, 1;

        /// Option style and type.
        pub style: UnlistedStyle, 2;

        /// Valuation method or trigger.
        pub valuation: UnlistedValuation, 3;

        /// Delivery.
        pub delivery: UnlistedDelivery, 4;
    }
}

macros::impl_group! {
    /// Foreign exchange.
    ///
    /// An option to buy or sell a foreign exchange agreement between two parties to exchange a
    /// given amount of one currency for another currency for spot delivery or for forward delivery
    /// at an agreed rate after a specified period of time.
    pub struct Forex {
        /// Underlying assets.
        pub underlying: ForexUnderlying, 1;

        /// Option style and type.
        pub style: UnlistedExerciseStyle, 2;

        /// Valuation method or trigger.
        pub valuation: UnlistedValuation, 3;

        /// Delivery.
        pub delivery: UnlistedDelivery, 4;
    }
}

macros::impl_group! {
    /// Others (miscellaneous).
    ///
    /// Options that do not fit into any of the above Groups.
    pub struct OtherUnlisted {
        /// Underlying assets.
        pub underlying: OtherUnderlying, 1;

        /// Option style and type.
        pub style: UnlistedStyle, 2;

        /// Valuation method or trigger.
        pub valuation: UnlistedValuation, 3;

        /// Delivery.
        pub delivery: OtherDelivery, 4;
    }
}

macros::impl_attr! {
    /// Exercise option style.
    pub enum ExerciseStyle[2] {
        /// European.
        European = b'E', "E";

        /// American.
        American = b'A', "A";

        /// Bermudan.
        Bermudan = b'B', "B";
    }
}

macros::impl_attr! {
    /// Underlying assets.
    ///
    /// Indicates the type of underlying assets that the option holder is entitled to acquire.
    pub enum Underlying[3] {
        /// Baskets.
        Basket = b'B', "B";

        /// Stock-equities.
        Stock = b'S', "S";

        /// Debt instruments.
        Debt = b'D', "D";

        /// Commodities.
        Commodity = b'T', "T";

        /// Currencies.
        Currency = b'C', "C";

        /// Indices.
        Index = b'I', "I";

        /// Options.
        Option = b'O', "O";

        /// Futures.
        Future = b'F', "F";

        /// Swaps.
        Swap = b'W', "W";

        /// Interest rates.
        InterestRate = b'N', "N";

        /// Others (miscellaneous).
        Other = b'M', "M";
    }
}

macros::impl_attr! {
    /// Delivery.
    ///
    /// Indicates whether the settlement of the option, when exercised, is made in cash or whether
    /// the underlying instruments are delivered.
    pub enum Delivery[4] {
        /// Physical (the meeting of a settlement obligation under a derivative contract through
        /// the receipt or delivery of the actual underlying instrument(s) instead of through
        /// cash settlement).
        Physical = b'P', "P";

        /// Cash (the discharge of an obligation by payment or receipt of a net cash amount instead
        /// of payment or delivery by both parties).
        Cash = b'C', "C";

        /// Non-deliverable (synthetic options on foreign exchange forwards that are based on
        /// non-convertible or thinly traded currencies).
        NonDeliverable = b'N', "N";

        /// Elect at exercise (the method of delivery of the underlying instrument when the option
        /// is exercised shall be determined at the time of exercise).
        ElectAtExercise = b'E', "E";
    }
}

macros::impl_attr! {
    /// Underlying rate assets.
    pub enum RateUnderlying[2] {
        /// Basis swap (float-float).
        Basis = b'A', "A";

        /// Fixed-floating swap.
        FixedFloating = b'C', "C";

        /// Fixed-fixed swap.
        FixedFixed = b'D', "D";

        /// Interest rate index.
        Interest = b'E', "E";

        /// Inflation rate index.
        Inflation = b'I', "I";

        /// QIS.
        Qis = b'H', "H";

        /// Options.
        Option = b'O', "O";

        /// Forwards.
        ///
        /// Derivatives involving the exchange of two rates on a defined future date, as agreed by
        /// the two parties to the transaction.
        Forwards = b'R', "R";

        /// Futures.
        Futures = b'F', "F";

        /// Others (miscellaneous).
        Other = b'M', "M";
    }
}

macros::impl_attr! {
    /// Option style and type.
    pub enum UnlistedStyle[3] {
        /// European-Call.
        ///
        /// An option on a contract which allows its holder (buyer) to exercise the right to buy
        /// specified assets (interest rates product) at a fixed price only on the expiration date
        /// of the call.
        EuropeanCall = b'A', "A";

        /// American-Call.
        ///
        /// An option on a contract which allows its holder (buyer) to exercise the right to buy
        /// specified assets (interest rates product) at a fixed price at any time during the term
        /// of the call option, up to and including the expiration date of the call.
        AmericanCall = b'B', "B";

        /// Bermudan-Call.
        ///
        /// An option on a contract which allows its holder (buyer) to exercise the right to buy
        /// specified assets (interest rates product) at a fixed price on a number of specific
        /// dates within the exercise period of the call.
        BermudanCall = b'C', "C";

        /// European-Put.
        ///
        /// An option on a contract which allows its holder (buyer) to exercise the right to sell
        /// specified assets (interest rates product) at a fixed price only on the expiration date
        /// of the put.
        EuropeanPut = b'D', "D";

        /// American-Put.
        ///
        /// An option on a contract which allows its holder (buyer) to exercise the right to sell
        /// specified assets (interest rates product) at a fixed price at any time during the term
        /// of the put option, up to and including the expiration date of the put.
        AmericanPut = b'E', "E";

        /// Bermudan-Put.
        ///
        /// An option on a contract which allows its holder (buyer) to exercise the right to sell
        /// specified assets (interest rates product) at a fixed price on a number of specific
        /// dates within the exercise period of the put.
        BermudanPut = b'F', "F";

        /// European-Chooser.
        ///
        /// An option on a contract which allows its holder (buyer) to exercise the right to buy
        /// (call) or sell (put) specified assets (interest rates product) at a fixed price, only
        /// on the contract's expiration date; the buyer does not have to decide whether the
        /// contract will be a put or a call until an agreed future date, prior to expiration.
        EuropeanChooser = b'G', "G";

        /// American-Chooser.
        ///
        /// An option on a contract which allows its holder (buyer) to exercise the right to buy
        /// (call) or sell (put) specified assets (interest rates product) at a fixed price at any
        /// time during the term of the contract, up to and including the expiration date of the
        /// call or put; the buyer does not have to decide whether the contract will be a put or a
        /// call until an agreed future date, prior to expiration.
        AmericanChooser = b'H', "H";

        /// Bermudan-Chooser.
        ///
        /// An option on a contract which allows its holder (buyer) to exercise the right to buy
        /// (call) or sell (put) specified assets (interest rates product) at a fixed price on a
        /// number of specific dates within the exercise period of the contract; the buyer does not
        /// have to decide whether the contract will be a put or a call until an agreed future
        /// date, prior to expiration.
        BermudanChooser = b'I', "I";
    }
}

macros::impl_attr! {
    /// Valuation method or trigger for rates.
    pub enum RateValuation[4] {
        /// Vanilla.
        ///
        /// An option for which all terms are standardized.
        Vanilla = b'V', "V";

        /// Asian.
        ///
        /// An option where either the strike price or the settlement price is the average level of
        /// an underlying instrument over a predetermined period; the averaging can be either a
        /// geometric or arithmetic average.
        Asian = b'A', "A";

        /// Digital (Binary).
        ///
        /// An option that has a pre-determined payout if the option is in-the-money and the payoff
        /// condition is satisfied; also referred to as a “binary option” or an “all-or-nothing
        /// option”.
        Digital = b'D', "D";

        /// Barrier.
        ///
        /// An option whose final exercise depends upon the path taken by the price of an
        /// underlying instrument; for a “knock-out” barrier option, the option is cancelled if the
        /// underlying price crosses a predetermined barrier level; for a “knock-in” barrier
        /// option, the option becomes available-for-exercise if the underlying price crosses a
        /// predetermined barrier level.
        Barrier = b'B', "B";

        /// Digital barrier.
        ///
        /// A digital option embedded with a barrier option; there are different variations of this
        /// type of option; as an example, a down-and-out digital call option will pay a fixed
        /// payoff, or the underlying, at any time before maturity that the underlying price is
        /// equal to or greater than the barrier level; it will pay zero if the underlying price is
        /// less than the barrier level.
        DigitalBarrier = b'G', "G";

        /// Lookback.
        ///
        /// An option that minimizes the uncertainties related to the timing of market entry; there
        /// are two types of lookback options: fixed and floating; the fixed option strike is
        /// determined at purchase, and the floating option strike is determined at maturity.
        Lookback = b'L', "L";

        /// Other path dependent.
        ///
        /// An option on a contract whose payoff is directly related to the price pattern the
        /// underlying asset follows during the life of the contract.
        PathDependent = b'P', "P";

        /// Cap.
        ///
        /// An option in which the payment is triggered when the value of the underlier exceeds a
        /// specified level.
        Cap = b'C', "C";

        /// Floor.
        ///
        /// An option in which the payment is triggered when the value of the underlier falls below
        /// a specified level.
        Floor = b'F', "F";

        /// Others (miscellaneous).
        Other = b'M', "M";
    }
}

macros::impl_attr! {
    /// Delivery.
    pub enum UnlistedDelivery[5] {
        /// Cash.
        Cash = b'C', "C";

        /// Physical.
        Physical = b'P', "P";

        /// Elect at settlement (determined at the time of settlement).
        ElectAtSettlement = b'E', "E";
    }
}

macros::impl_attr! {
    /// Underlying commodity assets.
    pub enum CommodityUnderlying[2] {
        /// Energy.
        Energy = b'J', "J";

        /// Metals.
        Metals = b'K', "K";

        /// Agriculture.
        Agriculture = b'A', "A";

        /// Environmental.
        Environmental = b'N', "N";

        /// Freight.
        Freight = b'G', "G";

        /// Polypropylene products.
        Polypropylene = b'P', "P";

        /// Fertilizer.
        Fertilizer = b'S', "S";

        /// Paper.
        Paper = b'T', "T";

        /// Index – single-commodity.
        ///
        /// An option where the underlying reference entity is a commodity index.
        SingleIndex = b'I', "I";

        /// Index – multi-commodity.
        ///
        /// An index containing constituents from two or more of the underlying assets identified
        /// for this attribute.
        MultiIndex = b'H', "H";

        /// Basket – single-commodity.
        ///
        /// A custom basket containing constituents from one of the underlying assets identified
        /// for this attribute.
        SingleBasket = b'B', "B";

        /// Basket – multi-commodity.
        ///
        /// A custom basket containing constituents from two or more of the underlying assets
        /// identified for this attribute.
        MultiBasket = b'C', "C";

        /// Options.
        Options = b'O', "O";

        /// Forwards.
        Forwards = b'R', "R";

        /// Swaps.
        Swaps = b'W', "W";

        /// Others (miscellaneous).
        Other = b'M', "M";
    }
}

macros::impl_attr! {
    /// Valuation method or trigger for unlisted options.
    pub enum UnlistedValuation[4] {
        /// Vanilla.
        ///
        /// An option for which all terms are standardized.
        Vanilla = b'V', "V";

        /// Asian.
        ///
        /// An option where either the strike price or the settlement price is the average level of
        /// an underlying instrument over a predetermined period; the averaging can be either a
        /// geometric or arithmetic average.
        Asian = b'A', "A";

        /// Digital (Binary).
        ///
        /// An option that has a pre-determined payout if the option is in-the-money and the payoff
        /// condition is satisfied; also referred to as a “binary option” or an “all-or-nothing
        /// option”.
        Digital = b'D', "D";

        /// Barrier.
        ///
        /// An option whose final exercise depends upon the path taken by the price of an
        /// underlying instrument; for a “knock-out” barrier option, the option is cancelled if the
        /// underlying price crosses a predetermined barrier level; for a “knock-in” barrier
        /// option, the option becomes available-for-exercise if the underlying price crosses a
        /// predetermined barrier level.
        Barrier = b'B', "B";

        /// Digital barrier.
        ///
        /// A digital option embedded with a barrier option; there are different variations of this
        /// type of option; as an example, a down-and-out digital call option will pay a fixed
        /// payoff, or the underlying, at any time before maturity that the underlying price is
        /// equal to or greater than the barrier level; it will pay zero if the underlying price is
        /// less than the barrier level.
        DigitalBarrier = b'G', "G";

        /// Lookback.
        ///
        /// An option that minimizes the uncertainties related to the timing of market entry; there
        /// are two types of lookback options: fixed and floating; the fixed option strike is
        /// determined at purchase, and the floating option strike is determined at maturity.
        Lookback = b'L', "L";

        /// Other path dependent.
        ///
        /// An option on a contract whose payoff is directly related to the price pattern the
        /// underlying asset follows during the life of the contract.
        PathDependent = b'P', "P";

        /// Others (miscellaneous).
        Other = b'M', "M";
    }
}

macros::impl_attr! {
    /// Underlying equity assets.
    pub enum EquityUnderlying[2] {
        /// Single stock.
        ///
        /// An option on a contract which gives the holder the right to buy, and to sell,
        /// single-named equity.
        Stock = b'S', "S";

        /// Index.
        ///
        /// An option on a contract which gives the holder the right to buy, and to sell, specified
        /// equity indices.
        Index = b'I', "I";

        /// Basket.
        ///
        /// An option on a contract that may be exercised based on the weighted average performance
        /// of several underlying equities instruments.
        Basket = b'B', "B";

        /// Options.
        Option = b'O', "O";

        /// Forwards.
        Forward = b'R', "R";

        /// Futures.
        Future = b'F', "F";

        /// Others (miscellaneous).
        Other = b'M', "M";
    }
}

macros::impl_attr! {
    /// Underlying credit assets.
    pub enum CreditUnderlying[2] {
        /// CDS on a single name.
        ///
        /// A CDS where the underlying risk is a single reference entity or single reference
        /// obligation.
        SingleName = b'U', "U";

        /// CDS on an index tranche.
        ///
        /// A synthetic CDO based on a CDS index where each tranche references a different segment
        /// of the loss distribution of the underlying CDS index; each tranche has a different
        /// priority of claims on the principal and interest flows from the collateral pool, and is
        /// traditionally portioned into rising levels of seniority.
        IndexTranche = b'V', "V";

        /// CDS on an index.
        ///
        /// Family of standardized credit derivative indices, where the underlying reference
        /// entities are a defined basket of credit from a particular geographic region (e.g. Asia,
        /// North America, Europe), and/or credit rating level (e.g. emerging markets, high yield,
        /// investment grade); credit default indices trade in standard maturities, and the
        /// reference entities are typically the most liquid; the reference portfolio is reassessed
        /// periodically to maintain this.
        Index = b'I', "I";

        /// Swaps (a swap other than a CDS).
        Swap = b'W', "W";

        /// Others (miscellaneous).
        Other = b'M', "M";
    }
}

macros::impl_attr! {
    /// Underlying foreign exchange assets.
    pub enum ForexUnderlying[2] {
        /// Forwards – Single Currency Pair.
        PairForward = b'R', "R";

        /// Futures – Single Currency Pair.
        PairFuture = b'F', "F";

        /// Spot – Single Currency Pair.
        ///
        /// An option on a foreign exchange transaction in which two parties agree to buy one
        /// currency against selling another currency at an agreed price for settlement on the spot
        /// date.
        PairSpot = b'T', "T";

        /// Volatility – Single Currency Pair (please refer to 6.8.4).
        PairVolatility = b'V', "V";

        /// Forwards – Currency Index.
        IndexForward = b'B', "B";

        /// Futures – Currency Index.
        IndexFuture = b'C', "C";

        /// Spot – Currency Index.
        IndexSpot = b'D', "D";

        /// Volatility – Currency Index
        IndexVolatility = b'E', "E";

        /// Forwards – Custom Basket of Currencies.
        BasketForward = b'Q', "Q";

        /// Futures – Custom Basket of Currencies.
        BasketFuture = b'U', "U";

        /// Spot – Custom Basket of Currencies
        BasketSpot = b'W', "W";

        /// Volatility – Custom Basket of Currencies.
        BasketVolatility = b'Y', "Y";

        /// Others (miscellaneous).
        Other = b'M', "M";
    }
}

macros::impl_attr! {
    /// Unlisted option exercise style.
    pub enum UnlistedExerciseStyle[3] {
        /// European.
        ///
        /// An option which allows its holder to exercise the right to buy/sell the specified
        /// call/put currency at a specified price only on the expiration date.
        European = b'J', "J";

        /// American.
        ///
        /// An option which allows its holder to exercise the right to buy/sell the specified
        /// call/put currency at a specified price at any time during the term of the option, up to
        /// and including the expiration date.
        American = b'K', "K";

        /// Bermudan.
        ///
        /// An option which allows its holder to exercise the right to buy/sell the specified
        /// call/put currency at a specified price on a number of specific dates within the
        /// exercise period of the option.
        Bermudan = b'L', "L";
    }
}

macros::impl_attr! {
    /// Underying assets for other unlisted options.
    pub enum OtherUnderlying[2] {
        /// Commercial property (or property derivative).
        Commercial = b'P', "P";

        /// Others (miscellaneous).
        Other = b'M', "M";
    }
}

macros::impl_attr! {
    /// Delivery styles for other unlisted options.
    pub enum OtherDelivery[5] {
        /// Cash.
        Cash = b'C', "C";

        /// Physical.
        Physical = b'P', "P";

        /// Elect at exercise.
        ElectAtExercise = b'E', "E";

        /// Non-deliverable.
        NonDeliverable = b'N', "N";

        /// Auction.
        Auction = b'A', "A";
    }
}
