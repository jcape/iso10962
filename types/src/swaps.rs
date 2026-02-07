//! Swaps category support.

use crate::{
    NotApplicable,
    macros::{self, impl_attr},
};

macros::impl_category! {
    /// Swaps.
    ///
    /// A swap is an agreement or contract where two counterparties agree to exchange periodic
    /// streams of cash flows with each other. Swaps can be executed with a variety of asset
    /// classes, as listed below.
    pub enum Swap {
        /// Rates.
        ///
        /// A rates swap is a contract in which two counterparties each agree to pay the other cash
        /// flows on defined dates during an agreed period, based on a specified notional amount
        /// and a floating interest, floating inflation or fixed interest rate.
        Rate(Rate) = b'R', "R";

        /// Commodities.
        ///
        /// A commodity swap is a derivative contract where the value of the contract is derived
        /// from an underlying commodity or commodity index. Commodity derivatives can be
        /// physically settled or cash settled. Primary underliers include metals, agricultural
        /// goods and energy.
        Commodity(Commodity) = b'T', "T";

        /// Equity.
        ///
        /// An equity swap is a derivative contract where payments are linked to the change in
        /// value of an underlying equity (e.g. shares, basket of equities or index). Equity swaps
        /// can be physically or cash settled.
        Equity(Equity) = b'E', "E";

        /// Credit.
        ///
        /// A credit swap references a value or event related to a debt product or debt issuer.
        Credit(Credit) = b'C', "C";

        /// Foreign exchange.
        ///
        /// A foreign exchange swap is a foreign exchange agreement between two parties to exchange
        /// a given amount of one currency for another currency for spot delivery or for forward
        /// delivery at an agreed rate after a specified period of time.
        Forex(Forex) = b'F', "F";

        /// Others (miscellaneous).
        ///
        /// Swaps that do not fit into any of the Swaps Groups.
        Other(Other) = b'M', "M";
    }
}

macros::impl_group! {
    /// `SR`: Rates.
    ///
    /// A rates swap is a contract in which two counterparties each agree to pay the other cash
    /// flows on defined dates during an agreed period, based on a specified notional amount and a
    /// floating interest, floating inflation or fixed interest rate.
    pub struct Rate {
        /// Underlying assets.
        pub underlying: RateUnderlying, 1;

        /// Notional (indicates the face amount of a swap upon which the payment streams for that
        /// swap are based).
        pub notional: Notional, 2;

        /// Single or multi-currency (indicates whether the swap is single or multi-currency).
        pub currency_kind: RateCurrency, 3;

        /// Delivery (indicates whether the payment currency for each leg of the swap is the same
        /// as the reference currency for that leg).
        pub delivery: RateDelivery, 4;
    }
}

macros::impl_group! {
    /// `ST`: Commodities.
    ///
    /// A commodity swap is a derivative contract where the value of the contract is derived from
    /// an underlying commodity or commodity index. Commodity derivatives can be physically settled
    /// or cash settled. Primary underliers include metals, agricultural goods and energy.
    pub struct Commodity {
        /// Underlying assets.
        pub underlying: CommodityUnderlying, 1;

        /// Return or payout trigger (method used to determine contract value).
        pub payout: CommodityPayout, 2;

        /// Not applicable/undefined.
        pub attr3: NotApplicable, 3;

        /// Delivery.
        pub delivery: CommodityDelivery, 4;
    }
}

macros::impl_group! {
    /// `SE`: Equity.
    ///
    /// An equity swap is a derivative contract where payments are linked to the change in value of
    /// an underlying equity (e.g. shares, basket of equities or index). Equity swaps can be
    /// physically or cash settled.
    pub struct Equity {
        /// Underlying assets.
        pub underlying: EquityUnderlying, 1;

        /// Return or payout trigger.
        pub payout: EquityPayout, 2;

        /// Not applicable/undefined.
        pub attr3: NotApplicable, 3;

        /// Delivery.
        pub delivery: EquityDelivery, 4;
    }
}

