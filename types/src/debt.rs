//! Debt Instrument category support

use crate::{Form, NotApplicable, macros};

macros::impl_category! {
    /// Financial instruments evidencing monies owed by the issuer to the holder on terms as
    /// specified.
    pub enum Debt {
        /// `B`: Bonds.
        ///
        /// Any interest-bearing or discounted security that normally obliges the issuer to pay the
        /// bondholder a contracted sum of money and to repay the principal amount of the debt.
        Bond(Bond) = b'B',

        /// `C`: Convertible bonds.
        ///
        /// A bond that can be converted into other securities.
        Convertible(Convertible) = b'C',

        /// `W`: Bonds with warrants attached.
        ///
        /// A bond that is issued together with one or more warrant(s) attached as part of the
        /// offer, the warrant(s) granting the holder the right to purchase a designated security,
        /// often the common stock of the issuer of the debt, at a specified price.
        WarrantAttached(WarrantAttached) = b'W',

        /// `T`: Medium-term notes.
        ///
        /// Negotiable debt instruments offered under a program agreement through one or more
        /// dealers upon request of the issuer. The program defines the terms and conditions of the
        /// notes.
        MediumTerm(MediumTerm) = b'T',

        /// `Y`: Money market instruments.
        ///
        /// Financial instruments designated at issuance as such with a short-term life, for
        /// instance treasury bills and commercial paper including municipal money market
        /// instruments.
        MoneyMarket(MoneyMarket) = b'Y',

        /// `S`: Structured products (with capital protection).
        ///
        /// Capital protected structured instruments offer investors exposure to chosen underlying
        /// assets using various approaches and offering a large variety of asymmetric pay-off
        /// profiles. There are one or more reference entities underlying the product. Redemption
        /// is made at least in the amount of the conditional capital protection at maturity,
        /// provided that no credit event by the reference entity has occurred. Conditional capital
        /// protection only applies to the nominal amount and not to the purchase price. The
        /// general functioning of a capital guaranteed structured instrument is as follows: the
        /// notional amount is split into a zero bond, that will deliver the capital guarantee at
        /// maturity, and the difference between the zero bond’s value (= present value of the
        /// guarantee level at maturity) and the notional amount is used for structuring the
        /// performance component with options which deliver the agreed pay-off profile of the
        /// structured instrument.
        ProtectedStructured(ProtectedStructured) = b'S',

        /// `E`: Structured products (without capital protection).
        ///
        /// A structured instrument without capital protection is a short-term note linked to an
        /// underlying stock. The security offers a steady stream of income due to the payment of a
        /// coupon rate. The redemption at the end of the term is determined on the basis of the
        /// performance and final fixing of the underlying asset: a redemption at the nominal value
        /// is guaranteed as long as the underlying asset has not touched its barrier during
        /// relevant barrier monitoring. If the underlying asset has touched its barrier but is
        /// again above the strike price at final fixing, the nominal price is also repaid.
        /// Nevertheless, if the underlying asset has touched its barrier during barrier monitoring
        /// and closes below the strike price at final fixing, the underlying asset is delivered or
        /// cash compensation paid, provided that no credit event by the reference entity has
        /// occurred. Depending on the characteristics of the product, either a coupon or a
        /// discount to the underlying asset can apply. A coupon is paid out regardless of the
        /// performance of the underlying asset, provided that no credit event by the reference
        /// entity has occurred.
        UnprotectedStructured(UnprotectedStructured) = b'E',

        /// `G`: Mortgage-backed securities (MBS).
        ///
        /// Mortgage-backed securities are debt obligations that represent claims to the cash flows
        /// from pools of mortgage loans, most commonly on residential property. Mortgage loans are
        /// purchased from banks, mortgage companies and other originators, and then assembled into
        /// pools by a governmental, quasi-governmental or private entity. The entity then issues
        /// securities that represent claims on the principal and interest payments made by
        /// borrowers on the loans in the pool, a process known as securitization.
        MortgageBacked(MortgageBacked) = b'G',

        /// `A`: Asset-backed securities (ABS).
        ///
        /// Debt instruments backed by receivables other than those arising out of real estate,
        /// loans or mortgages.
        AssetBacked(AssetBacked) = b'A',

        /// `N`: Municipal bonds.
        ///
        /// Bond issued by a state, provincial, city or local government excluding municipal money
        /// market securities, which shall be classified as debt, money market instruments (see
        /// money market instruments).
        Municipal(Municipal) = b'N',

        /// `D`: Depository receipts on debt instruments.
        ///
        /// Depository receipts are securities that facilitate the ownership of instruments traded
        /// in other jurisdictions. Depository receipts are widely used in order to allow the
        /// trading of debt instruments in jurisdictions other than the one where the original debt
        /// instruments were issued.
        Depository(Depository) = b'D',

        /// `M`: Others (miscellaneous).
        ///
        /// Debt instruments that do not fit into any of the above Groups.
        Other(Other) = b'M',
    }
}

