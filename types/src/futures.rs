//! Futures category support.

use crate::{NotApplicable, Standardized, macros};

macros::impl_category! {
    /// Futures.
    ///
    /// Contracts, listed on an exchange or regulated market, which obligate the buyer to receive
    /// and the seller to deliver in the future the assets specified at an agreed price. This
    /// includes forwards on regulated markets.
    pub enum Future {
        /// Financial futures.
        ///
        /// Futures contracts based on underlying assets excluding commodities.
        Financial(Financial) = b'F', "F";

        /// Commodities futures.
        ///
        /// Futures contracts based on bulk goods.
        Commodity(Commodity) = b'C', "C";
    }
}

macros::impl_group! {
    /// Financial futures.
    ///
    /// Futures contracts based on underlying assets excluding commodities.
    pub struct Financial {
        /// Underlying assets (indicates the type of underlying assets that the futures buyer
        /// receives, and that the seller delivers).
        pub underlying: UnderlyingFinancial, 1;

        /// Delivery (indicates whether the settlement of the future is made in cash or whether the
        /// underlying instruments are delivered).
        pub delivery: Delivery, 2;

        /// Standardized/non-standardized (indicates whether the terms of the contract are
        /// standardized or not).
        pub standardized: Standardized, 3;

        /// Not applicable/undefined.
        pub unassigned4: NotApplicable, 4;
    }
}

macros::impl_group! {
    /// Commodities futures.
    ///
    /// Futures contracts based on bulk goods.
    pub struct Commodity {
        /// Underlying assets.
        pub underlying: UnderlyingCommodity, 1;

        /// Delivery (indicates whether the settlement of the future is made in cash or whether the
        /// underlying instruments are delivered).
        pub delivery: Delivery, 2;

        /// Standardized/non-standardized (indicates whether the terms of the contract are
        /// standardized or not).
        pub standardized: Standardized, 3;

        /// Not applicable/undefined.
        pub unassigned4: NotApplicable, 4;
    }
}

macros::impl_attr! {
    /// Underlying assets.
    ///
    /// Indicates the type of underlying assets that the futures buyer receives, and that the
    /// seller delivers.
    pub enum UnderlyingFinancial[2] {
        /// Baskets.
        Baskets = b'B', "B";

        /// Stock-equities.
        Stock = b'S', "S";

        /// Debt instruments.
        Debt = b'D', "D";

        /// Currencies.
        Currency = b'C', "C";

        /// Indices.
        Index = b'I', "I";

        /// Options.
        Option = b'O', "O";

        /// Futures.
        Future = b'F', "F";

        /// Swaps.
        Swaps = b'W', "W";

        /// Interest rates.
        InterestRate = b'N', "N";

        /// Stock dividends.
        StockDividends = b'V', "V";

        /// Others (miscellaneous).
        Other = b'M', "M";
    }
}

macros::impl_attr! {
    /// Delivery.
    ///
    /// Indicates whether the settlement of the future is made in cash or whether the underlying
    /// instruments are delivered.
    pub enum Delivery[3] {
        /// Physical.
        Physical = b'P', "P";

        /// Cash.
        Cash = b'C', "C";

        /// Non-deliverable.
        NonDeliverable = b'N', "N";
    }
}

macros::impl_attr! {
    /// Underlying assets.
    pub enum UnderlyingCommodity[2] {
        /// Extraction resources (metals, precious metals, coal, oil, gas).
        Extraction = b'E', "E";

        /// Agriculture (commodities which include forestry, fishing, livestock, grain, dairy,
        /// corn, cocoa, soybeans, sugar, coffee).
        Agriculture = b'A', "A";

        /// Industrial products (construction, manufacturing).
        Industrial = b'I', "I";

        /// Services (transportation, communication, trade).
        Services = b'S', "S";

        /// Environmental (includes carbon-related, emission reduction, weather).
        Environmental = b'N', "N";

        /// Polypropylene products (includes plastics).
        Polypropylene = b'P', "P";

        /// Generated resources (includes electricity, renewable energy, or any power/energy
        /// delivered through a utility network or provider).
        Generated = b'H', "H";

        /// Others (miscellaneous).
        Other = b'M', "M";
    }
}