macros::impl_group! {
    /// `SC`: Credit.
    ///
    /// A credit swap references a value or event related to a debt product or debt issuer.
    pub struct Credit {
        /// Underlying assets.
        pub underlying: CreditUnderlying, 1;

        /// Return or payout trigger.
        pub payout: CreditPayout, 2;

        /// Underlying issuer type.
        pub issuer: CreditIssuer, 3;

        /// Delivery.
        pub delivery: CreditDelivery, 4;
    }
}

macros::impl_group! {
    /// `SF`: Foreign exchange.
    ///
    /// A foreign exchange swap is a foreign exchange agreement between two parties to exchange a
    /// given amount of one currency for another currency for spot delivery or for forward delivery
    /// at an agreed rate after a specified period of time.
    pub struct Forex {
        /// Underlying assets.
        pub underlying: ForexUnderlying, 1;

        /// Not applicable/undefined.
        pub attr2: NotApplicable, 2;

        /// Not applicable/undefined.
        pub attr3: NotApplicable, 3;

        /// Delivery.
        pub delivery: ForexDelivery, 4;
    }
}

macros::impl_group! {
    /// `SM`: Others (miscellaneous).
    ///
    /// Swaps that do not fit into any of the Swaps Groups.
    pub struct Other {
        /// Underlying assets.
        pub underlying: OtherUnderlying, 1;

        /// Not applicable/undefined.
        pub attr2: NotApplicable, 2;

        /// Not applicable/undefined.
        pub attr3: NotApplicable, 3;

        /// Delivery.
        pub delivery: OtherDelivery, 4;
    }
}

impl_attr! {
    /// Underlying rate assets.
    pub enum RateUnderlying[2] {
        /// Basis swap (float-float).
        ///
        /// A rate swap where the cash flows that are exchanged between each party are based on
        /// different floating interest rates or prices (i.e. one party pays an agreed floating
        /// rate multiplied by a notional amount, in exchange for receipt of periodic payments
        /// based on another agreed floating rate multiplied by the same notional amount, from the
        /// other party), except those swaps covered by the definitions below for attributes
        /// [`G`](Self::Inflation) or [`H`](Self::OvernightIndex).
        Basis = b'A', "A";

        /// Fixed-floating.
        ///
        /// A rate swap in which one party (the fixed rate payer) agrees to make fixed payments
        /// (the fixed leg) on set dates for an agreed period to another party (the floating rate
        /// payer), based on a fixed interest rate multiplied by a notional amount, in exchange for
        /// receipt of periodic payments (the floating leg), from the floating rate payer, based on
        /// a floating interest rate index multiplied by the same notional amount (in most cases)
        /// upon which the fixed rate payments are based], except those swaps covered by the
        /// definitions below for attributes [`G`](Self::Inflation), [`H`](Self::OvernightIndex) or
        /// [`Z`](Self::ZeroCoupon).
        FixedFloating = b'C', "C";

        /// Fixed-fixed.
        ///
        /// A rate swap in which both parties pay a fixed interest rate that they could not
        /// otherwise obtain outside of a swap arrangement; for example, if each counterparty uses
        /// a different native currency, but wants to borrow money in the other counterparty’s
        /// native currency; fixed-fixed swaps generally take the form of either a zero coupon swap
        /// or a cross-currency swap), except those swaps covered by the definitions below for
        /// attributes [`G`](Self::Inflation) or [`Z`](Self::ZeroCoupon).
        FixedFixed = b'D', "D";

        /// Inflation swap.
        ///
        /// A rate swap in which one party pays an amount calculated using an inflation rate index,
        /// and the other party pays an amount calculated using another inflation rate index, or a
        /// fixed or floating interest rate.
        Inflation = b'G', "G";

        /// Overnight index swap (OIS).
        ///
        /// A rate swap in which one party (the fixed rate payer) makes periodic payments to
        /// another party (the floating rate payer) based on a fixed interest rate (other than
        /// zero) or floating interest rate multiplied by a notional amount in exchange for receipt
        /// of periodic payments based on an overnight interest rate index multiplied by the same
        /// notional amount upon which the fixed rate payments are based.
        OvernightIndex = b'H', "H";

        /// Zero coupon.
        ///
        /// A rate swap in which the fixed rate cash flows are compounded and paid once on the
        /// expiration data, rather than periodically; the payments on the other side (which can be
        /// based on a floating interest rate or a fixed rate) follow typical swap payment
        /// schedules.
        ZeroCoupon = b'Z', "Z";

        /// Others (miscellaneous).
        Other = b'M', "M";
    }
}