macros::impl_group! {
    /// Any interest-bearing or discounted security that normally obliges the issuer to pay the
    /// bondholder a contracted sum of money and to repay the principal amount of the debt.
    pub struct Bond {
        /// Type of interest or cash payment.
        pub kind: InterestInKindOrCash, 1;

        /// Guarantee or ranking.
        ///
        /// Indicates, in the case of the issuer's inability to settle, whether the debt issue is
        /// additionally secured.
        pub guarantee: Guarantee, 2;

        /// Redemption/reimbursement.
        ///
        /// Indicates the retirement provisions made for the debt issue.
        pub redemption: Redemption, 3;

        /// Form (negotiability, transmission).
        pub form: Form, 4;
    }
}

macros::impl_group! {
    /// A bond that can be converted into other securities.
    pub struct Convertible {
        /// Type of interest.
        pub interest: InterestInKind, 1;

        /// Guarantee or ranking.
        ///
        /// Indicates, in the case of the issuer's inability to settle, whether the debt issue is
        /// additionally secured.
        pub guarantee: Guarantee, 2;

        /// Redemption/reimbursement.
        ///
        /// Indicates the retirement provisions made for the debt issue.
        pub redemption: Redemption, 3;

        /// Form (negotiability, transmission).
        pub form: Form, 4;
    }
}

macros::impl_group! {
    /// Bonds with warrants attached.
    ///
    /// A bond that is issued together with one or more warrant(s) attached as part of the offer,
    /// the warrant(s) granting the holder the right to purchase a designated security, often the
    /// common stock of the issuer of the debt, at a specified price.
    pub struct WarrantAttached {
        /// Type of interest.
        pub interest: InterestInKind, 1;

        /// Guarantee or ranking.
        ///
        /// Indicates, in the case of the issuer's inability to settle, whether the debt issue is
        /// additionally secured.
        pub guarantee: Guarantee, 2;

        /// Redemption/reimbursement.
        ///
        /// Indicates the retirement provisions made for the debt issue.
        pub redemption: Redemption, 3;

        /// Form (negotiability, transmission).
        pub form: Form, 4;
    }
}

macros::impl_group! {
    /// Medium-term Notes.
    ///
    /// Negotiable debt instruments offered under a program agreement through one or more dealers
    /// upon request of the issuer. The program defines the terms and conditions of the notes.
    pub struct MediumTerm {
        /// Type of interest.
        pub interest: InterestInKind, 1;

        /// Guarantee or ranking.
        ///
        /// Indicates, in the case of the issuer's inability to settle, whether the debt issue is
        /// additionally secured.
        pub guarantee: Guarantee, 2;

        /// Redemption/reimbursement.
        ///
        /// Indicates the retirement provisions made for the debt issue.
        pub redemption: Redemption, 3;

        /// Form (negotiability, transmission).
        pub form: Form, 4;
    }
}

macros::impl_group! {
    /// Money market instruments.
    ///
    /// Financial instruments designated at issuance as such with a short-term life, for instance
    /// treasury bills and commercial paper including municipal money market instruments.
    pub struct MoneyMarket {
        /// Type of interest.
        pub interest: InterestInKind, 1;

        /// Guarantee or ranking.
        ///
        /// Indicates, in the case of the issuer's inability to settle, whether the debt issue is
        /// additionally secured.
        pub guarantee: Guarantee, 2;

        /// Redemption/reimbursement.
        ///
        /// Indicates the retirement provisions made for the debt issue.
        pub attr3: NotApplicable, 3;

        /// Form (negotiability, transmission).
        pub form: Form, 4;
    }
}

