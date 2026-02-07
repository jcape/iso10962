//! Macros.

/// Generate a new attribute.
macro_rules! impl_attr {
    (
        $(#[$doc:meta])*
        $access:vis enum $name:ident[$($idx:literal),+] {
            $(
                $(#[$vardoc:meta])*
                $variant:ident = $value:literal, $char:literal;
            )*
        }
    ) => {
        pastey::paste! {
            $(#[$doc])*
            #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
            #[repr(u8)]
            $access enum $name {
                $(
                    #[doc = "`" $char "`: "]
                    $(#[$vardoc])*
                    $variant = $value,
                )*

                /// `X`: Not applicable or undefined.
                Undefined = b'X',
            }

            impl $name {
                $(
                    #[doc = " Check if this value is [`" $name "::" $variant "`]."]
                    #[inline]
                    #[must_use]
                    $access const fn [<is_ $variant:snake>](&self) -> bool {
                        matches!(self, Self::$variant)
                    }
                )*

                #[doc = " Check if this value is [`" $name "::Undefined"]
                #[inline]
                #[must_use]
                $access const fn is_undefined(&self) -> bool {
                        matches!(self, Self::Undefined)
                }

                /// Parse the given CFI byte into this attribute.
                ///
                /// # Errors
                ///
                /// - [`Error::InvalidAttribute`] if the byte is not one of the options.
                #[inline]
                $access const fn from_byte(value: u8) -> crate::error::Result<Self> {
                    match value {
                        $(
                            $value => Ok(Self::$variant),
                        )*
                        other => Err(crate::error::Error::InvalidAttribute(0, other as char)),
                    }
                }

                /// Parse the given CFI byte slice into this attribute.
                ///
                /// # Errors
                ///
                /// - [`Error::InvalidLength`](crate::Error::InvalidLength) if the byte slice is
                ///   not [`CFI_LENGTH`](crate::CFI_LENGTH) bytes.
                /// - [`Error::InvalidAttribute`] if the byte is not one of the options.
                #[inline]
                $access const fn from_bytes(value: &[u8], idx: usize) -> crate::error::Result<Self> {
                    if value.len() != crate::CFI_LENGTH {
                        return Err(crate::error::Error::InvalidLength);
                    }

                    match Self::from_byte(value[idx]) {
                        Err(crate::error::Error::InvalidAttribute(_, val)) => {
                            Err(crate::error::Error::InvalidAttribute(idx, val))
                        },
                        other => other,
                    }
                }
            }

            impl crate::Attr for $name {
                #[inline]
                fn from_code_byte(value: u8) -> crate::error::Result<Self> {
                    Self::from_byte(value)
                }
            }

            $(
                impl crate::AttrPos<$idx> for $name {}
            )*
        }
    };
}

/// Generate a new group.
macro_rules! impl_group {
    {
        $(#[$doc:meta])*
        pub struct $name:ident {
            $(
                $(#[$memdoc:meta])*
                pub $member:ident: $value:ty, $offset:literal;
            )*
        }
    } => {
        $(#[$doc])*
        #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        #[repr(C,packed)]
        pub struct $name {
            $(
                $(#[$memdoc])*
                pub $member: $value
            ),*
        }

        impl $name {
            $(
                $(#[$memdoc])*
                #[inline]
                #[must_use]
                pub const fn $member(&self) -> $value {
                    { self.$member }
                }
            )*

            /// Parse the given byte slice into the attributes for this group.
            ///
            /// # Errors
            ///
            /// - [`Error::InvalidLength`](crate::Error::InvalidLength) if byte slice is not
            ///   [`CFI_LENGTH`](crate::CFI_LENGTH) bytes.
            /// - A more specific error if a given attribute/field contained an invalid
            ///   character.
            #[inline]
            pub const fn from_bytes(src: &[u8]) -> crate::error::Result<Self> {
                Ok(Self {
                    $(
                        $member: match <$value>::from_bytes(src, $offset + 2) {
                            Ok(value) => value,
                            Err(error) => return Err(error),
                        },
                    )*
                })
            }
        }

        impl crate::CfiGroup for $name {
            pastey::paste! {
                $(
                    type [<Attr $offset>] = $value;

                    #[inline]
                    fn [<attr $offset>](&self) -> Self::[<Attr $offset>] {
                        self.$member()
                    }
                )*
            }

            #[inline]
            fn from_cfi_bytes(value: &[u8]) -> crate::error::Result<Self> {
                if value.len() != crate::CFI_LENGTH {
                    return Err(crate::error::Error::InvalidLength);
                }

                Ok(Self {
                    $(
                        $member: match <$value as crate::Attr>::from_code_byte(value[$offset + 2]) {
                            Ok(member) => member,
                            Err(error) => return Err(error),
                        },
                    )*
                })
            }
        }
    };
}

/// Generate a new category.
macro_rules! impl_category {
    (
        $(#[$doc:meta])*
        $access:vis enum $name:ident {
            $(
                $(#[$vardoc:meta])*
                $variant:ident($data:ident) = $value:literal, $char:literal;
            )*
        }
    ) => {
        pastey::paste! {
            $(#[$doc])*
            #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
            #[repr(u8)]
            $access enum $name {
                $(
                    #[doc = " `" $char "`: "]
                    $(#[$vardoc])*
                    $variant($data) = $value,
                )*
            }

            impl $name {
                $(
                    #[doc = "Whether the group value is [`Self::" $variant "`]."]
                    #[inline]
                    #[must_use]
                    $access const fn [<is_ $variant:snake>](&self) -> bool {
                        matches!(self, Self::$variant(_))
                    }
                )*

                /// Parse the given byte string into this category data.
                ///
                /// # Errors
                ///
                /// - [`Error::InvalidLength`](crate::Error::InvalidLength) if the byte string is
                ///   not 6 characters long.
                /// - A more specific error if a particular character could not be parsed.
                #[inline]
                $access const fn from_bytes(value: &[u8]) -> crate::error::Result<Self> {
                    if value.len() != crate::CFI_LENGTH {
                        return Err(crate::Error::InvalidLength);
                    }

                    match value[crate::GROUP_IDX] {
                        $(
                            $value => match <$data>::from_bytes(value) {
                                Ok(group) => Ok(Self::$variant(group)),
                                Err(error) => Err(error),
                            },
                        )*

                        other => Err(crate::error::Error::InvalidGroup(other as char))
                    }
                }
            }
        }
    };
}

pub(crate) use impl_attr;
pub(crate) use impl_category;
pub(crate) use impl_group;