macros::impl_attr! {
    /// Notional.
    ///
    /// Indicates the face amount of a swap upon which the payment streams for that swap are based.
    pub enum Notional[3] {
        /// Constant.
        ///
        /// The notional amount is constant through the life of the contract.
        Constant = b'C', "C";

        /// Accreting.
        ///
        /// The notional amount increases through the life of the contract.
        Acreting = b'I', "I";

        /// Amortizing.
        ///
        /// The notional amount decreases through the life of the contract.
        Amortizing = b'D', "D";

        /// Custom.
        ///
        /// Customized notional step schedule.
        Custom = b'Y', "Y";
    }
}

macros::impl_attr! {
    /// Single or multi-currency.
    ///
    /// Indicates whether the swap is single or multi-currency.
    pub enum RateCurrency[4] {
        /// Single currency.
        Single = b'S', "S";

        /// Cross-currency (multi-currency).
        Cross = b'C', "C";
    }
}

macros::impl_attr! {
    /// Rate Delivery.
    ///
    /// Indicates whether the payment currency for each leg of the swap is the same as the
    /// reference currency for that leg.
    pub enum RateDelivery[5] {
        /// Deliverable.
        ///
        /// The settlement, i.e. payment, currency amounts are paid in the respective reference
        /// currency for each leg of the swap for which the payments are being made.
        Deliverable = b'D', "D";

        /// Non-deliverable.
        ///
        /// The settlement, i.e. payment, currency amounts are paid in a single currency that
        /// either reflects the currency of either leg of the swap or in a currency other than the
        /// respective reference currency for each leg of the swap for which the payments are being
        /// made.
        NonDeliverable = b'N', "N";
    }
}

macros::impl_attr! {
    /// Underlying commodity assets.
    pub enum CommodityUnderlying[2] {
        /// Energy.
        ///
        /// An energy-related product, or a derivative of an energy-related product, including
        /// electricity, renewable energy, or any power/energy delivered through a utility network
        /// of provider; diesel fuel, fuel oil, gas oil, gasoline, heating oil, jet fuel, kerosene,
        /// natural gas, oil (Brent, Tapis, Dubai, WTI).
        Energy = b'J', "J";

        /// Metals.
        ///
        /// A precious or industrial metal, such as aluminium, copper, gold, lead, nickel,
        /// platinum, silver, tin, zinc.
        Metals = b'K', "K";

        /// Agriculture.
        ///
        /// Commodities which include forestry, fishing, livestock, grain, dairy, corn, cocoa,
        /// soybeans, sugar, coffee.
        Agriculture = b'A', "A";

        /// Environmental.
        ///
        /// Includes carbon-related, emission reduction, weather.
        Environmental = b'N', "N";

        /// Freight.
        ///
        /// The specified commodity is a freight index route.
        Freight = b'G', "G";

        /// Polypropylene products.
        Polypropylene = b'P', "P";

        /// Fertilizer.
        ///
        /// Ammonia, diammonium phosphate (DAP), potash, sulphur, urea, urea and ammonium nitrate
        /// (UAN).
        Fertilizer = b'S', "S";

        /// Paper.
        ///
        /// Containerboard, newsprint, pulp, recovered paper.
        Paper = b'T', "T";

        /// Index – single-commodity.
        ///
        /// An index containing constituents from one of the underlying assets identified for this
        /// attribute.
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

        /// Multi-commodity.
        ///
        /// Each leg of the swap references a different respective commodity than the other leg.
        Multi = b'Q', "Q";

        /// Others (miscellaneous).
        Other = b'M', "M";
    }
}