macros::impl_group! {
    /// Structured products (with capital protection).
    ///
    /// Capital protected structured instruments offer investors exposure to chosen underlying
    /// assets using various approaches and offering a large variety of asymmetric pay-off
    /// profiles. There are one or more reference entities underlying the product. Redemption is
    /// made at least in the amount of the conditional capital protection at maturity, provided
    /// that no credit event by the reference entity has occurred. Conditional capital protection
    /// only applies to the nominal amount and not to the purchase price. The general functioning
    /// of a capital guaranteed structured instrument is as follows: the notional amount is split
    /// into a zero bond, that will deliver the capital guarantee at maturity, and the difference
    /// between the zero bond’s value (= present value of the guarantee level at maturity) and the
    /// notional amount is used for structuring the performance component with options which
    /// deliver the agreed pay-off profile of the structured instrument.
    pub struct ProtectedStructured {
        /// Type of structured instrument with capital protection.
        pub kind: ProtectedKind, 1;

        /// Distribution.
        ///
        /// Indicates the cash distribution provided by the structured instrument.
        pub distribution: Distribution, 2;

        /// Repayment.
        ///
        /// Indicates the repayment form provided by the structured instrument.
        pub repayment: ProtectedRepayment, 3;

        /// Underlying assets.
        ///
        /// Indicates the type of underlying assets in which the structured instrument
        /// participates.
        pub underlying: Underlying, 4;
    }
}

macros::impl_group! {
    /// Structured instruments (without capital protection).
    ///
    /// A structured instrument without capital protection is a short-term note linked to an
    /// underlying stock. The security offers a steady stream of income due to the payment of a
    /// coupon rate. The redemption at the end of the term is determined on the basis of the
    /// performance and final fixing of the underlying asset: a redemption at the nominal value
    /// is guaranteed as long as the underlying asset has not touched its barrier during
    /// relevant barrier monitoring. If the underlying asset has touched its barrier but is
    /// again above the strike price at final fixing, the nominal price is also repaid.
    /// Nevertheless, if the underlying asset has touched its barrier during barrier monitoring
    /// and closes below the strike price at final fixing, the underlying asset is delivered or
    /// cash compensation paid, provided that no credit event by the reference entity has
    /// occurred. Depending on the characteristics of the product, either a coupon or a
    /// discount to the underlying asset can apply. A coupon is paid out regardless of the
    /// performance of the underlying asset, provided that no credit event by the reference
    /// entity has occurred.
    pub struct UnprotectedStructured {
        /// Type of structured instrument without capital protection.
        pub kind: UnprotectedKind, 1;

        /// Distribution.
        ///
        /// Indicates the cash distribution provided by the structured instrument.
        pub distribution: Distribution, 2;

        /// Repayment.
        ///
        /// Indicates the repayment form provided by the structured instrument.
        pub repayment: UnprotectedRepayment, 3;

        /// Underlying assets.
        ///
        /// Indicates the type of underlying assets in which the structured instrument
        /// participates.
        pub underlying: Underlying, 4;
    }
}

macros::impl_group! {
    /// Mortgage-backed securities (MBS).
    ///
    /// Mortgage-backed securities are debt obligations that represent claims to the cash flows
    /// from pools of mortgage loans, most commonly on residential property. Mortgage loans are
    /// purchased from banks, mortgage companies and other originators, and then assembled into
    /// pools by a governmental, quasi-governmental or private entity. The entity then issues
    /// securities that represent claims on the principal and interest payments made by
    /// borrowers on the loans in the pool, a process known as securitization.
    pub struct MortgageBacked {
        /// Type of interest.
        pub interest: Interest, 1;

        /// Guarantee or ranking.
        pub guarantee: Guarantee, 2;

        /// Redemption/reimbursement.
        pub redemption: Redemption, 3;

        /// Form (negotiability, transmission).
        pub form: Form, 4;
    }
}

