//! Options (listed and unlisted).

use crate::{NotApplicable, macros};

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
        Other(Other) = b'M', "M";
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
    /// Exercise option style.
    pub enum ExerciseStyle[2] InvalidExerciseStyle {
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
    pub enum Underlying[3] InvalidUnderlying {
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
    pub enum Delivery[4] InvalidDelivery {
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
    /// Standardized/non-standardized.
    ///
    /// Indicates whether the terms of the contract are standardized or not.
    pub enum Standardized[5] InvalidStandardized {
        /// Standardized (the underlying instruments, exercise price, expiration date and contract
        /// size of the options are standardized; these options are traded on special option
        /// exchanges).
        Standardized = b'S', "S";

        /// Non-standardized (options traded on option exchanges which have non-standard delivery
        /// or expiry terms).
        NonStandardized = b'N', "N";
    }
}