macros::impl_attr! {
    /// Return or payout trigger.
    ///
    /// Method used to determine contract value.
    pub enum CommodityPayout[3] {
        /// Contract for difference (CFD).
        ///
        /// A cash-settled total return swap or forward where the parties agree to exchange on the
        /// maturity of the contract the difference between the opening price and closing price of
        /// the underlying.
        Cfd = b'C', "C";

        /// Total return.
        ///
        /// The total economic return of an underlying asset is transferred from one party (total
        /// return buyer) to another (total return seller); total return seller takes on the risk
        /// of negative changes in market value of the reference asset, and pays any positive cash
        /// flow to the buyer such as coupon, capital gains or dividends of the reference asset.
        TotalReturn = b'T', "T";
    }
}

macros::impl_attr! {
    /// Commodity Delivery.
    pub enum CommodityDelivery[5] {
        /// Cash.
        Cash = b'C', "C";

        /// Physical.
        Physical = b'P', "P";

        /// Elect at settlement (determined at the time of settlement).
        ElectAtSettlement = b'E', "E";
    }
}

macros::impl_attr! {
    /// Underlying equity assets.
    pub enum EquityUnderlying[2] {
        /// Single stock (single name security).
        Single = b'S', "S";

        /// Index.
        ///
        /// A synthetic portfolio of underlying assets whose components have been set by a
        /// third-party administrator.
        Index = b'I', "I";

        /// Basket.
        ///
        /// A bespoke, synthetic portfolio of underlying assets whose components have been agreed
        /// to for a specific OTC derivative by the parties to the transaction.
        Basket = b'B', "B";

        /// Others (miscellaneous).
        Other = b'M', "M";
    }
}

macros::impl_attr! {
    /// Equity return or payout trigger.
    pub enum EquityPayout[3] {
        /// Price.
        ///
        /// Price return equity swap; similar to a total return swap, except that dividends are not
        /// passed through to the buyer.
        Price = b'P', "P";

        /// Dividend.
        ///
        /// A fixed-term contract between two parties where one party will make an interest rate
        /// payment for each interval and the other party will pay the total dividends received as
        /// pay-out by a selected underlying asset.
        Dividend = b'D', "D";

        /// Variance.
        ///
        /// Forward swap that uses the variance (being the volatility squared) of an underlying’s
        /// price movement over a period as the basis for the payoff calculation.
        Variance = b'V', "V";

        /// Volatility.
        ///
        /// The variability of movements in a security or underlying instrument’s price; it is a
        /// measure of the amount by which an asset’s price is expected to fluctuate over a given
        /// period of time; it is normally measured by the annual standard deviation of daily price
        /// changes.
        Volatility = b'L', "L";

        /// Total return.
        TotalReturn = b'T', "T";

        /// CFD.
        Cfd = b'C', "C";

        /// Others (miscellaneous).
        Other = b'M', "M";
    }
}

macros::impl_attr! {
    /// Equity Delivery.
    pub enum EquityDelivery[5] {
        /// Cash.
        Cash = b'C', "C";

        /// Physical.
        Physical = b'P', "P";

        /// Elect at settlement (determined at the time of settlement).
        ElectAtSettlement = b'E', "E";
    }
}

macros::impl_attr! {
    /// Underlying credit assets.
    pub enum CreditUnderlying[2] {
        /// Single name.
        ///
        /// The underlying risk is a single reference entity or reference obligation.
        Single = b'U', "U";

        /// Index tranche.
        ///
        /// A synthetic collateralized debt obligation (CDO) based on a credit index where each
        /// tranche references a different segment of the loss distribution of the underlying
        /// index; each tranche has a different priority of claims on the principal and interest
        /// flows from the collateral pool, and is traditionally portioned into rising levels of
        /// seniority.
        IndexTranche = b'V', "V";

        /// Index.
        ///
        /// Family of standardized credit derivative indices, where the underlying reference
        /// entities are a defined basket of credit from a particular geographic region (e.g. Asia,
        /// North America, Europe), and/or credit rating level (e.g. emerging markets, high yield,
        /// investment grade); credit default indices trade in standard maturities, and the
        /// reference entities are typically the most liquid; the reference portfolio is reassessed
        /// periodically to maintain this.
        Index = b'I', "I";

        /// Basket.
        ///
        /// A bespoke, synthetic portfolio of underlying assets whose components have been agreed
        /// to for a specific OTC derivative by the parties to the transaction.
        Basket = b'B', "B";

        /// Others (miscellaneous).
        Other = b'M', "M";
    }
}