macros::impl_group! {
    /// Asset-backed securities (ABS).
    ///
    /// Debt instruments backed by receivables other than those arising out of real estate,
    /// loans or mortgages.
    pub struct AssetBacked {
        /// Type of interest.
        pub interest: Interest, 1;

        /// Guarantee or ranking.
        pub guarantee: Guarantee, 2;

        /// Redemption/reimbursement.
        pub redemption: Redemption, 3;

        /// Form (negotiability, transmission).
        pub form: Form, 4;
    }
}

macros::impl_group! {
    /// Municipal bonds.
    ///
    /// Bond issued by a state, provincial, city or local government excluding municipal money
    /// market securities, which shall be classified as debt, money market instruments (see
    /// money market instruments).
    pub struct Municipal {
        /// Type of interest.
        pub interest: Interest, 1;

        /// Guarantee or ranking.
        pub guarantee: Guarantee, 2;

        /// Redemption/reimbursement.
        pub redemption: Redemption, 3;

        /// Form (negotiability, transmission).
        pub form: Form, 4;
    }
}

macros::impl_group! {
    /// Depository receipts on debt instruments.
    ///
    /// Depository receipts are securities that facilitate the ownership of instruments traded
    /// in other jurisdictions. Depository receipts are widely used in order to allow the
    /// trading of debt instruments in jurisdictions other than the one where the original debt
    /// instruments were issued.
    pub struct Depository {
        /// Instrument dependency.
        pub dependency: Dependency, 1;

        /// Type of interest/cash payment.
        pub interest: InterestOrCash, 2;

        /// Guarantee or ranking.
        pub guarantee: Guarantee, 3;

        /// Redemption/reimbursement.
        ///
        /// Indicates the retirement provisions made for the debt issue.
        pub redemption: Redemption, 4;
    }
}

macros::impl_group! {
    /// Others (miscellaneous).
    ///
    /// Debt instruments that do not fit into any of the above Groups.
    pub struct Other {
        /// The type of debt instrument.
        pub kind: OtherKind, 1;

        /// Not applicable/undefined.
        pub attr2: NotApplicable, 2;

        /// Not applicable/undefined.
        pub attr3: NotApplicable, 3;

        /// Form (negotiability, transmission).
        pub form: Form, 4;
    }
}

macros::impl_attr! {
    /// Type of interest or cash payment.
    pub enum InterestInKindOrCash[2] InvalidInterestInKindOrCash {
        /// Fixed rate.
        ///
        /// All interest payments are known at issuance and remain constant for the life of the issue.
        FixedRate = b'F', "F";

        /// Zero/discounted rate.
        ZeroRate = b'Z', "Z";

        /// Variable rate.
        Variable = b'V', "V";

        /// Cash payment.
        CashPayment = b'C', "C";

        /// Payment in-kind.
        PaymentInKind = b'K', "K";
    }
}

macros::impl_attr! {
    /// Guarantee or ranking (indicates, in the case of the issuer's inability to settle, whether
    /// the debt issue is additionally secured).
    ///
    /// # Guidelines:
    ///
    /// The values `N` ([`Senior`](Guarantee::Senior)), `O`
    /// ([`SeniorSubordinated`](Guarantee::SeniorSubordinated)), `Q`
    /// ([`Junior`](Guarantee::Junior)) and `J`
    /// ([`JuniorSubordinated`](Guarantee::JuniorSubordinated)) may only be used for
    /// unsecured securities. `P` ([`NegativePledge`](Guarantee::NegativePledge)) may only
    /// be used for unsecured securities that are neither senior nor junior. `U`
    /// ([`Unsecured`](Guarantee::Unsecured)) may be used only if one of these codes does not apply
    /// to the relevant security.
    pub enum Guarantee[3, 4] InvalidGuarantee {
        /// Government guarantee.
        ///
        /// The debt instrument is guaranteed by a federal, state, (semi)-government, sovereigns,
        /// agencies.
        Government = b'T', "T";

        /// Joint guarantee.
        ///
        /// The debt instrument is guaranteed by an entity (e.g. corporation) other than the
        /// issuer; not a federal or state government.
        Joint = b'G', "G";

        /// Secured.
        ///
        /// Debt issue against which specific assets are pledged to secure the obligation, e.g.
        /// mortgage or receivables.
        Secured = b'S', "S";

        /// Unsecured/unguaranteed.
        ///
        /// The direct obligations of the issuer rest solely on its general credit.
        Unsecured = b'U', "U";

        /// Negative pledge.
        ///
        /// The borrower agrees not to pledge any assets if such pledging would result in less
        /// security for the agreement's bondholders.
        NegativePledge = b'P', "P";

        /// Senior.
        ///
        /// Applies to senior debts that are placed before senior subordinated, junior and junior
        /// subordinated in the ranking in the event of liquidation.
        Senior = b'N', "N";

        /// Senior subordinated.
        ///
        /// Applies to senior subordinated debts that are placed before junior and junior
        /// subordinated in the ranking in the event of liquidation.
        SeniorSubordinated = b'O', "O";

        /// Junior.
        ///
        /// Applies to junior debts that are placed before junior subordinated in the ranking in
        /// the event of liquidation.
        Junior = b'Q', "Q";

        /// Junior subordinated.
        ///
        /// Applies to junior subordinated debts in the ranking in the event of liquidation.
        JuniorSubordinated = b'J', "J";

        /// Supranational.
        ///
        /// Organization defined as being beyond the scope or borders of any one nation such as two
        /// or more central banks or two or more central governments. Examples of supranational
        /// include the United Nations, the European Union, the European Investment Bank and the
        /// World Bank.
        Supranational = b'C', "C";
    }
}

