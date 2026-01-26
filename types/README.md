# ISO 10962 Types for Classification of Financial Instruments

[![Crates.io][crate-image]][crate-link]<!--
-->[![Docs.rs][docs-image]][docs-link]<!--
-->![MSRV 1.87.0][msrv-image]

This crate provides `no-std`, `no-alloc` compatible data structures for use handling ISO 10962 Classification of Financial Instruments. CFI codes themselves are 6 consecutive ASCII characters in a row. In this crate, they are built as a series of nested Rust enumerations.

For example, the code for a standard equity (e.g. `AAPL`) would use a code `ESVUFR` to identify the characteristics of the stock:

| | Char | Rust Type |
| - | - | - |
| Category | `E` | [`Code::Equity`](crate::Code::Equity) |
| Group | `S` | [`Equity::Common`](crate::equities::Equity::Common) |
| Attr 1 | `V` | [`VotringRight::Voting`](crate::equities::VotingRight) |
| Attr 2 | `U` | [`Ownership::Free`](crate::equities::Ownership) |
| Attr 3 | `F` | [`PaymentStatus::Fully`](crate::equities::PaymentStatus) |
| Attr 4 | `R` | [`Form::Registered`](crate::Form::Registered) |

[crate-image]: https://img.shields.io/crates/v/iso10962-types.svg?style=for-the-badge
[crate-link]: https://crates.io/crates/iso10962-types/0.3.2
[docs-image]: https://img.shields.io/docsrs/iso10962-types?style=for-the-badge
[docs-link]: https://docs.rs/crate/iso10962-types/0.3.2
[msrv-image]: https://img.shields.io/crates/msrv/iso10962-types/0.3.2?style=for-the-badge
