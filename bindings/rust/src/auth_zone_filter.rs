pub use auth_zone_filter::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types,
)]
pub mod auth_zone_filter {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("allowAll"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("allowAll"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Bool,
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                        ::std::vec![
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                                    ::ethers::core::abi::ethabi::ParamType::Bool,
                                                                ],
                                                            ),
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                                ],
                                                            ),
                                                        ],
                                                    ),
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                        ::std::vec![
                                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                                        ],
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                        ::std::vec![
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                        ],
                                                    ),
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ],
                                            ),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IAuthZone.AuthFilter",
                                        ),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("allowNone"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("allowNone"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Bool,
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                        ::std::vec![
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                                    ::ethers::core::abi::ethabi::ParamType::Bool,
                                                                ],
                                                            ),
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                                ],
                                                            ),
                                                        ],
                                                    ),
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                        ::std::vec![
                                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                                        ],
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                        ::std::vec![
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                        ],
                                                    ),
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ],
                                            ),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IAuthZone.AuthFilter",
                                        ),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("isAddressFilterEqual"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "isAddressFilterEqual",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("a"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IAuthZone.AddressFilter",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("b"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IAuthZone.AddressFilter",
                                        ),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("isFilterEqual"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("isFilterEqual"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("a"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Bool,
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                        ::std::vec![
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                                    ::ethers::core::abi::ethabi::ParamType::Bool,
                                                                ],
                                                            ),
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                                ],
                                                            ),
                                                        ],
                                                    ),
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                        ::std::vec![
                                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                                        ],
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                        ::std::vec![
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                        ],
                                                    ),
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ],
                                            ),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IAuthZone.AuthFilter",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("b"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Bool,
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                        ::std::vec![
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                                    ::ethers::core::abi::ethabi::ParamType::Bool,
                                                                ],
                                                            ),
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                                ],
                                                            ),
                                                        ],
                                                    ),
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                        ::std::vec![
                                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                                        ],
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                        ::std::vec![
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                        ],
                                                    ),
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ],
                                            ),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IAuthZone.AuthFilter",
                                        ),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("isItemFilterEqual"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("isItemFilterEqual"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("a"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Bool,
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ],
                                            ),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IAuthZone.ItemFilter",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("b"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Bool,
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ],
                                            ),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IAuthZone.ItemFilter",
                                        ),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("isRangeFilterEqual"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("isRangeFilterEqual"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("a"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IAuthZone.RangeFilter",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("b"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IAuthZone.RangeFilter",
                                        ),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static AUTHZONEFILTER_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"a\x08\xC7a\0:`\x0B\x82\x82\x829\x80Q`\0\x1A`s\x14a\0-WcNH{q`\xE0\x1B`\0R`\0`\x04R`$`\0\xFD[0`\0R`s\x81S\x82\x81\xF3\xFEs\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x000\x14`\x80`@R`\x046\x10a\0lW`\x005`\xE0\x1C\x80cN\xE6C\xA5\x14a\0qW\x80cu{\xB7\x8B\x14a\0\x8FW\x80c\xA24Vu\x14a\0\xB2W\x80c\xA6\xF9\xA1\xC4\x14a\0\xC5W\x80c\xC4\xE6\xF85\x14a\0\xD8W\x80c\xF8I\x0C\x12\x14a\0\xE0W[`\0\x80\xFD[a\0ya\0\xF3V[`@Qa\0\x86\x91\x90a\x04\x14V[`@Q\x80\x91\x03\x90\xF3[a\0\xA2a\0\x9D6`\x04a\x05\xB1V[a\x01\x0FV[`@Q\x90\x15\x15\x81R` \x01a\0\x86V[a\0\xA2a\0\xC06`\x04a\x06GV[a\x013V[a\0\xA2a\0\xD36`\x04a\x06\xA8V[a\x01bV[a\0ya\x01\x8BV[a\0\xA2a\0\xEE6`\x04a\x07\xF0V[a\x01\xFAV[a\0\xFBa\x03\"V[`\0a\x01\x05a\x01\x8BV[`\x01\x81R\x92\x91PPV[\x80Q\x82Q`\0\x91\x14\x80\x15a\x01*WP\x81` \x01Q\x83` \x01Q\x14[\x90P[\x92\x91PPV[\x80Q\x82Q`\0\x91`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x91\x16\x14\x80\x15a\x01*WPP` \x90\x81\x01Q\x91\x01Q\x15\x15\x90\x15\x15\x14\x90V[\x81Q\x81Q`\0\x91a\x01r\x91a\x013V[\x80\x15a\x01*WPa\x01*\x83` \x01Q\x83` \x01Qa\x01\x0FV[a\x01\x93a\x03\"V[P`@\x80Q\x80\x82\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x90R\x83Q\x80\x85\x01\x85R\x82\x81R\x80\x82\x01\x83\x90R\x84Q\x80\x86\x01\x86R\x84\x81R\x80\x83\x01\x82\x90R\x85Q`\xC0\x81\x01\x87R\x93\x84R\x91\x83\x01\x93\x90\x93R``\x93\x82\x01\x84\x90R\x92\x81\x01\x92\x90\x92R`\x80\x82\x01\x81\x90R`\xA0\x82\x01R\x90V[\x80Q\x82Q`\0\x91\x15\x15\x90\x15\x15\x14a\x02\x13WP`\0a\x01-V[a\x02%\x83` \x01Q\x83` \x01Qa\x013V[a\x021WP`\0a\x01-V[\x81`@\x01QQ\x83`@\x01QQ\x14a\x02JWP`\0a\x01-V[`\0[\x83`@\x01QQ\x81\x10\x15a\x02\xBEWa\x02\x9E\x84`@\x01Q\x82\x81Q\x81\x10a\x02sWa\x02sa\x08TV[` \x02` \x01\x01Q\x84`@\x01Q\x83\x81Q\x81\x10a\x02\x91Wa\x02\x91a\x08TV[` \x02` \x01\x01Qa\x01bV[a\x02\xACW`\0\x91PPa\x01-V[\x80a\x02\xB6\x81a\x08jV[\x91PPa\x02MV[Pa\x02\xD1\x83``\x01Q\x83``\x01Qa\x01bV[a\x02\xDDWP`\0a\x01-V[a\x02\xEF\x83`\x80\x01Q\x83`\x80\x01Qa\x01\x0FV[a\x02\xFBWP`\0a\x01-V[a\x03\r\x83`\xA0\x01Q\x83`\xA0\x01Qa\x01\x0FV[a\x03\x19WP`\0a\x01-V[P`\x01\x92\x91PPV[`@Q\x80`\xC0\x01`@R\x80`\0\x15\x15\x81R` \x01a\x03^`@Q\x80`@\x01`@R\x80`\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01`\0\x15\x15\x81RP\x90V[\x81R` \x01``\x81R` \x01a\x03ra\x03\xBBV[\x81R` \x01a\x03\x94`@Q\x80`@\x01`@R\x80`\0\x81R` \x01`\0\x81RP\x90V[\x81R` \x01a\x03\xB6`@Q\x80`@\x01`@R\x80`\0\x81R` \x01`\0\x81RP\x90V[\x90R\x90V[`@\x80Q`\x80\x81\x01\x82R`\0\x91\x81\x01\x82\x81R``\x82\x01\x92\x90\x92R\x90\x81\x90a\x03\x94V[a\x03\xFD\x82\x82Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x82R` \x90\x81\x01Q\x15\x15\x91\x01RV[` \x90\x81\x01Q\x80Q`@\x84\x01R\x01Q``\x90\x91\x01RV[`\0` \x80\x83Ra\x01\xA0\x83\x01\x84Q\x15\x15\x82\x85\x01R\x81\x85\x01Qa\x04N`@\x86\x01\x82\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x82R` \x90\x81\x01Q\x15\x15\x91\x01RV[P`@\x85\x01Q`\x80a\x01\x80\x81\x87\x01R\x82\x82Q\x80\x85Ra\x01\xC0\x88\x01\x91P\x85\x84\x01\x94P`\0\x93P[\x80\x84\x10\x15a\x04\x9BWa\x04\x87\x82\x86Qa\x03\xDDV[\x93\x85\x01\x93`\x01\x93\x90\x93\x01\x92\x90\x82\x01\x90a\x04tV[P``\x88\x01Q\x94Pa\x04\xB0`\xA0\x88\x01\x86a\x03\xDDV[\x87\x82\x01Q\x80Qa\x01 \x89\x01R` \x81\x01Qa\x01@\x89\x01R\x94P`\xA0\x88\x01Q\x80Qa\x01`\x89\x01R` \x81\x01Qa\x01\x80\x89\x01R\x94P\x97\x96PPPPPPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@\x80Q\x90\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x05'Wa\x05'a\x04\xEEV[`@R\x90V[`@Q`\xC0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x05'Wa\x05'a\x04\xEEV[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x05yWa\x05ya\x04\xEEV[`@R\x91\x90PV[`\0`@\x82\x84\x03\x12\x15a\x05\x93W`\0\x80\xFD[a\x05\x9Ba\x05\x04V[\x90P\x815\x81R` \x82\x015` \x82\x01R\x92\x91PPV[`\0\x80`\x80\x83\x85\x03\x12\x15a\x05\xC4W`\0\x80\xFD[a\x05\xCE\x84\x84a\x05\x81V[\x91Pa\x05\xDD\x84`@\x85\x01a\x05\x81V[\x90P\x92P\x92\x90PV[\x805\x80\x15\x15\x81\x14a\x05\xF6W`\0\x80\xFD[\x91\x90PV[`\0`@\x82\x84\x03\x12\x15a\x06\rW`\0\x80\xFD[a\x06\x15a\x05\x04V[\x90P\x815`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x06.W`\0\x80\xFD[\x81Ra\x06<` \x83\x01a\x05\xE6V[` \x82\x01R\x92\x91PPV[`\0\x80`\x80\x83\x85\x03\x12\x15a\x06ZW`\0\x80\xFD[a\x06d\x84\x84a\x05\xFBV[\x91Pa\x05\xDD\x84`@\x85\x01a\x05\xFBV[`\0`\x80\x82\x84\x03\x12\x15a\x06\x85W`\0\x80\xFD[a\x06\x8Da\x05\x04V[\x90Pa\x06\x99\x83\x83a\x05\xFBV[\x81Ra\x06<\x83`@\x84\x01a\x05\x81V[`\0\x80a\x01\0\x83\x85\x03\x12\x15a\x06\xBCW`\0\x80\xFD[a\x06\xC6\x84\x84a\x06sV[\x91Pa\x05\xDD\x84`\x80\x85\x01a\x06sV[`\0a\x01\x80\x82\x84\x03\x12\x15a\x06\xE8W`\0\x80\xFD[a\x06\xF0a\x05-V[\x90Pa\x06\xFB\x82a\x05\xE6V[\x81R` a\x07\x0B\x84\x82\x85\x01a\x05\xFBV[\x81\x83\x01R``\x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x07*W`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x07>W`\0\x80\xFD[\x815\x81\x81\x11\x15a\x07PWa\x07Pa\x04\xEEV[a\x07^\x84\x82`\x05\x1B\x01a\x05PV[\x81\x81R\x84\x81\x01\x92P`\x07\x91\x90\x91\x1B\x83\x01\x84\x01\x90\x87\x82\x11\x15a\x07~W`\0\x80\xFD[\x92\x84\x01\x92[\x81\x84\x10\x15a\x07\xA7Wa\x07\x95\x88\x85a\x06sV[\x83R\x84\x83\x01\x92P`\x80\x84\x01\x93Pa\x07\x83V[\x80`@\x87\x01RPPPPPa\x07\xBF\x83`\x80\x84\x01a\x06sV[``\x82\x01Ra\x07\xD2\x83a\x01\0\x84\x01a\x05\x81V[`\x80\x82\x01Ra\x07\xE5\x83a\x01@\x84\x01a\x05\x81V[`\xA0\x82\x01R\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15a\x08\x03W`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x08\x1BW`\0\x80\xFD[a\x08'\x86\x83\x87\x01a\x06\xD5V[\x93P` \x85\x015\x91P\x80\x82\x11\x15a\x08=W`\0\x80\xFD[Pa\x08J\x85\x82\x86\x01a\x06\xD5V[\x91PP\x92P\x92\x90PV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0`\x01\x82\x01a\x08\x8AWcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[P`\x01\x01\x90V\xFE\xA2dipfsX\"\x12 \xED\t\xC2%\xF5\x8DA\x1E\x93\xD0\x13\x07\x98\xE6\x12JG\xDB\xD6\xAE\xC8\x8Fj\xD5\x05\xFF+\xF9#\0TgdsolcC\0\x08\x11\x003";
    /// The bytecode of the contract.
    pub static AUTHZONEFILTER_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"s\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x000\x14`\x80`@R`\x046\x10a\0lW`\x005`\xE0\x1C\x80cN\xE6C\xA5\x14a\0qW\x80cu{\xB7\x8B\x14a\0\x8FW\x80c\xA24Vu\x14a\0\xB2W\x80c\xA6\xF9\xA1\xC4\x14a\0\xC5W\x80c\xC4\xE6\xF85\x14a\0\xD8W\x80c\xF8I\x0C\x12\x14a\0\xE0W[`\0\x80\xFD[a\0ya\0\xF3V[`@Qa\0\x86\x91\x90a\x04\x14V[`@Q\x80\x91\x03\x90\xF3[a\0\xA2a\0\x9D6`\x04a\x05\xB1V[a\x01\x0FV[`@Q\x90\x15\x15\x81R` \x01a\0\x86V[a\0\xA2a\0\xC06`\x04a\x06GV[a\x013V[a\0\xA2a\0\xD36`\x04a\x06\xA8V[a\x01bV[a\0ya\x01\x8BV[a\0\xA2a\0\xEE6`\x04a\x07\xF0V[a\x01\xFAV[a\0\xFBa\x03\"V[`\0a\x01\x05a\x01\x8BV[`\x01\x81R\x92\x91PPV[\x80Q\x82Q`\0\x91\x14\x80\x15a\x01*WP\x81` \x01Q\x83` \x01Q\x14[\x90P[\x92\x91PPV[\x80Q\x82Q`\0\x91`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x91\x16\x14\x80\x15a\x01*WPP` \x90\x81\x01Q\x91\x01Q\x15\x15\x90\x15\x15\x14\x90V[\x81Q\x81Q`\0\x91a\x01r\x91a\x013V[\x80\x15a\x01*WPa\x01*\x83` \x01Q\x83` \x01Qa\x01\x0FV[a\x01\x93a\x03\"V[P`@\x80Q\x80\x82\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x90R\x83Q\x80\x85\x01\x85R\x82\x81R\x80\x82\x01\x83\x90R\x84Q\x80\x86\x01\x86R\x84\x81R\x80\x83\x01\x82\x90R\x85Q`\xC0\x81\x01\x87R\x93\x84R\x91\x83\x01\x93\x90\x93R``\x93\x82\x01\x84\x90R\x92\x81\x01\x92\x90\x92R`\x80\x82\x01\x81\x90R`\xA0\x82\x01R\x90V[\x80Q\x82Q`\0\x91\x15\x15\x90\x15\x15\x14a\x02\x13WP`\0a\x01-V[a\x02%\x83` \x01Q\x83` \x01Qa\x013V[a\x021WP`\0a\x01-V[\x81`@\x01QQ\x83`@\x01QQ\x14a\x02JWP`\0a\x01-V[`\0[\x83`@\x01QQ\x81\x10\x15a\x02\xBEWa\x02\x9E\x84`@\x01Q\x82\x81Q\x81\x10a\x02sWa\x02sa\x08TV[` \x02` \x01\x01Q\x84`@\x01Q\x83\x81Q\x81\x10a\x02\x91Wa\x02\x91a\x08TV[` \x02` \x01\x01Qa\x01bV[a\x02\xACW`\0\x91PPa\x01-V[\x80a\x02\xB6\x81a\x08jV[\x91PPa\x02MV[Pa\x02\xD1\x83``\x01Q\x83``\x01Qa\x01bV[a\x02\xDDWP`\0a\x01-V[a\x02\xEF\x83`\x80\x01Q\x83`\x80\x01Qa\x01\x0FV[a\x02\xFBWP`\0a\x01-V[a\x03\r\x83`\xA0\x01Q\x83`\xA0\x01Qa\x01\x0FV[a\x03\x19WP`\0a\x01-V[P`\x01\x92\x91PPV[`@Q\x80`\xC0\x01`@R\x80`\0\x15\x15\x81R` \x01a\x03^`@Q\x80`@\x01`@R\x80`\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01`\0\x15\x15\x81RP\x90V[\x81R` \x01``\x81R` \x01a\x03ra\x03\xBBV[\x81R` \x01a\x03\x94`@Q\x80`@\x01`@R\x80`\0\x81R` \x01`\0\x81RP\x90V[\x81R` \x01a\x03\xB6`@Q\x80`@\x01`@R\x80`\0\x81R` \x01`\0\x81RP\x90V[\x90R\x90V[`@\x80Q`\x80\x81\x01\x82R`\0\x91\x81\x01\x82\x81R``\x82\x01\x92\x90\x92R\x90\x81\x90a\x03\x94V[a\x03\xFD\x82\x82Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x82R` \x90\x81\x01Q\x15\x15\x91\x01RV[` \x90\x81\x01Q\x80Q`@\x84\x01R\x01Q``\x90\x91\x01RV[`\0` \x80\x83Ra\x01\xA0\x83\x01\x84Q\x15\x15\x82\x85\x01R\x81\x85\x01Qa\x04N`@\x86\x01\x82\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x82R` \x90\x81\x01Q\x15\x15\x91\x01RV[P`@\x85\x01Q`\x80a\x01\x80\x81\x87\x01R\x82\x82Q\x80\x85Ra\x01\xC0\x88\x01\x91P\x85\x84\x01\x94P`\0\x93P[\x80\x84\x10\x15a\x04\x9BWa\x04\x87\x82\x86Qa\x03\xDDV[\x93\x85\x01\x93`\x01\x93\x90\x93\x01\x92\x90\x82\x01\x90a\x04tV[P``\x88\x01Q\x94Pa\x04\xB0`\xA0\x88\x01\x86a\x03\xDDV[\x87\x82\x01Q\x80Qa\x01 \x89\x01R` \x81\x01Qa\x01@\x89\x01R\x94P`\xA0\x88\x01Q\x80Qa\x01`\x89\x01R` \x81\x01Qa\x01\x80\x89\x01R\x94P\x97\x96PPPPPPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@\x80Q\x90\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x05'Wa\x05'a\x04\xEEV[`@R\x90V[`@Q`\xC0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x05'Wa\x05'a\x04\xEEV[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x05yWa\x05ya\x04\xEEV[`@R\x91\x90PV[`\0`@\x82\x84\x03\x12\x15a\x05\x93W`\0\x80\xFD[a\x05\x9Ba\x05\x04V[\x90P\x815\x81R` \x82\x015` \x82\x01R\x92\x91PPV[`\0\x80`\x80\x83\x85\x03\x12\x15a\x05\xC4W`\0\x80\xFD[a\x05\xCE\x84\x84a\x05\x81V[\x91Pa\x05\xDD\x84`@\x85\x01a\x05\x81V[\x90P\x92P\x92\x90PV[\x805\x80\x15\x15\x81\x14a\x05\xF6W`\0\x80\xFD[\x91\x90PV[`\0`@\x82\x84\x03\x12\x15a\x06\rW`\0\x80\xFD[a\x06\x15a\x05\x04V[\x90P\x815`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x06.W`\0\x80\xFD[\x81Ra\x06<` \x83\x01a\x05\xE6V[` \x82\x01R\x92\x91PPV[`\0\x80`\x80\x83\x85\x03\x12\x15a\x06ZW`\0\x80\xFD[a\x06d\x84\x84a\x05\xFBV[\x91Pa\x05\xDD\x84`@\x85\x01a\x05\xFBV[`\0`\x80\x82\x84\x03\x12\x15a\x06\x85W`\0\x80\xFD[a\x06\x8Da\x05\x04V[\x90Pa\x06\x99\x83\x83a\x05\xFBV[\x81Ra\x06<\x83`@\x84\x01a\x05\x81V[`\0\x80a\x01\0\x83\x85\x03\x12\x15a\x06\xBCW`\0\x80\xFD[a\x06\xC6\x84\x84a\x06sV[\x91Pa\x05\xDD\x84`\x80\x85\x01a\x06sV[`\0a\x01\x80\x82\x84\x03\x12\x15a\x06\xE8W`\0\x80\xFD[a\x06\xF0a\x05-V[\x90Pa\x06\xFB\x82a\x05\xE6V[\x81R` a\x07\x0B\x84\x82\x85\x01a\x05\xFBV[\x81\x83\x01R``\x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x07*W`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x07>W`\0\x80\xFD[\x815\x81\x81\x11\x15a\x07PWa\x07Pa\x04\xEEV[a\x07^\x84\x82`\x05\x1B\x01a\x05PV[\x81\x81R\x84\x81\x01\x92P`\x07\x91\x90\x91\x1B\x83\x01\x84\x01\x90\x87\x82\x11\x15a\x07~W`\0\x80\xFD[\x92\x84\x01\x92[\x81\x84\x10\x15a\x07\xA7Wa\x07\x95\x88\x85a\x06sV[\x83R\x84\x83\x01\x92P`\x80\x84\x01\x93Pa\x07\x83V[\x80`@\x87\x01RPPPPPa\x07\xBF\x83`\x80\x84\x01a\x06sV[``\x82\x01Ra\x07\xD2\x83a\x01\0\x84\x01a\x05\x81V[`\x80\x82\x01Ra\x07\xE5\x83a\x01@\x84\x01a\x05\x81V[`\xA0\x82\x01R\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15a\x08\x03W`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x08\x1BW`\0\x80\xFD[a\x08'\x86\x83\x87\x01a\x06\xD5V[\x93P` \x85\x015\x91P\x80\x82\x11\x15a\x08=W`\0\x80\xFD[Pa\x08J\x85\x82\x86\x01a\x06\xD5V[\x91PP\x92P\x92\x90PV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0`\x01\x82\x01a\x08\x8AWcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[P`\x01\x01\x90V\xFE\xA2dipfsX\"\x12 \xED\t\xC2%\xF5\x8DA\x1E\x93\xD0\x13\x07\x98\xE6\x12JG\xDB\xD6\xAE\xC8\x8Fj\xD5\x05\xFF+\xF9#\0TgdsolcC\0\x08\x11\x003";
    /// The deployed bytecode of the contract.
    pub static AUTHZONEFILTER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct AuthZoneFilter<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for AuthZoneFilter<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for AuthZoneFilter<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for AuthZoneFilter<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for AuthZoneFilter<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(AuthZoneFilter))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> AuthZoneFilter<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    AUTHZONEFILTER_ABI.clone(),
                    client,
                ),
            )
        }
        /// Constructs the general purpose `Deployer` instance based on the provided constructor arguments and sends it.
        /// Returns a new instance of a deployer that returns an instance of this contract after sending the transaction
        ///
        /// Notes:
        /// - If there are no constructor arguments, you should pass `()` as the argument.
        /// - The default poll duration is 7 seconds.
        /// - The default number of confirmations is 1 block.
        ///
        ///
        /// # Example
        ///
        /// Generate contract bindings with `abigen!` and deploy a new contract instance.
        ///
        /// *Note*: this requires a `bytecode` and `abi` object in the `greeter.json` artifact.
        ///
        /// ```ignore
        /// # async fn deploy<M: ethers::providers::Middleware>(client: ::std::sync::Arc<M>) {
        ///     abigen!(Greeter, "../greeter.json");
        ///
        ///    let greeter_contract = Greeter::deploy(client, "Hello world!".to_string()).unwrap().send().await.unwrap();
        ///    let msg = greeter_contract.greet().call().await.unwrap();
        /// # }
        /// ```
        pub fn deploy<T: ::ethers::core::abi::Tokenize>(
            client: ::std::sync::Arc<M>,
            constructor_args: T,
        ) -> ::core::result::Result<
            ::ethers::contract::builders::ContractDeployer<M, Self>,
            ::ethers::contract::ContractError<M>,
        > {
            let factory = ::ethers::contract::ContractFactory::new(
                AUTHZONEFILTER_ABI.clone(),
                AUTHZONEFILTER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `allowAll` (0x4ee643a5) function
        pub fn allow_all(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, AuthFilter> {
            self.0
                .method_hash([78, 230, 67, 165], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `allowNone` (0xc4e6f835) function
        pub fn allow_none(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, AuthFilter> {
            self.0
                .method_hash([196, 230, 248, 53], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isAddressFilterEqual` (0x0eb444e8) function
        pub fn is_address_filter_equal(
            &self,
            a: AddressFilter,
            b: AddressFilter,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([14, 180, 68, 232], (a, b))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isFilterEqual` (0x166220a0) function
        pub fn is_filter_equal(
            &self,
            a: AuthFilter,
            b: AuthFilter,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([22, 98, 32, 160], (a, b))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isItemFilterEqual` (0xee23bb79) function
        pub fn is_item_filter_equal(
            &self,
            a: ItemFilter,
            b: ItemFilter,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([238, 35, 187, 121], (a, b))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isRangeFilterEqual` (0x1d494cd1) function
        pub fn is_range_filter_equal(
            &self,
            a: RangeFilter,
            b: RangeFilter,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([29, 73, 76, 209], (a, b))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for AuthZoneFilter<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `allowAll` function with signature `allowAll()` and selector `0x4ee643a5`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "allowAll", abi = "allowAll()")]
    pub struct AllowAllCall;
    ///Container type for all input parameters for the `allowNone` function with signature `allowNone()` and selector `0xc4e6f835`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "allowNone", abi = "allowNone()")]
    pub struct AllowNoneCall;
    ///Container type for all input parameters for the `isAddressFilterEqual` function with signature `isAddressFilterEqual((address,bool),(address,bool))` and selector `0x0eb444e8`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "isAddressFilterEqual",
        abi = "isAddressFilterEqual((address,bool),(address,bool))"
    )]
    pub struct IsAddressFilterEqualCall {
        pub a: AddressFilter,
        pub b: AddressFilter,
    }
    ///Container type for all input parameters for the `isFilterEqual` function with signature `isFilterEqual((bool,(address,bool),((address,bool),(uint256,uint256))[],((address,bool),(uint256,uint256)),(uint256,uint256),(uint256,uint256)),(bool,(address,bool),((address,bool),(uint256,uint256))[],((address,bool),(uint256,uint256)),(uint256,uint256),(uint256,uint256)))` and selector `0x166220a0`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "isFilterEqual",
        abi = "isFilterEqual((bool,(address,bool),((address,bool),(uint256,uint256))[],((address,bool),(uint256,uint256)),(uint256,uint256),(uint256,uint256)),(bool,(address,bool),((address,bool),(uint256,uint256))[],((address,bool),(uint256,uint256)),(uint256,uint256),(uint256,uint256)))"
    )]
    pub struct IsFilterEqualCall {
        pub a: AuthFilter,
        pub b: AuthFilter,
    }
    ///Container type for all input parameters for the `isItemFilterEqual` function with signature `isItemFilterEqual(((address,bool),(uint256,uint256)),((address,bool),(uint256,uint256)))` and selector `0xee23bb79`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "isItemFilterEqual",
        abi = "isItemFilterEqual(((address,bool),(uint256,uint256)),((address,bool),(uint256,uint256)))"
    )]
    pub struct IsItemFilterEqualCall {
        pub a: ItemFilter,
        pub b: ItemFilter,
    }
    ///Container type for all input parameters for the `isRangeFilterEqual` function with signature `isRangeFilterEqual((uint256,uint256),(uint256,uint256))` and selector `0x1d494cd1`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "isRangeFilterEqual",
        abi = "isRangeFilterEqual((uint256,uint256),(uint256,uint256))"
    )]
    pub struct IsRangeFilterEqualCall {
        pub a: RangeFilter,
        pub b: RangeFilter,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum AuthZoneFilterCalls {
        AllowAll(AllowAllCall),
        AllowNone(AllowNoneCall),
        IsAddressFilterEqual(IsAddressFilterEqualCall),
        IsFilterEqual(IsFilterEqualCall),
        IsItemFilterEqual(IsItemFilterEqualCall),
        IsRangeFilterEqual(IsRangeFilterEqualCall),
    }
    impl ::ethers::core::abi::AbiDecode for AuthZoneFilterCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <AllowAllCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AllowAll(decoded));
            }
            if let Ok(decoded) = <AllowNoneCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AllowNone(decoded));
            }
            if let Ok(decoded) = <IsAddressFilterEqualCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IsAddressFilterEqual(decoded));
            }
            if let Ok(decoded) = <IsFilterEqualCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IsFilterEqual(decoded));
            }
            if let Ok(decoded) = <IsItemFilterEqualCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IsItemFilterEqual(decoded));
            }
            if let Ok(decoded) = <IsRangeFilterEqualCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IsRangeFilterEqual(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for AuthZoneFilterCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AllowAll(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AllowNone(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsAddressFilterEqual(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsFilterEqual(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsItemFilterEqual(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsRangeFilterEqual(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for AuthZoneFilterCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AllowAll(element) => ::core::fmt::Display::fmt(element, f),
                Self::AllowNone(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsAddressFilterEqual(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::IsFilterEqual(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsItemFilterEqual(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsRangeFilterEqual(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<AllowAllCall> for AuthZoneFilterCalls {
        fn from(value: AllowAllCall) -> Self {
            Self::AllowAll(value)
        }
    }
    impl ::core::convert::From<AllowNoneCall> for AuthZoneFilterCalls {
        fn from(value: AllowNoneCall) -> Self {
            Self::AllowNone(value)
        }
    }
    impl ::core::convert::From<IsAddressFilterEqualCall> for AuthZoneFilterCalls {
        fn from(value: IsAddressFilterEqualCall) -> Self {
            Self::IsAddressFilterEqual(value)
        }
    }
    impl ::core::convert::From<IsFilterEqualCall> for AuthZoneFilterCalls {
        fn from(value: IsFilterEqualCall) -> Self {
            Self::IsFilterEqual(value)
        }
    }
    impl ::core::convert::From<IsItemFilterEqualCall> for AuthZoneFilterCalls {
        fn from(value: IsItemFilterEqualCall) -> Self {
            Self::IsItemFilterEqual(value)
        }
    }
    impl ::core::convert::From<IsRangeFilterEqualCall> for AuthZoneFilterCalls {
        fn from(value: IsRangeFilterEqualCall) -> Self {
            Self::IsRangeFilterEqual(value)
        }
    }
    ///Container type for all return fields from the `allowAll` function with signature `allowAll()` and selector `0x4ee643a5`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct AllowAllReturn(pub AuthFilter);
    ///Container type for all return fields from the `allowNone` function with signature `allowNone()` and selector `0xc4e6f835`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct AllowNoneReturn(pub AuthFilter);
    ///Container type for all return fields from the `isAddressFilterEqual` function with signature `isAddressFilterEqual((address,bool),(address,bool))` and selector `0x0eb444e8`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct IsAddressFilterEqualReturn(pub bool);
    ///Container type for all return fields from the `isFilterEqual` function with signature `isFilterEqual((bool,(address,bool),((address,bool),(uint256,uint256))[],((address,bool),(uint256,uint256)),(uint256,uint256),(uint256,uint256)),(bool,(address,bool),((address,bool),(uint256,uint256))[],((address,bool),(uint256,uint256)),(uint256,uint256),(uint256,uint256)))` and selector `0x166220a0`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct IsFilterEqualReturn(pub bool);
    ///Container type for all return fields from the `isItemFilterEqual` function with signature `isItemFilterEqual(((address,bool),(uint256,uint256)),((address,bool),(uint256,uint256)))` and selector `0xee23bb79`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct IsItemFilterEqualReturn(pub bool);
    ///Container type for all return fields from the `isRangeFilterEqual` function with signature `isRangeFilterEqual((uint256,uint256),(uint256,uint256))` and selector `0x1d494cd1`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct IsRangeFilterEqualReturn(pub bool);
}