macros::impl_attr! {
    /// Redemption/reimbursement.
    ///
    /// Indicates the retirement provisions made for the debt issue.
    pub enum Redemption[4, 5] InvalidRedemption {
        /// Fixed maturity.
        ///
        /// The principal amount is repaid in full at maturity.
        FixedMaturity = b'F', "F";

        /// Fixed maturity with call feature.
        ///
        /// The issue may be called for redemption prior to the fixed maturity date.
        FixedWithCall = b'G', "G";

        /// Fixed maturity with put feature.
        ///
        /// The holder may request the reimbursement of his or her bonds prior to the maturity
        /// date.
        FixedWithPut = b'C', "C";

        /// Fixed maturity with both put and call features.
        FixedWithPutAndCall = b'D', "D";

        /// Amortization plan.
        ///
        /// Reduction of principal by regular payments.
        Amortization = b'A', "A";

        /// Amortization plan with call feature.
        ///
        /// The redemption of principal may occur as the result of the outstanding portion of the
        /// bond being called.
        AmortizationWithCall = b'B', "B";

        /// Amortization plan with put feature.
        AmortizationWithPut = b'T', "T";

        /// Amortization plan with put and call feature.
        AmortizationWithPutAndCall = b'L', "L";

        /// Perpetual.
        ///
        /// The debt instrument has no fixed maturity date and is only due for redemption in the
        /// case of the issuer's liquidation.
        Perpetual = b'P', "P";

        /// Perpetual with call feature.
        ///
        /// The issue may be called for redemption at some time in the future.
        PerpetualWithCall = b'Q', "Q";

        /// Perpetual with put feature.
        ///
        /// The issue may be puttable for redemption at some time in the future.
        PerpeetualWithPut = b'R', "R";

        /// Extendible.
        Extendible = b'E', "E";
    }
}

macros::impl_attr! {
    /// Type of interest.
    pub enum InterestInKind[2] InvalidInterestInKind {
        /// Fixed rate.
        Fixed = b'F', "F";
        /// Zero rate/discounted.
        Zero = b'Z', "Z";
        /// Variable.
        Variable = b'V', "V";
        /// Payment in kind.
        InKind = b'K', "K";
    }
}

