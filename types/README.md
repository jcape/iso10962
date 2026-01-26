# ISO 10962 Types for Classification of Financial Instruments

[![Crates.io][crate-image]][crate-link]<!--
-->[![Docs.rs][docs-image]][docs-link]<!--
-->![MSRV 1.87.0][msrv-image]

This crate provides `no-std`, `no-alloc` compatible data structures for use handling ISO 10962 Classification of Financial Instruments. They are built as a series of nested Rust enumerations:

- [`Code`] is the top-level enum of CFI categories.
  - [`equities::Equity`] is a second-level enum of CFI groups for equities.
    - [`equities::Common`] is the collection of attributes associated with common stock, including:
      - [`equities::VotingRight`] describes the voting rights for stocks (and is an attribute of common shares)
      - [`equities::Ownership`] describes the ownership a given stock represents.
      - [`equities::PaymentStatus`] describes the payment status of a share.
      - [`Form`] describes the form of the share.

[crate-image]: https://img.shields.io/crates/v/iso10962-types.svg?style=for-the-badge
[crate-link]: https://crates.io/crates/iso10962-types/0.3.2
[docs-image]: https://img.shields.io/docsrs/iso10962-types?style=for-the-badge
[docs-link]: https://docs.rs/crate/iso10962-types/0.3.2
[msrv-image]: https://img.shields.io/crates/msrv/iso10962-types/0.3.2?style=for-the-badge