macros::impl_attr! {
    /// Credit return or payout trigger.
    pub enum CreditPayout[3] {
        /// Credit default.
        ///
        /// A credit default swap (CDS) is a contract in which one party (protection seller) agrees
        /// to provide payment to the other party (protection buyer) should a credit event occur
        /// against the underlying, which could be a specified debt (the reference obligation), a
        /// specific debt issuer (reference entity), a basket of reference entities and/or
        /// reference obligations or a credit index (reference index).
        Default = b'C', "C";

        /// Total return.
        TotalReturn = b'T', "T";

        /// Others (miscellaneous).
        Other = b'M', "M";
    }
}

macros::impl_attr! {
    /// Underlying issuer type.
    pub enum CreditIssuer[4] {
        /// Corporate entity.
        ///
        /// The underlying exposure is a corporate (a private sector entity).
        Corporate = b'C', "C";

        /// Sovereign entity.
        ///
        /// The underlying exposure is a sovereign, e.g. country; thus, investor’s risk is that a
        /// country may not (be able to) pay its debt obligations; supranationals would be included
        /// here.
        Sovereign = b'S', "S";

        /// Local.
        ///
        /// A municipality or local government authority.
        Local = b'L', "L";
    }
}

macros::impl_attr! {
    /// Credit Delivery.
    pub enum CreditDelivery[5] {
        /// Cash.
        Cash = b'C', "C";

        /// Physical.
        Physical = b'P', "P";

        /// Auction.
        ///
        /// An independently administered synthetic auction process on a set of defined deliverable
        /// obligations that sets a reference final price that can be used to facilitate cash
        /// settlement of all covered transactions following a credit event.
        Auction = b'A', "A";
    }
}

macros::impl_attr! {
    /// Forex underlying.
    pub enum ForexUnderlying[2] {
        /// Spot-forward swap.
        ///
        /// A transaction that involves both an exchange of two currencies on the spot settlement
        /// date at a fixed rate that is agreed upon at the inception of the contract covering the
        /// exchange; and a reverse exchange of the same two currencies at a later date and at a
        /// fixed rate that is agreed upon at the inception of the contract covering the exchange.
        SpotForward = b'A', "A";

        /// Forward-forward swap.
        ///
        /// A transaction that involves both an exchange of two currencies on a specified future
        /// date at a fixed rate that is agreed upon at the inception of the contract covering the
        /// exchange; and a reverse exchange of the same two currencies at a further future date,
        /// at a fixed rate that is agreed upon at the inception of the contract covering the
        /// exchange, e.g. a swap between the 3-month forward and 6-month forward dates.
        ForwardForward = b'C', "C";

        /// Others (miscellaneous).
        Other = b'M', "M";
    }
}

macros::impl_attr! {
    /// Forex delivery.
    pub enum ForexDelivery[5] {
        /// Physical (delivery of traded currencies on settlement date).
        Physical = b'P', "P";

        /// Cash.
        ///
        /// On settlement date of the trade, if the net settlement amount is positive, then the
        /// currency buyer will pay that amount in the settlement currency to the currency seller;
        /// if that amount is negative, the seller will make that payment to the buyer.
        Cash = b'C', "C";
    }
}

macros::impl_attr! {
    /// Other underlying assets.
    pub enum OtherUnderlying[2] {
        /// Commercial property (or property derivative).
        ///
        /// A derivative where the underlying is commercial property; property derivatives are
        /// mostly in the form of swaps where one party pays the return on the index if positive
        /// versus the other party paying LIBOR (London interbank offered rate).
        CommercialProperty = b'P', "P";

        /// Others (miscellaneous).
        Other = b'M', "M";
    }
}

macros::impl_attr! {
    /// Other delivery.
    pub enum OtherDelivery[5] {
        /// Cash.
        Cash = b'C', "C";

        /// Physical.
        Physical = b'P', "P";

        /// Elect at settlement (determined at the time of settlement).
        ElectAtSettlement = b'E', "E";
    }
}