macros::impl_attr! {
    /// Type of structured instrument with capital protection.
    pub enum ProtectedKind[2] InvalidProtectedKind {
        /// Capital protection certificate with participation.
        ///
        /// Minimum redemption at expiry equivalent to the capital protection; capital protection
        /// is defined as a percentage of the nominal amount (e.g. 100 %); capital protection
        /// refers to the nominal amount only, and not to the purchase price; the value of the
        /// product may fall below its capital protection value during its lifetime; participation
        /// is in the underlying price increase above the strike.
        Participation = b'A', "A";

        /// Capital protection convertible certificate.
        ///
        /// Minimum redemption at expiry equivalent to the capital protection; capital protection
        /// is defined as a percentage of the nominal price (e.g. 100%); capital protection refers
        /// to the nominal price only, and not to the purchase price; the value of the product may
        /// fall below its capital protection value during its lifetime; participation is in the
        /// underlying price increase above the conversion price; coupon payment is possible.
        Convertible = b'B', "B";

        /// Barrier capital protection certificate.
        ///
        /// Minimum redemption at expiry equivalent to the capital protection; capital protection
        /// is defined as a percentage of the nominal price (e.g. 100%); capital protection refers
        /// to the nominal amount only, and not to the purchase price; the value of the product may
        /// fall below its capital protection value during its lifetime; participation is in the
        /// underlying price increase above the strike up to the barrier; possibility of rebate
        /// payment once barrier is breached; limited profit potential.
        Barrier = b'C', "C";

        /// Capital protection certificate with coupons.
        ///
        /// Minimum redemption at expiry equivalent to the capital protection; capital protection
        /// is defined as a percentage of the nominal price (e.g. 100%); capital protection refers
        /// to the nominal amount only, and not to the purchase price; the value of the product may
        /// fall below its capital protection value during its lifetime; the coupon amount is
        /// dependent on the development of the underlying asset; periodic coupon payment is
        /// expected.
        Coupons = b'D', "D";

        /// Others (miscellaneous).
        Other = b'M', "M";
    }
}

macros::impl_attr! {
    /// Distribution.
    ///
    /// Indicates the cash distribution provided by the structured instrument.
    pub enum Distribution[3] InvalidDistribution {
        /// Fixed interest payments.
        Fixed = b'F', 1;

        /// Dividend payments.
        Dividend = b'D', 2;

        /// Variable interest payments.
        Variable = b'V', 3;

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
    pub enum ProtectedRepayment[4] InvalidRepayment {
        /// Fixed cash repayment.
        ///
        /// Only protected capital level.
        Fixed = b'F', "F";

        /// Variable cash repayment.
        ///
        /// Protected capital level and additional performance capital depending on the underlying.
        Variable = b'V', "V";

        /// Others (miscellaneous).
        Other = b'M', "M";
    }
}

macros::impl_attr! {
    /// Underlying assets.
    ///
    /// Indicates the type of underlying assets in which the structured instrument participates.
    pub enum Underlying[5] InvalidAsset {
        /// Baskets.
        Basket = b'B', "B";

        /// Equities.
        Equity = b'S', "S";

        /// Debt instruments.
        Debt = b'D', "D";

        /// Commodities.
        Commodity = b'T', "T";

        /// Currencies (specified exchange rate).
        Currency = b'C', "C";

        /// Indices (the performance of an index).
        Index = b'I', "I";

        /// Interest rates (specified amount based on the future level of interest rates).
        InterestRate = b'N', "N";

        /// Other (miscellaneous).
        Other = b'M', "M";

    }
}

macros::impl_attr! {
    /// Type of structured instrument without protection.
    pub enum UnprotectedKind[2] InvalidUnprotectedKind {
        /// Discount certificate.
        ///
        /// Should the underlying asset close below the strike on expiry, the underlying asset(s)
        /// and/or a cash amount is redeemed; discount certificates enable investors to acquire the
        /// underlying asset at a lower price; it corresponds to a buy-write-strategy; it has
        /// reduced risk compared to a direct investment into the underlying asset; with higher
        /// risk levels multiple underlying assets (worst-of) allow for higher discounts; limited
        /// profit opportunity (Cap).
        Discount = b'A', "A";

        /// Barrier discount certificate.
        ///
        /// The maximum redemption amount (Cap) is paid out if the barrier is never breached;
        /// barrier discount certificates enable investors to acquire the underlying asset(s) at a
        /// lower price; due to the barrier, the probability of maximum redemption is higher; the
        /// discount, however, is smaller than for a discount certificate; if the barrier is
        /// breached the product changes into a discount certificate; it has reduced risk compared
        /// to a direct investment into the underlying asset; limited profit potential (Cap); with
        /// higher risk levels multiple underlying assets (worst-of) allow for higher discounts or
        /// a lower barrier.
        BarrierDiscount = b'B', "B";

        /// Reverse convertible.
        ///
        /// Should the underlying asset close below the strike on expiry, the underlying asset(s)
        /// and/or a cash amount is redeemed; should the underlying asset close above the strike at
        /// expiry, the nominal amount plus the coupon is paid at redemption; the coupon is paid
        /// regardless of the underlying development; it has reduced risk compared to a direct
        /// investment into the underlying asset; with higher risk levels, multiple underlying
        /// assets (worst-of) allow for higher coupons; limited profit potential (Cap).
        Reverse = b'C', "C";

        /// Barrier reverse convertible.
        ///
        /// Should the barrier never be breached, the nominal price plus coupon is paid at
        /// redemption; due to the barrier, the probability of maximum redemption is higher; the
        /// coupon, however, is smaller than for a reverse convertible; if the barrier is breached
        /// the product changes into a reverse convertible; the coupon is paid regardless of the
        /// underlying development; it has reduced risk compared to a direct investment into the
        /// underlying asset(s); with higher risk levels, multiple underlying assets (worst-of)
        /// allow for higher coupons or lower barriers; limited profit potential (Cap).
        BarrierReverse = b'D', "D";

        /// Express certificate.
        ///
        /// Should the underlying trade above the strike on the observation date, an early
        /// redemption consisting of nominal price plus an additional coupon amount is paid; it
        /// offers the possibility of an early redemption combined with an attractive yield
        /// opportunity; it has reduced risk compared to a direct investment into the underlying
        /// asset(s); with higher risk levels, multiple underlying assets (worst-of) allow for
        /// higher coupons or lower barriers; limited profit opportunity (Cap).
        Express = b'E', "E";

        /// Others (miscellaneous).
        Other = b'M', "M";
    }
}

macros::impl_attr! {
    /// Repayment.
    ///
    /// Indicates the repayment form provided by the structured instrument.
    pub enum UnprotectedRepayment[4] InvalidRepayment {
        /// Repayment in cash (depending on the underlying, if the barrier is not breached).
        Cash = b'R', "R";

        /// Repayment in assets.
        Assets = b'S', "S";

        /// Repayment in assets and cash.
        AssetsAndCash = b'C', "C";

        /// Repayment in assets or cash.
        AssetsOrCash = b'T', "T";

        /// Others (miscellaneous).
        Other = b'M', "M";
    }
}

macros::impl_attr! {
    /// Type of interest.
    pub enum Interest[2] InvalidInterest {
        /// Fixed rate.
        Fixed = b'F', "F";

        /// Zero rate/discounted.
        Zero = b'Z', "Z";

        /// Variable rate.
        Variable = b'V', "V";
    }
}

macros::impl_attr! {
    /// Instrument dependency.
    pub enum Dependency[2] InvalidDependency {
        /// Bonds.
        Bonds = b'B', "B";

        /// Convertible bonds.
        Convertible = b'C', "C";

        /// Bonds with warrants attached.
        WarrantsAttached = b'W', "W";

        /// Medium-term notes.
        MediumTerm = b'T', "T";

        /// Money market instruments.
        MoneyMarket = b'Y', "Y";

        /// Mortgage-backed securities.
        MortgageBacked = b'G', "G";

        /// Asset-backed securities.
        AssetBacked = b'A', "A";

        /// Municipal bonds.
        Municipal = b'N', "N";

        /// Others (miscellaneous).
        Other = b'M', "M";
    }
}

macros::impl_attr! {
    /// Type of interest/cache payment.
    pub enum InterestOrCash[3] InvalidInterestOrCash {
        /// Fixed rate.
        Fixed = b'F', "F";

        /// Zero rate/discounted.
        Zero = b'Z', "Z";

        /// Variable rate.
        Variable = b'V', "V";

        /// Cash payment.
        Cash = b'C', "C";

    }
}

macros::impl_attr! {
    /// Debt instruments that do not fit into any of the Groups of debt instruments.
    pub enum OtherKind[2] InvalidOtherKind {
        /// Bank loan.
        ///
        /// An amount of money loaned at interest by a bank to a borrower, usually on collateral
        /// security, for a certain period of time.
        BankLoan = b'B', "B";

        /// Promissory note.
        ///
        /// Written promise by one party to pay another party a definite sum of money either on
        /// demand or at a specified future date.
        PromissoryNote = b'P', "P";

        /// Others (miscellaneous).
        Other = b'M', "M";
    }
}
