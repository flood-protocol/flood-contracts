pub use zone::*;
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
pub mod zone {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("admin"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("DEFAULT_ADMIN_ROLE"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("DEFAULT_ADMIN_ROLE"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("FULFILLER_ROLE"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("FULFILLER_ROLE"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("acceptDefaultAdminTransfer"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "acceptDefaultAdminTransfer",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("authorizationFilter"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "authorizationFilter",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("actor"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("beginDefaultAdminTransfer"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "beginDefaultAdminTransfer",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newAdmin"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("cancelDefaultAdminTransfer"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "cancelDefaultAdminTransfer",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("changeDefaultAdminDelay"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "changeDefaultAdminDelay",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newDelay"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(48usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint48"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("defaultAdmin"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("defaultAdmin"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("defaultAdminDelay"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("defaultAdminDelay"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(48usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint48"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("defaultAdminDelayIncreaseWait"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "defaultAdminDelayIncreaseWait",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(48usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint48"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("fee"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("fee"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                        ::std::vec![
                                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                        ],
                                                    ),
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                        ::std::vec![
                                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                                        ],
                                                    ),
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                        ::std::vec![
                                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                                        ],
                                                    ),
                                                ),
                                            ),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct IFloodPlain.Order"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct IZone.FeeInfo"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getRoleAdmin"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getRoleAdmin"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("role"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("grantRole"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("grantRole"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("role"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("hasRole"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("hasRole"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("role"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("owner"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("owner"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("pause"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("pause"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("paused"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("paused"),
                            inputs: ::std::vec![],
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("pendingDefaultAdmin"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "pendingDefaultAdmin",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newAdmin"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("schedule"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(48usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint48"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("pendingDefaultAdminDelay"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "pendingDefaultAdminDelay",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newDelay"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(48usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint48"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("schedule"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(48usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint48"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("renounceRole"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("renounceRole"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("role"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("revokeRole"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("revokeRole"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("role"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("rollbackDefaultAdminDelay"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "rollbackDefaultAdminDelay",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setAuthorizationFilter"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "setAuthorizationFilter",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("actor"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("filter"),
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
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setFee"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setFee"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newFee"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct IZone.FeeInfo"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("supportsInterface"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("supportsInterface"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("interfaceId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        4usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes4"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("unpause"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("unpause"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("validate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("validate"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                        ::std::vec![
                                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                        ],
                                                    ),
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                        ::std::vec![
                                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                                        ],
                                                    ),
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                        ::std::vec![
                                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                                        ],
                                                    ),
                                                ),
                                            ),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct IFloodPlain.Order"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("fulfiller"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("DefaultAdminDelayChangeCanceled"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "DefaultAdminDelayChangeCanceled",
                            ),
                            inputs: ::std::vec![],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("DefaultAdminDelayChangeScheduled"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "DefaultAdminDelayChangeScheduled",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newDelay"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(48usize),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("effectSchedule"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(48usize),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("DefaultAdminTransferCanceled"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "DefaultAdminTransferCanceled",
                            ),
                            inputs: ::std::vec![],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("DefaultAdminTransferScheduled"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "DefaultAdminTransferScheduled",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newAdmin"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("acceptSchedule"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(48usize),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("FeeUpdated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("FeeUpdated"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newFee"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                        ],
                                    ),
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("FilterUpdated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("FilterUpdated"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("actor"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("filter"),
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
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("FulfillerUpdated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("FulfillerUpdated"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("fulfiller"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("valid"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Paused"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Paused"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RoleAdminChanged"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("RoleAdminChanged"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("role"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("previousAdminRole"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newAdminRole"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RoleGranted"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("RoleGranted"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("role"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("sender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RoleRevoked"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("RoleRevoked"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("role"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("sender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Unpaused"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Unpaused"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
            ]),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("AccessControlBadConfirmation"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "AccessControlBadConfirmation",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "AccessControlEnforcedDefaultAdminDelay",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "AccessControlEnforcedDefaultAdminDelay",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("schedule"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(48usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint48"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "AccessControlEnforcedDefaultAdminRules",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "AccessControlEnforcedDefaultAdminRules",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("AccessControlInvalidDefaultAdmin"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "AccessControlInvalidDefaultAdmin",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("defaultAdmin"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("AccessControlUnauthorizedAccount"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "AccessControlUnauthorizedAccount",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("neededRole"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("EnforcedPause"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("EnforcedPause"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ExpectedPause"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("ExpectedPause"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SafeCastOverflowedUintDowncast"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "SafeCastOverflowedUintDowncast",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("bits"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static ZONE_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15b\0\0\x11W`\0\x80\xFD[P`@Qb\0\x1B\x8B8\x03\x80b\0\x1B\x8B\x839\x81\x01`@\x81\x90Rb\0\x004\x91b\0\x01\xCCV[a\x0E\x10\x81`\x01`\x01`\xA0\x1B\x03\x81\x16b\0\0gW`@Qca\x16@\x11`\xE1\x1B\x81R`\0`\x04\x82\x01R`$\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x80T`\x01`\x01`\xD0\x1B\x03\x16`\x01`\xD0\x1Be\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x02\x17\x90Ub\0\0\x93`\0\x82b\0\0\xA8V[PP`\x03\x80T`\xFF\x19\x16\x90UPb\0\x01\xFE\x90PV[`\0\x82b\0\x01\tW`\0b\0\0\xC5`\x02T`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x14b\0\0\xEDW`@Qc\x1F\xE1\xE1=`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x84\x16\x17\x90U[b\0\x01\x15\x83\x83b\0\x01\x1EV[\x90P[\x92\x91PPV[`\0\x82\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x84R\x90\x91R\x81 T`\xFF\x16b\0\x01\xC3W`\0\x83\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x86\x16\x84R\x90\x91R\x90 \x80T`\xFF\x19\x16`\x01\x17\x90Ub\0\x01z3\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x82`\x01`\x01`\xA0\x1B\x03\x16\x84\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4P`\x01b\0\x01\x18V[P`\0b\0\x01\x18V[`\0` \x82\x84\x03\x12\x15b\0\x01\xDFW`\0\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x01\xF7W`\0\x80\xFD[\x93\x92PPPV[a\x19}\x80b\0\x02\x0E`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01\xA9W`\x005`\xE0\x1C\x80cs*\xDA\xAE\x11a\0\xF9W\x80c\xA8\xE9\x8D\x85\x11a\0\x97W\x80c\xCE\xFC\x14)\x11a\0qW\x80c\xCE\xFC\x14)\x14a\x04\x0CW\x80c\xCFn\xEF\xB7\x14a\x04\x14W\x80c\xD5Gt\x1F\x14a\x04BW\x80c\xD6\x02\xB9\xFD\x14a\x04UW`\0\x80\xFD[\x80c\xA8\xE9\x8D\x85\x14a\x03\xD1W\x80c\xADs\xD6\x89\x14a\x03\xF1W\x80c\xCC\x84c\xC8\x14a\x04\x04W`\0\x80\xFD[\x80c\x8D\xA5\xCB[\x11a\0\xD3W\x80c\x8D\xA5\xCB[\x14a\x03\x87W\x80c\x91\xD1HT\x14a\x03\x8FW\x80c\xA1\xED\xA5<\x14a\x03\xA2W\x80c\xA2\x17\xFD\xDF\x14a\x03\xC9W`\0\x80\xFD[\x80cs*\xDA\xAE\x14a\x02\xD9W\x80c\x84V\xCBY\x14a\x03ZW\x80c\x84\xEF\x8F\xFC\x14a\x03bW`\0\x80\xFD[\x80c?K\xA8:\x11a\x01fW\x80c\\\x97Z\xBB\x11a\x01@W\x80c\\\x97Z\xBB\x14a\x02\x95W\x80cb\xF4\xC6\xE9\x14a\x02\xA0W\x80ccN\x93\xDA\x14a\x02\xB3W\x80cd\x9A^\xC7\x14a\x02\xC6W`\0\x80\xFD[\x80c?K\xA8:\x14a\x02SW\x80cBq\x96\xB1\x14a\x02[W\x80cO\xD31\xC6\x14a\x02nW`\0\x80\xFD[\x80c\x01\xFF\xC9\xA7\x14a\x01\xAEW\x80c\x02-c\xFB\x14a\x01\xD6W\x80c\n\xA6\"\x0B\x14a\x01\xF2W\x80c$\x8A\x9C\xA3\x14a\x01\xFCW\x80c//\xF1]\x14a\x02-W\x80c6V\x8A\xBE\x14a\x02@W[`\0\x80\xFD[a\x01\xC1a\x01\xBC6`\x04a\x12\x01V[a\x04]V[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[b\x06\x97\x80[`@Qe\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x01\xCDV[a\x01\xFAa\x04\x88V[\0[a\x02\x1Fa\x02\n6`\x04a\x12+V[`\0\x90\x81R` \x81\x90R`@\x90 `\x01\x01T\x90V[`@Q\x90\x81R` \x01a\x01\xCDV[a\x01\xFAa\x02;6`\x04a\x12YV[a\x04\x9EV[a\x01\xFAa\x02N6`\x04a\x12YV[a\x04\xCAV[a\x01\xFAa\x05vV[a\x01\xFAa\x02i6`\x04a\x12\x89V[a\x05\x89V[a\x02\x1F\x7F_\xD8E\x82\xB3\x0B\xAC\xE1\xCB\xB5\xCC\x91\xA7[\x8E\xE4\x8A\x0E\x84\xDA\x1Ed\xC2\xD8\x80\xC8\xC8e\xC8\x13DO\x81V[`\x03T`\xFF\x16a\x01\xC1V[a\x01\xFAa\x02\xAE6`\x04a\x12\xD6V[a\x06\x01V[a\x01\xFAa\x02\xC16`\x04a\x12\xEEV[a\x06\xB8V[a\x01\xFAa\x02\xD46`\x04a\x13\x0BV[a\x06\xCCV[a\x03,a\x02\xE76`\x04a\x133V[PP`@\x80Q\x80\x82\x01\x82R`\0\x80\x82R` \x91\x82\x01R\x81Q\x80\x83\x01\x90\x92R`\x05T`\x01`\x01`\xA0\x1B\x03\x81\x16\x83R`\x01`\xA0\x1B\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x82\x01R\x90V[`@\x80Q\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x92\x83\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x92\x81\x01\x92\x90\x92R\x01a\x01\xCDV[a\x01\xFAa\x06\xE0V[`\x02T`\x01`\x01`\xA0\x1B\x03\x16[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01\xCDV[a\x03oa\x06\xF3V[a\x01\xC1a\x03\x9D6`\x04a\x12YV[a\x07\x0CV[a\x03\xAAa\x075V[`@\x80Qe\xFF\xFF\xFF\xFF\xFF\xFF\x93\x84\x16\x81R\x92\x90\x91\x16` \x83\x01R\x01a\x01\xCDV[a\x02\x1F`\0\x81V[a\x03\xE4a\x03\xDF6`\x04a\x12\xEEV[a\x07\x89V[`@Qa\x01\xCD\x91\x90a\x13\xB7V[a\x01\xC1a\x03\xFF6`\x04a\x133V[a\t)V[a\x01\xDBa\tkV[a\x01\xFAa\t\xCAV[a\x04\x1Ca\n\nV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x83Re\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16` \x83\x01R\x01a\x01\xCDV[a\x01\xFAa\x04P6`\x04a\x12YV[a\n+V[a\x01\xFAa\nSV[`\0`\x01`\x01`\xE0\x1B\x03\x19\x82\x16c\x18\xA4\xC3\xC3`\xE1\x1B\x14\x80a\x04\x82WPa\x04\x82\x82a\nfV[\x92\x91PPV[`\0a\x04\x93\x81a\n\x9BV[a\x04\x9Ba\n\xA5V[PV[\x81a\x04\xBCW`@Qc\x1F\xE1\xE1=`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x04\xC6\x82\x82a\n\xB2V[PPV[\x81\x15\x80\x15a\x04\xE5WP`\x02T`\x01`\x01`\xA0\x1B\x03\x82\x81\x16\x91\x16\x14[\x15a\x05lW`\0\x80a\x04\xF5a\n\nV[\x90\x92P\x90P`\x01`\x01`\xA0\x1B\x03\x82\x16\x15\x15\x80a\x05\x17WPe\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x15[\x80a\x05*WPBe\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x10\x15[\x15a\x05WW`@Qc\x19\xCA^\xBB`\xE0\x1B\x81Re\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16`\x04\x82\x01R`$\x01[`@Q\x80\x91\x03\x90\xFD[PP`\x01\x80Te\xFF\xFF\xFF\xFF\xFF\xFF`\xA0\x1B\x19\x16\x90U[a\x04\xC6\x82\x82a\n\xDDV[`\0a\x05\x81\x81a\n\x9BV[a\x04\x9Ba\x0B\x15V[`\0a\x05\x94\x81a\n\x9BV[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\x04` R`@\x90 \x82\x90a\x05\xB8\x82\x82a\x16QV[\x90PP\x82`\x01`\x01`\xA0\x1B\x03\x16\x7F\xB3\x84\xAF\xC3\xDE+R2\x7F\x80\x9B\x02f\xD5\xBE\xFE\xDB\xC5\xED\x94\xE7~\x82(\x16\x91\xB0\x80\xD9\xFF\x9C\xCE\x83`@Qa\x05\xF4\x91\x90a\x17\xBCV[`@Q\x80\x91\x03\x90\xA2PPPV[`\0a\x06\x0C\x81a\n\x9BV[a\x06\x1C`@\x83\x01` \x84\x01a\x18\xABV[`\x05\x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x90\x92\x16`\x01`\xA0\x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xA0\x1B\x19\x90\x92\x16\x91\x90\x91\x17\x90Ua\x06V` \x83\x01\x83a\x12\xEEV[`\x05\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U`@Qa\x06\x84\x90\x83\x90a\x18\xC6V[`@Q\x90\x81\x90\x03\x81 \x90\x7F_eK\x8A\x8FhV\xDD\xF1\xDE\xB0\xD7\xFBj\xBF\xD50\xE0\x81i\xDF\xC0\xE5\xE0\xBD\xB3\xEC\xD4U\x93\xC7L\x90`\0\x90\xA2PPV[`\0a\x06\xC3\x81a\n\x9BV[a\x04\xC6\x82a\x0BgV[`\0a\x06\xD7\x81a\n\x9BV[a\x04\xC6\x82a\x0B\xDAV[`\0a\x06\xEB\x81a\n\x9BV[a\x04\x9Ba\x0CJV[`\0a\x07\x07`\x02T`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x90P\x90V[`\0\x91\x82R` \x82\x81R`@\x80\x84 `\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16\x84R\x91\x90R\x90 T`\xFF\x16\x90V[`\x02T`\0\x90`\x01`\xD0\x1B\x90\x04e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x80\x15\x15\x80\x15a\x07aWPBe\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x10\x15[a\x07mW`\0\x80a\x07\x81V[`\x02T`\x01`\xA0\x1B\x90\x04e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81[\x91P\x91P\x90\x91V[a\x07\x91a\x11FV[`\x01`\x01`\xA0\x1B\x03\x80\x83\x16`\0\x90\x81R`\x04` \x90\x81R`@\x80\x83 \x81Q`\xC0\x81\x01\x83R\x81T`\xFF\x90\x81\x16\x15\x15\x82R\x83Q\x80\x85\x01\x85R`\x01\x84\x01T\x97\x88\x16\x81R`\x01`\xA0\x1B\x90\x97\x04\x16\x15\x15\x86\x85\x01R\x80\x84\x01\x95\x90\x95R`\x02\x81\x01\x80T\x83Q\x81\x86\x02\x81\x01\x86\x01\x85R\x81\x81R\x92\x95\x93\x87\x01\x94\x92\x93\x91\x92\x90\x91\x90\x84\x01[\x82\x82\x10\x15a\x08\x83W`\0\x84\x81R` \x90\x81\x90 `@\x80Q`\x80\x81\x01\x82R`\x03\x86\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x81\x16\x84\x84\x01\x90\x81R`\x01`\xA0\x1B\x90\x91\x04`\xFF\x16\x15\x15``\x85\x01R\x83R\x81Q\x80\x83\x01\x90\x92R`\x01\x80\x82\x01T\x83R`\x02\x90\x91\x01T\x82\x85\x01R\x82\x84\x01\x91\x90\x91R\x90\x83R\x90\x92\x01\x91\x01a\x08\x0BV[PPP\x90\x82RP`@\x80Q`\x80\x81\x01\x82R`\x03\x84\x01T`\x01`\x01`\xA0\x1B\x03\x81\x16\x82\x84\x01\x90\x81R`\x01`\xA0\x1B\x90\x91\x04`\xFF\x16\x15\x15``\x80\x84\x01\x91\x90\x91R\x90\x82R\x82Q\x80\x84\x01\x84R`\x04\x86\x01T\x81R`\x05\x86\x01T` \x80\x83\x01\x91\x90\x91R\x80\x84\x01\x91\x90\x91R\x80\x85\x01\x92\x90\x92R\x82Q\x80\x84\x01\x84R`\x06\x86\x01T\x81R`\x07\x86\x01T\x81\x84\x01R\x83\x85\x01R\x82Q\x80\x84\x01\x90\x93R`\x08\x85\x01T\x83R`\t\x90\x94\x01T\x90\x82\x01R\x91\x01R\x92\x91PPV[`\0a\tU\x7F_\xD8E\x82\xB3\x0B\xAC\xE1\xCB\xB5\xCC\x91\xA7[\x8E\xE4\x8A\x0E\x84\xDA\x1Ed\xC2\xD8\x80\xC8\xC8e\xC8\x13DO\x83a\x07\x0CV[\x80\x15a\tdWP`\x03T`\xFF\x16\x15[\x93\x92PPPV[`\x02T`\0\x90`\x01`\xD0\x1B\x90\x04e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x80\x15\x15\x80\x15a\t\x96WPBe\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x10[a\t\xB1W`\x01T`\x01`\xD0\x1B\x90\x04e\xFF\xFF\xFF\xFF\xFF\xFF\x16a\t\xC4V[`\x02T`\x01`\xA0\x1B\x90\x04e\xFF\xFF\xFF\xFF\xFF\xFF\x16[\x91PP\x90V[`\0a\t\xD4a\n\nV[P\x90P3`\x01`\x01`\xA0\x1B\x03\x82\x16\x14a\n\x02W`@Qca\x16@\x11`\xE1\x1B\x81R3`\x04\x82\x01R`$\x01a\x05NV[a\x04\x9Ba\x0C\x87V[`\x01T`\x01`\x01`\xA0\x1B\x03\x81\x16\x91`\x01`\xA0\x1B\x90\x91\x04e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[\x81a\nIW`@Qc\x1F\xE1\xE1=`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x04\xC6\x82\x82a\r V[`\0a\n^\x81a\n\x9BV[a\x04\x9Ba\rEV[`\0`\x01`\x01`\xE0\x1B\x03\x19\x82\x16cye\xDB\x0B`\xE0\x1B\x14\x80a\x04\x82WPc\x01\xFF\xC9\xA7`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x83\x16\x14a\x04\x82V[a\x04\x9B\x813a\rPV[a\n\xB0`\0\x80a\r\x89V[V[`\0\x82\x81R` \x81\x90R`@\x90 `\x01\x01Ta\n\xCD\x81a\n\x9BV[a\n\xD7\x83\x83a\x0EIV[PPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x163\x14a\x0B\x06W`@Qc3K\xD9\x19`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0B\x10\x82\x82a\x0E\xB1V[PPPV[a\x0B\x1Da\x0E\xEEV[`\x03\x80T`\xFF\x19\x16\x90U\x7F]\xB9\xEE\nI[\xF2\xE6\xFF\x9C\x91\xA7\x83L\x1B\xA4\xFD\xD2D\xA5\xE8\xAANS{\xD3\x8A\xEA\xE4\xB0s\xAA3[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01`@Q\x80\x91\x03\x90\xA1V[`\0a\x0Bqa\tkV[a\x0BzBa\x0F\x11V[a\x0B\x84\x91\x90a\x19\x02V[\x90Pa\x0B\x90\x82\x82a\x0FHV[`@Qe\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x7F3w\xDCD$\x1Ew\x9D\xD0j\xFA\xB5\xB7\x88\xA3\\\xA5\xF3\xB7x\x83n)\x90\xBD\xB2j*K.^\xD6\x90` \x01`@Q\x80\x91\x03\x90\xA2PPV[`\0a\x0B\xE5\x82a\x0F\xC7V[a\x0B\xEEBa\x0F\x11V[a\x0B\xF8\x91\x90a\x19\x02V[\x90Pa\x0C\x04\x82\x82a\r\x89V[`@\x80Qe\xFF\xFF\xFF\xFF\xFF\xFF\x80\x85\x16\x82R\x83\x16` \x82\x01R\x7F\xF1\x03\x8C\x18\xCF\x84\xA5nC/\xDB\xFA\xF7F\x92K~\xA5\x11\xDF\xE0:e\x06\xA0\xCE\xBAH\x88x\x8D\x9B\x91\x01`@Q\x80\x91\x03\x90\xA1PPV[a\x0CRa\x10\x0FV[`\x03\x80T`\xFF\x19\x16`\x01\x17\x90U\x7Fb\xE7\x8C\xEA\x01\xBE\xE3 \xCDNB\x02p\xB5\xEAt\0\r\x11\xB0\xC9\xF7GT\xEB\xDB\xFCTK\x05\xA2Xa\x0BJ3\x90V[`\0\x80a\x0C\x92a\n\nV[\x91P\x91Pa\x0C\xA7\x81e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x15\x15\x90V[\x15\x80a\x0C\xBBWPBe\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x10\x15[\x15a\x0C\xE3W`@Qc\x19\xCA^\xBB`\xE0\x1B\x81Re\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16`\x04\x82\x01R`$\x01a\x05NV[a\x0C\xFF`\0a\x0C\xFA`\x02T`\x01`\x01`\xA0\x1B\x03\x16\x90V[a\x0E\xB1V[Pa\r\x0B`\0\x83a\x0EIV[PP`\x01\x80T`\x01`\x01`\xD0\x1B\x03\x19\x16\x90UPV[`\0\x82\x81R` \x81\x90R`@\x90 `\x01\x01Ta\r;\x81a\n\x9BV[a\n\xD7\x83\x83a\x0E\xB1V[a\n\xB0`\0\x80a\x0FHV[a\rZ\x82\x82a\x07\x0CV[a\x04\xC6W`@Qc\xE2Q}?`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R`$\x81\x01\x83\x90R`D\x01a\x05NV[`\x02T`\x01`\xD0\x1B\x90\x04e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x80\x15a\x0E\x0CWBe\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x10\x15a\r\xE2W`\x02T`\x01\x80T`\x01`\x01`\xD0\x1B\x03\x16`\x01`\xA0\x1B\x90\x92\x04e\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x01`\xD0\x1B\x02\x91\x90\x91\x17\x90Ua\x0E\x0CV[`@Q\x7F+\x1F\xA2\xED\xAF\xE6\xF7\xB9\xE9|\x1A\x9E\x0C6`\xE6E\xBE\xB2\xDC\xAA-E\xBD\xBF\x9B\xEA\xF5G.\x1E\xC5\x90`\0\x90\xA1[P`\x02\x80T`\x01`\x01`\xA0\x1B\x03\x16`\x01`\xA0\x1Be\xFF\xFF\xFF\xFF\xFF\xFF\x94\x85\x16\x02`\x01`\x01`\xD0\x1B\x03\x16\x17`\x01`\xD0\x1B\x92\x90\x93\x16\x91\x90\x91\x02\x91\x90\x91\x17\x90UV[`\0\x82a\x0E\xA7W`\0a\x0Ed`\x02T`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x0E\x8BW`@Qc\x1F\xE1\xE1=`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x84\x16\x17\x90U[a\td\x83\x83a\x103V[`\0\x82\x15\x80\x15a\x0E\xCEWP`\x02T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16\x91\x16\x14[\x15a\x0E\xE4W`\x02\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x90U[a\td\x83\x83a\x10\xC5V[`\x03T`\xFF\x16a\n\xB0W`@Qc\x8D\xFC +`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0e\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x0FDW`@Qc\x06\xDF\xCCe`\xE4\x1B\x81R`0`\x04\x82\x01R`$\x81\x01\x83\x90R`D\x01a\x05NV[P\x90V[`\0a\x0FRa\n\nV[`\x01\x80Te\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16`\x01`\xA0\x1B\x02`\x01`\x01`\xD0\x1B\x03\x19\x90\x91\x16`\x01`\x01`\xA0\x1B\x03\x88\x16\x17\x17\x90U\x91Pa\x0F\x94\x90P\x81e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x15\x15\x90V[\x15a\x0B\x10W`@Q\x7F\x88\x86\xEB\xFCBY\xAB\xDB\xC1f\x01\xDD\x8F\xB5g\x8ET\x87\x8FG\xB3\xC3H6\xCF\xC5\x11T\xA9`Q\t\x90`\0\x90\xA1PPPV[`\0\x80a\x0F\xD2a\tkV[\x90P\x80e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11a\x0F\xFAWa\x0F\xF5\x83\x82a\x19(V[a\tdV[a\tde\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16b\x06\x97\x80a\x110V[`\x03T`\xFF\x16\x15a\n\xB0W`@Qc\xD9<\x06e`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x10?\x83\x83a\x07\x0CV[a\x10\xBDW`\0\x83\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x86\x16\x84R\x90\x91R\x90 \x80T`\xFF\x19\x16`\x01\x17\x90Ua\x10u3\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x82`\x01`\x01`\xA0\x1B\x03\x16\x84\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4P`\x01a\x04\x82V[P`\0a\x04\x82V[`\0a\x10\xD1\x83\x83a\x07\x0CV[\x15a\x10\xBDW`\0\x83\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x86\x16\x80\x85R\x92R\x80\x83 \x80T`\xFF\x19\x16\x90UQ3\x92\x86\x91\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B\x91\x90\xA4P`\x01a\x04\x82V[`\0\x81\x83\x10a\x11?W\x81a\tdV[P\x90\x91\x90PV[`@Q\x80`\xC0\x01`@R\x80`\0\x15\x15\x81R` \x01a\x11\x82`@Q\x80`@\x01`@R\x80`\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01`\0\x15\x15\x81RP\x90V[\x81R` \x01``\x81R` \x01a\x11\x96a\x11\xDFV[\x81R` \x01a\x11\xB8`@Q\x80`@\x01`@R\x80`\0\x81R` \x01`\0\x81RP\x90V[\x81R` \x01a\x11\xDA`@Q\x80`@\x01`@R\x80`\0\x81R` \x01`\0\x81RP\x90V[\x90R\x90V[`@\x80Q`\x80\x81\x01\x82R`\0\x91\x81\x01\x82\x81R``\x82\x01\x92\x90\x92R\x90\x81\x90a\x11\xB8V[`\0` \x82\x84\x03\x12\x15a\x12\x13W`\0\x80\xFD[\x815`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a\tdW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x12=W`\0\x80\xFD[P5\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x04\x9BW`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a\x12lW`\0\x80\xFD[\x825\x91P` \x83\x015a\x12~\x81a\x12DV[\x80\x91PP\x92P\x92\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x12\x9CW`\0\x80\xFD[\x825a\x12\xA7\x81a\x12DV[\x91P` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x12\xC3W`\0\x80\xFD[\x83\x01a\x01\x80\x81\x86\x03\x12\x15a\x12~W`\0\x80\xFD[`\0`@\x82\x84\x03\x12\x15a\x12\xE8W`\0\x80\xFD[P\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x13\0W`\0\x80\xFD[\x815a\td\x81a\x12DV[`\0` \x82\x84\x03\x12\x15a\x13\x1DW`\0\x80\xFD[\x815e\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\tdW`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a\x13FW`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x13]W`\0\x80\xFD[\x83\x01a\x01@\x81\x86\x03\x12\x15a\x13pW`\0\x80\xFD[\x91P` \x83\x015a\x12~\x81a\x12DV[a\x13\xA0\x82\x82Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x82R` \x90\x81\x01Q\x15\x15\x91\x01RV[` \x90\x81\x01Q\x80Q`@\x84\x01R\x01Q``\x90\x91\x01RV[`\0` \x80\x83Ra\x01\xA0\x83\x01\x84Q\x15\x15` \x85\x01R` \x85\x01Qa\x13\xF3`@\x86\x01\x82\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x82R` \x90\x81\x01Q\x15\x15\x91\x01RV[P`@\x85\x01Q`\x80a\x01\x80`\x80\x87\x01R\x82\x82Q\x80\x85Ra\x01\xC0\x88\x01\x91P` \x84\x01\x94P`\0\x93P[\x80\x84\x10\x15a\x14BWa\x14.\x82\x86Qa\x13\x80V[\x93\x85\x01\x93`\x01\x93\x90\x93\x01\x92\x90\x82\x01\x90a\x14\x1BV[P``\x88\x01Q\x94Pa\x14W`\xA0\x88\x01\x86a\x13\x80V[`\x80\x88\x01Q\x80Qa\x01 \x89\x01R` \x81\x01Qa\x01@\x89\x01R\x94P`\xA0\x88\x01Q\x80Qa\x01`\x89\x01R` \x81\x01Qa\x01\x80\x89\x01R\x94P\x97\x96PPPPPPPV[\x80\x15\x15\x81\x14a\x04\x9BW`\0\x80\xFD[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a\x14\xBBW`\0\x80\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x14\xD6W`\0\x80\xFD[` \x01\x91P`\x07\x81\x90\x1B6\x03\x82\x13\x15a\x14\xEEW`\0\x80\xFD[\x92P\x92\x90PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x815a\x15\x16\x81a\x12DV[\x81T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x82\x17\x83U` \x84\x015a\x15B\x81a\x14\x96V[`\x01`\x01`\xA8\x1B\x03\x19\x91\x90\x91\x16\x90\x91\x17\x90\x15\x15`\xA0\x1B`\xFF`\xA0\x1B\x16\x17\x81U`@\x82\x015`\x01\x82\x01U``\x90\x91\x015`\x02\x90\x91\x01UV[h\x01\0\0\0\0\0\0\0\0\x83\x11\x15a\x15\xA0WcNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[\x80T\x83\x82U\x80\x84\x10\x15a\x16\x14W`\x03\x81`\x03\x02`\x03\x81\x04\x83\x14a\x15\xC5Wa\x15\xC5a\x14\xF5V[\x85`\x03\x02`\x03\x81\x04\x87\x14a\x15\xDBWa\x15\xDBa\x14\xF5V[`\0\x85\x81R` \x81 \x92\x83\x01\x92\x91\x90\x91\x01\x90[\x82\x82\x10\x15a\x16\x0FW\x80\x82U\x80`\x01\x83\x01U\x80`\x02\x83\x01U\x83\x82\x01\x91Pa\x15\xEEV[PPPP[P`\0\x81\x81R` \x81 \x83\x91[\x85\x81\x10\x15a\x16IWa\x163\x83\x83a\x15\x0BV[`\x80\x92\x90\x92\x01\x91`\x03\x91\x90\x91\x01\x90`\x01\x01a\x16!V[PPPPPPV[\x815a\x16\\\x81a\x14\x96V[\x81T\x90\x15\x15`\xFF\x16`\xFF\x19\x91\x90\x91\x16\x17\x81U`\x01\x81\x01` \x83\x015a\x16\x80\x81a\x12DV[\x81T`@\x85\x015a\x16\x90\x81a\x14\x96V[`\xFF`\xA0\x1B\x90\x15\x15`\xA0\x1B\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16`\x01`\x01`\xA8\x1B\x03\x19\x91\x90\x91\x16\x17\x17\x90Ua\x16\xC6``\x83\x01\x83a\x14\xA4V[a\x16\xD4\x81\x83`\x02\x86\x01a\x15yV[PPa\x16\xE6`\x80\x83\x01`\x03\x83\x01a\x15\x0BV[a\x01\0\x82\x015`\x06\x82\x01Ua\x01 \x82\x015`\x07\x82\x01Ua\x01@\x82\x015`\x08\x82\x01Ua\x01`\x82\x015`\t\x82\x01UPPV[\x805a\x17!\x81a\x12DV[`\x01`\x01`\xA0\x1B\x03\x16\x82R` \x81\x015a\x17:\x81a\x14\x96V[\x80\x15\x15` \x84\x01RPPPV[a\x17Q\x82\x82a\x17\x16V[a\x04\xC6`@\x83\x01`@\x83\x01\x805\x82R` \x90\x81\x015\x91\x01RV[\x81\x83R` \x83\x01\x92P`\0\x81`\0[\x84\x81\x10\x15a\x17\xB2Wa\x17\x8C\x86\x83a\x17\x16V[`@\x82\x81\x015\x90\x87\x01R``\x80\x83\x015\x90\x87\x01R`\x80\x95\x86\x01\x95\x90\x91\x01\x90`\x01\x01a\x17zV[P\x93\x94\x93PPPPV[` \x81R`\0\x825a\x17\xCD\x81a\x14\x96V[\x80\x15\x15` \x84\x01RPa\x17\xE6`@\x83\x01` \x85\x01a\x17\x16V[``\x83\x015`\x1E\x19\x846\x03\x01\x81\x12a\x17\xFDW`\0\x80\xFD[\x83\x01` \x81\x01\x905g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x18\x1AW`\0\x80\xFD[\x80`\x07\x1B6\x03\x82\x13\x15a\x18,W`\0\x80\xFD[a\x01\x80`\x80\x85\x01Ra\x18Ca\x01\xA0\x85\x01\x82\x84a\x17kV[\x91PPa\x18V`\xA0\x84\x01`\x80\x86\x01a\x17GV[a\x18ra\x01 \x84\x01a\x01\0\x86\x01\x805\x82R` \x90\x81\x015\x91\x01RV[a\tda\x01`\x84\x01a\x01@\x86\x01\x805\x82R` \x90\x81\x015\x91\x01RV[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x18\xA6W`\0\x80\xFD[\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x18\xBDW`\0\x80\xFD[a\td\x82a\x18\x8EV[`\0\x825a\x18\xD3\x81a\x12DV[`\x01`\x01`\xA0\x1B\x03\x16\x82Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x18\xF3` \x85\x01a\x18\x8EV[\x16` \x83\x01RP`@\x01\x91\x90PV[e\xFF\xFF\xFF\xFF\xFF\xFF\x81\x81\x16\x83\x82\x16\x01\x90\x80\x82\x11\x15a\x19!Wa\x19!a\x14\xF5V[P\x92\x91PPV[e\xFF\xFF\xFF\xFF\xFF\xFF\x82\x81\x16\x82\x82\x16\x03\x90\x80\x82\x11\x15a\x19!Wa\x19!a\x14\xF5V\xFE\xA2dipfsX\"\x12 v\xCA\x95(r\xE9\r\x1C\xE1?\xCD\xFC\xA3>\xCD\xD5\xE03\xC0qp\xE3\xEF\x16L\x91+\0\xC1r59dsolcC\0\x08\x17\x003";
    /// The bytecode of the contract.
    pub static ZONE_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01\xA9W`\x005`\xE0\x1C\x80cs*\xDA\xAE\x11a\0\xF9W\x80c\xA8\xE9\x8D\x85\x11a\0\x97W\x80c\xCE\xFC\x14)\x11a\0qW\x80c\xCE\xFC\x14)\x14a\x04\x0CW\x80c\xCFn\xEF\xB7\x14a\x04\x14W\x80c\xD5Gt\x1F\x14a\x04BW\x80c\xD6\x02\xB9\xFD\x14a\x04UW`\0\x80\xFD[\x80c\xA8\xE9\x8D\x85\x14a\x03\xD1W\x80c\xADs\xD6\x89\x14a\x03\xF1W\x80c\xCC\x84c\xC8\x14a\x04\x04W`\0\x80\xFD[\x80c\x8D\xA5\xCB[\x11a\0\xD3W\x80c\x8D\xA5\xCB[\x14a\x03\x87W\x80c\x91\xD1HT\x14a\x03\x8FW\x80c\xA1\xED\xA5<\x14a\x03\xA2W\x80c\xA2\x17\xFD\xDF\x14a\x03\xC9W`\0\x80\xFD[\x80cs*\xDA\xAE\x14a\x02\xD9W\x80c\x84V\xCBY\x14a\x03ZW\x80c\x84\xEF\x8F\xFC\x14a\x03bW`\0\x80\xFD[\x80c?K\xA8:\x11a\x01fW\x80c\\\x97Z\xBB\x11a\x01@W\x80c\\\x97Z\xBB\x14a\x02\x95W\x80cb\xF4\xC6\xE9\x14a\x02\xA0W\x80ccN\x93\xDA\x14a\x02\xB3W\x80cd\x9A^\xC7\x14a\x02\xC6W`\0\x80\xFD[\x80c?K\xA8:\x14a\x02SW\x80cBq\x96\xB1\x14a\x02[W\x80cO\xD31\xC6\x14a\x02nW`\0\x80\xFD[\x80c\x01\xFF\xC9\xA7\x14a\x01\xAEW\x80c\x02-c\xFB\x14a\x01\xD6W\x80c\n\xA6\"\x0B\x14a\x01\xF2W\x80c$\x8A\x9C\xA3\x14a\x01\xFCW\x80c//\xF1]\x14a\x02-W\x80c6V\x8A\xBE\x14a\x02@W[`\0\x80\xFD[a\x01\xC1a\x01\xBC6`\x04a\x12\x01V[a\x04]V[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[b\x06\x97\x80[`@Qe\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x01\xCDV[a\x01\xFAa\x04\x88V[\0[a\x02\x1Fa\x02\n6`\x04a\x12+V[`\0\x90\x81R` \x81\x90R`@\x90 `\x01\x01T\x90V[`@Q\x90\x81R` \x01a\x01\xCDV[a\x01\xFAa\x02;6`\x04a\x12YV[a\x04\x9EV[a\x01\xFAa\x02N6`\x04a\x12YV[a\x04\xCAV[a\x01\xFAa\x05vV[a\x01\xFAa\x02i6`\x04a\x12\x89V[a\x05\x89V[a\x02\x1F\x7F_\xD8E\x82\xB3\x0B\xAC\xE1\xCB\xB5\xCC\x91\xA7[\x8E\xE4\x8A\x0E\x84\xDA\x1Ed\xC2\xD8\x80\xC8\xC8e\xC8\x13DO\x81V[`\x03T`\xFF\x16a\x01\xC1V[a\x01\xFAa\x02\xAE6`\x04a\x12\xD6V[a\x06\x01V[a\x01\xFAa\x02\xC16`\x04a\x12\xEEV[a\x06\xB8V[a\x01\xFAa\x02\xD46`\x04a\x13\x0BV[a\x06\xCCV[a\x03,a\x02\xE76`\x04a\x133V[PP`@\x80Q\x80\x82\x01\x82R`\0\x80\x82R` \x91\x82\x01R\x81Q\x80\x83\x01\x90\x92R`\x05T`\x01`\x01`\xA0\x1B\x03\x81\x16\x83R`\x01`\xA0\x1B\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x82\x01R\x90V[`@\x80Q\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x92\x83\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x92\x81\x01\x92\x90\x92R\x01a\x01\xCDV[a\x01\xFAa\x06\xE0V[`\x02T`\x01`\x01`\xA0\x1B\x03\x16[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01\xCDV[a\x03oa\x06\xF3V[a\x01\xC1a\x03\x9D6`\x04a\x12YV[a\x07\x0CV[a\x03\xAAa\x075V[`@\x80Qe\xFF\xFF\xFF\xFF\xFF\xFF\x93\x84\x16\x81R\x92\x90\x91\x16` \x83\x01R\x01a\x01\xCDV[a\x02\x1F`\0\x81V[a\x03\xE4a\x03\xDF6`\x04a\x12\xEEV[a\x07\x89V[`@Qa\x01\xCD\x91\x90a\x13\xB7V[a\x01\xC1a\x03\xFF6`\x04a\x133V[a\t)V[a\x01\xDBa\tkV[a\x01\xFAa\t\xCAV[a\x04\x1Ca\n\nV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x83Re\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16` \x83\x01R\x01a\x01\xCDV[a\x01\xFAa\x04P6`\x04a\x12YV[a\n+V[a\x01\xFAa\nSV[`\0`\x01`\x01`\xE0\x1B\x03\x19\x82\x16c\x18\xA4\xC3\xC3`\xE1\x1B\x14\x80a\x04\x82WPa\x04\x82\x82a\nfV[\x92\x91PPV[`\0a\x04\x93\x81a\n\x9BV[a\x04\x9Ba\n\xA5V[PV[\x81a\x04\xBCW`@Qc\x1F\xE1\xE1=`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x04\xC6\x82\x82a\n\xB2V[PPV[\x81\x15\x80\x15a\x04\xE5WP`\x02T`\x01`\x01`\xA0\x1B\x03\x82\x81\x16\x91\x16\x14[\x15a\x05lW`\0\x80a\x04\xF5a\n\nV[\x90\x92P\x90P`\x01`\x01`\xA0\x1B\x03\x82\x16\x15\x15\x80a\x05\x17WPe\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x15[\x80a\x05*WPBe\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x10\x15[\x15a\x05WW`@Qc\x19\xCA^\xBB`\xE0\x1B\x81Re\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16`\x04\x82\x01R`$\x01[`@Q\x80\x91\x03\x90\xFD[PP`\x01\x80Te\xFF\xFF\xFF\xFF\xFF\xFF`\xA0\x1B\x19\x16\x90U[a\x04\xC6\x82\x82a\n\xDDV[`\0a\x05\x81\x81a\n\x9BV[a\x04\x9Ba\x0B\x15V[`\0a\x05\x94\x81a\n\x9BV[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\x04` R`@\x90 \x82\x90a\x05\xB8\x82\x82a\x16QV[\x90PP\x82`\x01`\x01`\xA0\x1B\x03\x16\x7F\xB3\x84\xAF\xC3\xDE+R2\x7F\x80\x9B\x02f\xD5\xBE\xFE\xDB\xC5\xED\x94\xE7~\x82(\x16\x91\xB0\x80\xD9\xFF\x9C\xCE\x83`@Qa\x05\xF4\x91\x90a\x17\xBCV[`@Q\x80\x91\x03\x90\xA2PPPV[`\0a\x06\x0C\x81a\n\x9BV[a\x06\x1C`@\x83\x01` \x84\x01a\x18\xABV[`\x05\x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x90\x92\x16`\x01`\xA0\x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xA0\x1B\x19\x90\x92\x16\x91\x90\x91\x17\x90Ua\x06V` \x83\x01\x83a\x12\xEEV[`\x05\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U`@Qa\x06\x84\x90\x83\x90a\x18\xC6V[`@Q\x90\x81\x90\x03\x81 \x90\x7F_eK\x8A\x8FhV\xDD\xF1\xDE\xB0\xD7\xFBj\xBF\xD50\xE0\x81i\xDF\xC0\xE5\xE0\xBD\xB3\xEC\xD4U\x93\xC7L\x90`\0\x90\xA2PPV[`\0a\x06\xC3\x81a\n\x9BV[a\x04\xC6\x82a\x0BgV[`\0a\x06\xD7\x81a\n\x9BV[a\x04\xC6\x82a\x0B\xDAV[`\0a\x06\xEB\x81a\n\x9BV[a\x04\x9Ba\x0CJV[`\0a\x07\x07`\x02T`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x90P\x90V[`\0\x91\x82R` \x82\x81R`@\x80\x84 `\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16\x84R\x91\x90R\x90 T`\xFF\x16\x90V[`\x02T`\0\x90`\x01`\xD0\x1B\x90\x04e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x80\x15\x15\x80\x15a\x07aWPBe\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x10\x15[a\x07mW`\0\x80a\x07\x81V[`\x02T`\x01`\xA0\x1B\x90\x04e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81[\x91P\x91P\x90\x91V[a\x07\x91a\x11FV[`\x01`\x01`\xA0\x1B\x03\x80\x83\x16`\0\x90\x81R`\x04` \x90\x81R`@\x80\x83 \x81Q`\xC0\x81\x01\x83R\x81T`\xFF\x90\x81\x16\x15\x15\x82R\x83Q\x80\x85\x01\x85R`\x01\x84\x01T\x97\x88\x16\x81R`\x01`\xA0\x1B\x90\x97\x04\x16\x15\x15\x86\x85\x01R\x80\x84\x01\x95\x90\x95R`\x02\x81\x01\x80T\x83Q\x81\x86\x02\x81\x01\x86\x01\x85R\x81\x81R\x92\x95\x93\x87\x01\x94\x92\x93\x91\x92\x90\x91\x90\x84\x01[\x82\x82\x10\x15a\x08\x83W`\0\x84\x81R` \x90\x81\x90 `@\x80Q`\x80\x81\x01\x82R`\x03\x86\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x81\x16\x84\x84\x01\x90\x81R`\x01`\xA0\x1B\x90\x91\x04`\xFF\x16\x15\x15``\x85\x01R\x83R\x81Q\x80\x83\x01\x90\x92R`\x01\x80\x82\x01T\x83R`\x02\x90\x91\x01T\x82\x85\x01R\x82\x84\x01\x91\x90\x91R\x90\x83R\x90\x92\x01\x91\x01a\x08\x0BV[PPP\x90\x82RP`@\x80Q`\x80\x81\x01\x82R`\x03\x84\x01T`\x01`\x01`\xA0\x1B\x03\x81\x16\x82\x84\x01\x90\x81R`\x01`\xA0\x1B\x90\x91\x04`\xFF\x16\x15\x15``\x80\x84\x01\x91\x90\x91R\x90\x82R\x82Q\x80\x84\x01\x84R`\x04\x86\x01T\x81R`\x05\x86\x01T` \x80\x83\x01\x91\x90\x91R\x80\x84\x01\x91\x90\x91R\x80\x85\x01\x92\x90\x92R\x82Q\x80\x84\x01\x84R`\x06\x86\x01T\x81R`\x07\x86\x01T\x81\x84\x01R\x83\x85\x01R\x82Q\x80\x84\x01\x90\x93R`\x08\x85\x01T\x83R`\t\x90\x94\x01T\x90\x82\x01R\x91\x01R\x92\x91PPV[`\0a\tU\x7F_\xD8E\x82\xB3\x0B\xAC\xE1\xCB\xB5\xCC\x91\xA7[\x8E\xE4\x8A\x0E\x84\xDA\x1Ed\xC2\xD8\x80\xC8\xC8e\xC8\x13DO\x83a\x07\x0CV[\x80\x15a\tdWP`\x03T`\xFF\x16\x15[\x93\x92PPPV[`\x02T`\0\x90`\x01`\xD0\x1B\x90\x04e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x80\x15\x15\x80\x15a\t\x96WPBe\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x10[a\t\xB1W`\x01T`\x01`\xD0\x1B\x90\x04e\xFF\xFF\xFF\xFF\xFF\xFF\x16a\t\xC4V[`\x02T`\x01`\xA0\x1B\x90\x04e\xFF\xFF\xFF\xFF\xFF\xFF\x16[\x91PP\x90V[`\0a\t\xD4a\n\nV[P\x90P3`\x01`\x01`\xA0\x1B\x03\x82\x16\x14a\n\x02W`@Qca\x16@\x11`\xE1\x1B\x81R3`\x04\x82\x01R`$\x01a\x05NV[a\x04\x9Ba\x0C\x87V[`\x01T`\x01`\x01`\xA0\x1B\x03\x81\x16\x91`\x01`\xA0\x1B\x90\x91\x04e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[\x81a\nIW`@Qc\x1F\xE1\xE1=`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x04\xC6\x82\x82a\r V[`\0a\n^\x81a\n\x9BV[a\x04\x9Ba\rEV[`\0`\x01`\x01`\xE0\x1B\x03\x19\x82\x16cye\xDB\x0B`\xE0\x1B\x14\x80a\x04\x82WPc\x01\xFF\xC9\xA7`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x83\x16\x14a\x04\x82V[a\x04\x9B\x813a\rPV[a\n\xB0`\0\x80a\r\x89V[V[`\0\x82\x81R` \x81\x90R`@\x90 `\x01\x01Ta\n\xCD\x81a\n\x9BV[a\n\xD7\x83\x83a\x0EIV[PPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x163\x14a\x0B\x06W`@Qc3K\xD9\x19`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0B\x10\x82\x82a\x0E\xB1V[PPPV[a\x0B\x1Da\x0E\xEEV[`\x03\x80T`\xFF\x19\x16\x90U\x7F]\xB9\xEE\nI[\xF2\xE6\xFF\x9C\x91\xA7\x83L\x1B\xA4\xFD\xD2D\xA5\xE8\xAANS{\xD3\x8A\xEA\xE4\xB0s\xAA3[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01`@Q\x80\x91\x03\x90\xA1V[`\0a\x0Bqa\tkV[a\x0BzBa\x0F\x11V[a\x0B\x84\x91\x90a\x19\x02V[\x90Pa\x0B\x90\x82\x82a\x0FHV[`@Qe\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x7F3w\xDCD$\x1Ew\x9D\xD0j\xFA\xB5\xB7\x88\xA3\\\xA5\xF3\xB7x\x83n)\x90\xBD\xB2j*K.^\xD6\x90` \x01`@Q\x80\x91\x03\x90\xA2PPV[`\0a\x0B\xE5\x82a\x0F\xC7V[a\x0B\xEEBa\x0F\x11V[a\x0B\xF8\x91\x90a\x19\x02V[\x90Pa\x0C\x04\x82\x82a\r\x89V[`@\x80Qe\xFF\xFF\xFF\xFF\xFF\xFF\x80\x85\x16\x82R\x83\x16` \x82\x01R\x7F\xF1\x03\x8C\x18\xCF\x84\xA5nC/\xDB\xFA\xF7F\x92K~\xA5\x11\xDF\xE0:e\x06\xA0\xCE\xBAH\x88x\x8D\x9B\x91\x01`@Q\x80\x91\x03\x90\xA1PPV[a\x0CRa\x10\x0FV[`\x03\x80T`\xFF\x19\x16`\x01\x17\x90U\x7Fb\xE7\x8C\xEA\x01\xBE\xE3 \xCDNB\x02p\xB5\xEAt\0\r\x11\xB0\xC9\xF7GT\xEB\xDB\xFCTK\x05\xA2Xa\x0BJ3\x90V[`\0\x80a\x0C\x92a\n\nV[\x91P\x91Pa\x0C\xA7\x81e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x15\x15\x90V[\x15\x80a\x0C\xBBWPBe\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x10\x15[\x15a\x0C\xE3W`@Qc\x19\xCA^\xBB`\xE0\x1B\x81Re\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16`\x04\x82\x01R`$\x01a\x05NV[a\x0C\xFF`\0a\x0C\xFA`\x02T`\x01`\x01`\xA0\x1B\x03\x16\x90V[a\x0E\xB1V[Pa\r\x0B`\0\x83a\x0EIV[PP`\x01\x80T`\x01`\x01`\xD0\x1B\x03\x19\x16\x90UPV[`\0\x82\x81R` \x81\x90R`@\x90 `\x01\x01Ta\r;\x81a\n\x9BV[a\n\xD7\x83\x83a\x0E\xB1V[a\n\xB0`\0\x80a\x0FHV[a\rZ\x82\x82a\x07\x0CV[a\x04\xC6W`@Qc\xE2Q}?`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R`$\x81\x01\x83\x90R`D\x01a\x05NV[`\x02T`\x01`\xD0\x1B\x90\x04e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x80\x15a\x0E\x0CWBe\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x10\x15a\r\xE2W`\x02T`\x01\x80T`\x01`\x01`\xD0\x1B\x03\x16`\x01`\xA0\x1B\x90\x92\x04e\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x01`\xD0\x1B\x02\x91\x90\x91\x17\x90Ua\x0E\x0CV[`@Q\x7F+\x1F\xA2\xED\xAF\xE6\xF7\xB9\xE9|\x1A\x9E\x0C6`\xE6E\xBE\xB2\xDC\xAA-E\xBD\xBF\x9B\xEA\xF5G.\x1E\xC5\x90`\0\x90\xA1[P`\x02\x80T`\x01`\x01`\xA0\x1B\x03\x16`\x01`\xA0\x1Be\xFF\xFF\xFF\xFF\xFF\xFF\x94\x85\x16\x02`\x01`\x01`\xD0\x1B\x03\x16\x17`\x01`\xD0\x1B\x92\x90\x93\x16\x91\x90\x91\x02\x91\x90\x91\x17\x90UV[`\0\x82a\x0E\xA7W`\0a\x0Ed`\x02T`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x0E\x8BW`@Qc\x1F\xE1\xE1=`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x84\x16\x17\x90U[a\td\x83\x83a\x103V[`\0\x82\x15\x80\x15a\x0E\xCEWP`\x02T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16\x91\x16\x14[\x15a\x0E\xE4W`\x02\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x90U[a\td\x83\x83a\x10\xC5V[`\x03T`\xFF\x16a\n\xB0W`@Qc\x8D\xFC +`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0e\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x0FDW`@Qc\x06\xDF\xCCe`\xE4\x1B\x81R`0`\x04\x82\x01R`$\x81\x01\x83\x90R`D\x01a\x05NV[P\x90V[`\0a\x0FRa\n\nV[`\x01\x80Te\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16`\x01`\xA0\x1B\x02`\x01`\x01`\xD0\x1B\x03\x19\x90\x91\x16`\x01`\x01`\xA0\x1B\x03\x88\x16\x17\x17\x90U\x91Pa\x0F\x94\x90P\x81e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x15\x15\x90V[\x15a\x0B\x10W`@Q\x7F\x88\x86\xEB\xFCBY\xAB\xDB\xC1f\x01\xDD\x8F\xB5g\x8ET\x87\x8FG\xB3\xC3H6\xCF\xC5\x11T\xA9`Q\t\x90`\0\x90\xA1PPPV[`\0\x80a\x0F\xD2a\tkV[\x90P\x80e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11a\x0F\xFAWa\x0F\xF5\x83\x82a\x19(V[a\tdV[a\tde\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16b\x06\x97\x80a\x110V[`\x03T`\xFF\x16\x15a\n\xB0W`@Qc\xD9<\x06e`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x10?\x83\x83a\x07\x0CV[a\x10\xBDW`\0\x83\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x86\x16\x84R\x90\x91R\x90 \x80T`\xFF\x19\x16`\x01\x17\x90Ua\x10u3\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x82`\x01`\x01`\xA0\x1B\x03\x16\x84\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4P`\x01a\x04\x82V[P`\0a\x04\x82V[`\0a\x10\xD1\x83\x83a\x07\x0CV[\x15a\x10\xBDW`\0\x83\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x86\x16\x80\x85R\x92R\x80\x83 \x80T`\xFF\x19\x16\x90UQ3\x92\x86\x91\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B\x91\x90\xA4P`\x01a\x04\x82V[`\0\x81\x83\x10a\x11?W\x81a\tdV[P\x90\x91\x90PV[`@Q\x80`\xC0\x01`@R\x80`\0\x15\x15\x81R` \x01a\x11\x82`@Q\x80`@\x01`@R\x80`\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01`\0\x15\x15\x81RP\x90V[\x81R` \x01``\x81R` \x01a\x11\x96a\x11\xDFV[\x81R` \x01a\x11\xB8`@Q\x80`@\x01`@R\x80`\0\x81R` \x01`\0\x81RP\x90V[\x81R` \x01a\x11\xDA`@Q\x80`@\x01`@R\x80`\0\x81R` \x01`\0\x81RP\x90V[\x90R\x90V[`@\x80Q`\x80\x81\x01\x82R`\0\x91\x81\x01\x82\x81R``\x82\x01\x92\x90\x92R\x90\x81\x90a\x11\xB8V[`\0` \x82\x84\x03\x12\x15a\x12\x13W`\0\x80\xFD[\x815`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a\tdW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x12=W`\0\x80\xFD[P5\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x04\x9BW`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a\x12lW`\0\x80\xFD[\x825\x91P` \x83\x015a\x12~\x81a\x12DV[\x80\x91PP\x92P\x92\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x12\x9CW`\0\x80\xFD[\x825a\x12\xA7\x81a\x12DV[\x91P` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x12\xC3W`\0\x80\xFD[\x83\x01a\x01\x80\x81\x86\x03\x12\x15a\x12~W`\0\x80\xFD[`\0`@\x82\x84\x03\x12\x15a\x12\xE8W`\0\x80\xFD[P\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x13\0W`\0\x80\xFD[\x815a\td\x81a\x12DV[`\0` \x82\x84\x03\x12\x15a\x13\x1DW`\0\x80\xFD[\x815e\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\tdW`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a\x13FW`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x13]W`\0\x80\xFD[\x83\x01a\x01@\x81\x86\x03\x12\x15a\x13pW`\0\x80\xFD[\x91P` \x83\x015a\x12~\x81a\x12DV[a\x13\xA0\x82\x82Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x82R` \x90\x81\x01Q\x15\x15\x91\x01RV[` \x90\x81\x01Q\x80Q`@\x84\x01R\x01Q``\x90\x91\x01RV[`\0` \x80\x83Ra\x01\xA0\x83\x01\x84Q\x15\x15` \x85\x01R` \x85\x01Qa\x13\xF3`@\x86\x01\x82\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x82R` \x90\x81\x01Q\x15\x15\x91\x01RV[P`@\x85\x01Q`\x80a\x01\x80`\x80\x87\x01R\x82\x82Q\x80\x85Ra\x01\xC0\x88\x01\x91P` \x84\x01\x94P`\0\x93P[\x80\x84\x10\x15a\x14BWa\x14.\x82\x86Qa\x13\x80V[\x93\x85\x01\x93`\x01\x93\x90\x93\x01\x92\x90\x82\x01\x90a\x14\x1BV[P``\x88\x01Q\x94Pa\x14W`\xA0\x88\x01\x86a\x13\x80V[`\x80\x88\x01Q\x80Qa\x01 \x89\x01R` \x81\x01Qa\x01@\x89\x01R\x94P`\xA0\x88\x01Q\x80Qa\x01`\x89\x01R` \x81\x01Qa\x01\x80\x89\x01R\x94P\x97\x96PPPPPPPV[\x80\x15\x15\x81\x14a\x04\x9BW`\0\x80\xFD[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a\x14\xBBW`\0\x80\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x14\xD6W`\0\x80\xFD[` \x01\x91P`\x07\x81\x90\x1B6\x03\x82\x13\x15a\x14\xEEW`\0\x80\xFD[\x92P\x92\x90PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x815a\x15\x16\x81a\x12DV[\x81T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x82\x17\x83U` \x84\x015a\x15B\x81a\x14\x96V[`\x01`\x01`\xA8\x1B\x03\x19\x91\x90\x91\x16\x90\x91\x17\x90\x15\x15`\xA0\x1B`\xFF`\xA0\x1B\x16\x17\x81U`@\x82\x015`\x01\x82\x01U``\x90\x91\x015`\x02\x90\x91\x01UV[h\x01\0\0\0\0\0\0\0\0\x83\x11\x15a\x15\xA0WcNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[\x80T\x83\x82U\x80\x84\x10\x15a\x16\x14W`\x03\x81`\x03\x02`\x03\x81\x04\x83\x14a\x15\xC5Wa\x15\xC5a\x14\xF5V[\x85`\x03\x02`\x03\x81\x04\x87\x14a\x15\xDBWa\x15\xDBa\x14\xF5V[`\0\x85\x81R` \x81 \x92\x83\x01\x92\x91\x90\x91\x01\x90[\x82\x82\x10\x15a\x16\x0FW\x80\x82U\x80`\x01\x83\x01U\x80`\x02\x83\x01U\x83\x82\x01\x91Pa\x15\xEEV[PPPP[P`\0\x81\x81R` \x81 \x83\x91[\x85\x81\x10\x15a\x16IWa\x163\x83\x83a\x15\x0BV[`\x80\x92\x90\x92\x01\x91`\x03\x91\x90\x91\x01\x90`\x01\x01a\x16!V[PPPPPPV[\x815a\x16\\\x81a\x14\x96V[\x81T\x90\x15\x15`\xFF\x16`\xFF\x19\x91\x90\x91\x16\x17\x81U`\x01\x81\x01` \x83\x015a\x16\x80\x81a\x12DV[\x81T`@\x85\x015a\x16\x90\x81a\x14\x96V[`\xFF`\xA0\x1B\x90\x15\x15`\xA0\x1B\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16`\x01`\x01`\xA8\x1B\x03\x19\x91\x90\x91\x16\x17\x17\x90Ua\x16\xC6``\x83\x01\x83a\x14\xA4V[a\x16\xD4\x81\x83`\x02\x86\x01a\x15yV[PPa\x16\xE6`\x80\x83\x01`\x03\x83\x01a\x15\x0BV[a\x01\0\x82\x015`\x06\x82\x01Ua\x01 \x82\x015`\x07\x82\x01Ua\x01@\x82\x015`\x08\x82\x01Ua\x01`\x82\x015`\t\x82\x01UPPV[\x805a\x17!\x81a\x12DV[`\x01`\x01`\xA0\x1B\x03\x16\x82R` \x81\x015a\x17:\x81a\x14\x96V[\x80\x15\x15` \x84\x01RPPPV[a\x17Q\x82\x82a\x17\x16V[a\x04\xC6`@\x83\x01`@\x83\x01\x805\x82R` \x90\x81\x015\x91\x01RV[\x81\x83R` \x83\x01\x92P`\0\x81`\0[\x84\x81\x10\x15a\x17\xB2Wa\x17\x8C\x86\x83a\x17\x16V[`@\x82\x81\x015\x90\x87\x01R``\x80\x83\x015\x90\x87\x01R`\x80\x95\x86\x01\x95\x90\x91\x01\x90`\x01\x01a\x17zV[P\x93\x94\x93PPPPV[` \x81R`\0\x825a\x17\xCD\x81a\x14\x96V[\x80\x15\x15` \x84\x01RPa\x17\xE6`@\x83\x01` \x85\x01a\x17\x16V[``\x83\x015`\x1E\x19\x846\x03\x01\x81\x12a\x17\xFDW`\0\x80\xFD[\x83\x01` \x81\x01\x905g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x18\x1AW`\0\x80\xFD[\x80`\x07\x1B6\x03\x82\x13\x15a\x18,W`\0\x80\xFD[a\x01\x80`\x80\x85\x01Ra\x18Ca\x01\xA0\x85\x01\x82\x84a\x17kV[\x91PPa\x18V`\xA0\x84\x01`\x80\x86\x01a\x17GV[a\x18ra\x01 \x84\x01a\x01\0\x86\x01\x805\x82R` \x90\x81\x015\x91\x01RV[a\tda\x01`\x84\x01a\x01@\x86\x01\x805\x82R` \x90\x81\x015\x91\x01RV[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x18\xA6W`\0\x80\xFD[\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x18\xBDW`\0\x80\xFD[a\td\x82a\x18\x8EV[`\0\x825a\x18\xD3\x81a\x12DV[`\x01`\x01`\xA0\x1B\x03\x16\x82Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x18\xF3` \x85\x01a\x18\x8EV[\x16` \x83\x01RP`@\x01\x91\x90PV[e\xFF\xFF\xFF\xFF\xFF\xFF\x81\x81\x16\x83\x82\x16\x01\x90\x80\x82\x11\x15a\x19!Wa\x19!a\x14\xF5V[P\x92\x91PPV[e\xFF\xFF\xFF\xFF\xFF\xFF\x82\x81\x16\x82\x82\x16\x03\x90\x80\x82\x11\x15a\x19!Wa\x19!a\x14\xF5V\xFE\xA2dipfsX\"\x12 v\xCA\x95(r\xE9\r\x1C\xE1?\xCD\xFC\xA3>\xCD\xD5\xE03\xC0qp\xE3\xEF\x16L\x91+\0\xC1r59dsolcC\0\x08\x17\x003";
    /// The deployed bytecode of the contract.
    pub static ZONE_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct Zone<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for Zone<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for Zone<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for Zone<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for Zone<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(Zone)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> Zone<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    ZONE_ABI.clone(),
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
                ZONE_ABI.clone(),
                ZONE_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `DEFAULT_ADMIN_ROLE` (0xa217fddf) function
        pub fn default_admin_role(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([162, 23, 253, 223], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `FULFILLER_ROLE` (0x4fd331c6) function
        pub fn fulfiller_role(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([79, 211, 49, 198], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `acceptDefaultAdminTransfer` (0xcefc1429) function
        pub fn accept_default_admin_transfer(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([206, 252, 20, 41], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `authorizationFilter` (0xa8e98d85) function
        pub fn authorization_filter(
            &self,
            actor: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, AuthFilter> {
            self.0
                .method_hash([168, 233, 141, 133], actor)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `beginDefaultAdminTransfer` (0x634e93da) function
        pub fn begin_default_admin_transfer(
            &self,
            new_admin: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([99, 78, 147, 218], new_admin)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `cancelDefaultAdminTransfer` (0xd602b9fd) function
        pub fn cancel_default_admin_transfer(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([214, 2, 185, 253], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `changeDefaultAdminDelay` (0x649a5ec7) function
        pub fn change_default_admin_delay(
            &self,
            new_delay: u64,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([100, 154, 94, 199], new_delay)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `defaultAdmin` (0x84ef8ffc) function
        pub fn default_admin(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([132, 239, 143, 252], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `defaultAdminDelay` (0xcc8463c8) function
        pub fn default_admin_delay(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([204, 132, 99, 200], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `defaultAdminDelayIncreaseWait` (0x022d63fb) function
        pub fn default_admin_delay_increase_wait(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([2, 45, 99, 251], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `fee` (0x732adaae) function
        pub fn fee(
            &self,
            p0: Order,
            p1: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, FeeInfo> {
            self.0
                .method_hash([115, 42, 218, 174], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getRoleAdmin` (0x248a9ca3) function
        pub fn get_role_admin(
            &self,
            role: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([36, 138, 156, 163], role)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `grantRole` (0x2f2ff15d) function
        pub fn grant_role(
            &self,
            role: [u8; 32],
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([47, 47, 241, 93], (role, account))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `hasRole` (0x91d14854) function
        pub fn has_role(
            &self,
            role: [u8; 32],
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([145, 209, 72, 84], (role, account))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `owner` (0x8da5cb5b) function
        pub fn owner(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([141, 165, 203, 91], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `pause` (0x8456cb59) function
        pub fn pause(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([132, 86, 203, 89], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `paused` (0x5c975abb) function
        pub fn paused(&self) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([92, 151, 90, 187], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `pendingDefaultAdmin` (0xcf6eefb7) function
        pub fn pending_default_admin(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::Address, u64),
        > {
            self.0
                .method_hash([207, 110, 239, 183], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `pendingDefaultAdminDelay` (0xa1eda53c) function
        pub fn pending_default_admin_delay(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, (u64, u64)> {
            self.0
                .method_hash([161, 237, 165, 60], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `renounceRole` (0x36568abe) function
        pub fn renounce_role(
            &self,
            role: [u8; 32],
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([54, 86, 138, 190], (role, account))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `revokeRole` (0xd547741f) function
        pub fn revoke_role(
            &self,
            role: [u8; 32],
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([213, 71, 116, 31], (role, account))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `rollbackDefaultAdminDelay` (0x0aa6220b) function
        pub fn rollback_default_admin_delay(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([10, 166, 34, 11], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setAuthorizationFilter` (0x427196b1) function
        pub fn set_authorization_filter(
            &self,
            actor: ::ethers::core::types::Address,
            filter: AuthFilter,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([66, 113, 150, 177], (actor, filter))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setFee` (0x62f4c6e9) function
        pub fn set_fee(
            &self,
            new_fee: FeeInfo,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([98, 244, 198, 233], (new_fee,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `supportsInterface` (0x01ffc9a7) function
        pub fn supports_interface(
            &self,
            interface_id: [u8; 4],
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([1, 255, 201, 167], interface_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `unpause` (0x3f4ba83a) function
        pub fn unpause(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([63, 75, 168, 58], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `validate` (0xad73d689) function
        pub fn validate(
            &self,
            p0: Order,
            fulfiller: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([173, 115, 214, 137], (p0, fulfiller))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `DefaultAdminDelayChangeCanceled` event
        pub fn default_admin_delay_change_canceled_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            DefaultAdminDelayChangeCanceledFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `DefaultAdminDelayChangeScheduled` event
        pub fn default_admin_delay_change_scheduled_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            DefaultAdminDelayChangeScheduledFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `DefaultAdminTransferCanceled` event
        pub fn default_admin_transfer_canceled_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            DefaultAdminTransferCanceledFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `DefaultAdminTransferScheduled` event
        pub fn default_admin_transfer_scheduled_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            DefaultAdminTransferScheduledFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `FeeUpdated` event
        pub fn fee_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            FeeUpdatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `FilterUpdated` event
        pub fn filter_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            FilterUpdatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `FulfillerUpdated` event
        pub fn fulfiller_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            FulfillerUpdatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Paused` event
        pub fn paused_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, PausedFilter> {
            self.0.event()
        }
        ///Gets the contract's `RoleAdminChanged` event
        pub fn role_admin_changed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RoleAdminChangedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `RoleGranted` event
        pub fn role_granted_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RoleGrantedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `RoleRevoked` event
        pub fn role_revoked_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RoleRevokedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Unpaused` event
        pub fn unpaused_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            UnpausedFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, ZoneEvents> {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for Zone<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `AccessControlBadConfirmation` with signature `AccessControlBadConfirmation()` and selector `0x6697b232`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(
        name = "AccessControlBadConfirmation",
        abi = "AccessControlBadConfirmation()"
    )]
    pub struct AccessControlBadConfirmation;
    ///Custom Error type `AccessControlEnforcedDefaultAdminDelay` with signature `AccessControlEnforcedDefaultAdminDelay(uint48)` and selector `0x19ca5ebb`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(
        name = "AccessControlEnforcedDefaultAdminDelay",
        abi = "AccessControlEnforcedDefaultAdminDelay(uint48)"
    )]
    pub struct AccessControlEnforcedDefaultAdminDelay {
        pub schedule: u64,
    }
    ///Custom Error type `AccessControlEnforcedDefaultAdminRules` with signature `AccessControlEnforcedDefaultAdminRules()` and selector `0x3fc3c27a`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(
        name = "AccessControlEnforcedDefaultAdminRules",
        abi = "AccessControlEnforcedDefaultAdminRules()"
    )]
    pub struct AccessControlEnforcedDefaultAdminRules;
    ///Custom Error type `AccessControlInvalidDefaultAdmin` with signature `AccessControlInvalidDefaultAdmin(address)` and selector `0xc22c8022`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(
        name = "AccessControlInvalidDefaultAdmin",
        abi = "AccessControlInvalidDefaultAdmin(address)"
    )]
    pub struct AccessControlInvalidDefaultAdmin {
        pub default_admin: ::ethers::core::types::Address,
    }
    ///Custom Error type `AccessControlUnauthorizedAccount` with signature `AccessControlUnauthorizedAccount(address,bytes32)` and selector `0xe2517d3f`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(
        name = "AccessControlUnauthorizedAccount",
        abi = "AccessControlUnauthorizedAccount(address,bytes32)"
    )]
    pub struct AccessControlUnauthorizedAccount {
        pub account: ::ethers::core::types::Address,
        pub needed_role: [u8; 32],
    }
    ///Custom Error type `EnforcedPause` with signature `EnforcedPause()` and selector `0xd93c0665`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "EnforcedPause", abi = "EnforcedPause()")]
    pub struct EnforcedPause;
    ///Custom Error type `ExpectedPause` with signature `ExpectedPause()` and selector `0x8dfc202b`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "ExpectedPause", abi = "ExpectedPause()")]
    pub struct ExpectedPause;
    ///Custom Error type `SafeCastOverflowedUintDowncast` with signature `SafeCastOverflowedUintDowncast(uint8,uint256)` and selector `0x6dfcc650`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(
        name = "SafeCastOverflowedUintDowncast",
        abi = "SafeCastOverflowedUintDowncast(uint8,uint256)"
    )]
    pub struct SafeCastOverflowedUintDowncast {
        pub bits: u8,
        pub value: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's custom errors
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        serde::Serialize,
        serde::Deserialize,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub enum ZoneErrors {
        AccessControlBadConfirmation(AccessControlBadConfirmation),
        AccessControlEnforcedDefaultAdminDelay(AccessControlEnforcedDefaultAdminDelay),
        AccessControlEnforcedDefaultAdminRules(AccessControlEnforcedDefaultAdminRules),
        AccessControlInvalidDefaultAdmin(AccessControlInvalidDefaultAdmin),
        AccessControlUnauthorizedAccount(AccessControlUnauthorizedAccount),
        EnforcedPause(EnforcedPause),
        ExpectedPause(ExpectedPause),
        SafeCastOverflowedUintDowncast(SafeCastOverflowedUintDowncast),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for ZoneErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <AccessControlBadConfirmation as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AccessControlBadConfirmation(decoded));
            }
            if let Ok(decoded) = <AccessControlEnforcedDefaultAdminDelay as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AccessControlEnforcedDefaultAdminDelay(decoded));
            }
            if let Ok(decoded) = <AccessControlEnforcedDefaultAdminRules as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AccessControlEnforcedDefaultAdminRules(decoded));
            }
            if let Ok(decoded) = <AccessControlInvalidDefaultAdmin as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AccessControlInvalidDefaultAdmin(decoded));
            }
            if let Ok(decoded) = <AccessControlUnauthorizedAccount as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AccessControlUnauthorizedAccount(decoded));
            }
            if let Ok(decoded) = <EnforcedPause as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EnforcedPause(decoded));
            }
            if let Ok(decoded) = <ExpectedPause as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ExpectedPause(decoded));
            }
            if let Ok(decoded) = <SafeCastOverflowedUintDowncast as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SafeCastOverflowedUintDowncast(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ZoneErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::AccessControlBadConfirmation(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AccessControlEnforcedDefaultAdminDelay(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AccessControlEnforcedDefaultAdminRules(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AccessControlInvalidDefaultAdmin(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AccessControlUnauthorizedAccount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EnforcedPause(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExpectedPause(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SafeCastOverflowedUintDowncast(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for ZoneErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <AccessControlBadConfirmation as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <AccessControlEnforcedDefaultAdminDelay as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <AccessControlEnforcedDefaultAdminRules as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <AccessControlInvalidDefaultAdmin as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <AccessControlUnauthorizedAccount as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <EnforcedPause as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ExpectedPause as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <SafeCastOverflowedUintDowncast as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for ZoneErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AccessControlBadConfirmation(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AccessControlEnforcedDefaultAdminDelay(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AccessControlEnforcedDefaultAdminRules(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AccessControlInvalidDefaultAdmin(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AccessControlUnauthorizedAccount(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::EnforcedPause(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExpectedPause(element) => ::core::fmt::Display::fmt(element, f),
                Self::SafeCastOverflowedUintDowncast(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for ZoneErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<AccessControlBadConfirmation> for ZoneErrors {
        fn from(value: AccessControlBadConfirmation) -> Self {
            Self::AccessControlBadConfirmation(value)
        }
    }
    impl ::core::convert::From<AccessControlEnforcedDefaultAdminDelay> for ZoneErrors {
        fn from(value: AccessControlEnforcedDefaultAdminDelay) -> Self {
            Self::AccessControlEnforcedDefaultAdminDelay(value)
        }
    }
    impl ::core::convert::From<AccessControlEnforcedDefaultAdminRules> for ZoneErrors {
        fn from(value: AccessControlEnforcedDefaultAdminRules) -> Self {
            Self::AccessControlEnforcedDefaultAdminRules(value)
        }
    }
    impl ::core::convert::From<AccessControlInvalidDefaultAdmin> for ZoneErrors {
        fn from(value: AccessControlInvalidDefaultAdmin) -> Self {
            Self::AccessControlInvalidDefaultAdmin(value)
        }
    }
    impl ::core::convert::From<AccessControlUnauthorizedAccount> for ZoneErrors {
        fn from(value: AccessControlUnauthorizedAccount) -> Self {
            Self::AccessControlUnauthorizedAccount(value)
        }
    }
    impl ::core::convert::From<EnforcedPause> for ZoneErrors {
        fn from(value: EnforcedPause) -> Self {
            Self::EnforcedPause(value)
        }
    }
    impl ::core::convert::From<ExpectedPause> for ZoneErrors {
        fn from(value: ExpectedPause) -> Self {
            Self::ExpectedPause(value)
        }
    }
    impl ::core::convert::From<SafeCastOverflowedUintDowncast> for ZoneErrors {
        fn from(value: SafeCastOverflowedUintDowncast) -> Self {
            Self::SafeCastOverflowedUintDowncast(value)
        }
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "DefaultAdminDelayChangeCanceled",
        abi = "DefaultAdminDelayChangeCanceled()"
    )]
    pub struct DefaultAdminDelayChangeCanceledFilter;
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "DefaultAdminDelayChangeScheduled",
        abi = "DefaultAdminDelayChangeScheduled(uint48,uint48)"
    )]
    pub struct DefaultAdminDelayChangeScheduledFilter {
        pub new_delay: u64,
        pub effect_schedule: u64,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "DefaultAdminTransferCanceled",
        abi = "DefaultAdminTransferCanceled()"
    )]
    pub struct DefaultAdminTransferCanceledFilter;
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "DefaultAdminTransferScheduled",
        abi = "DefaultAdminTransferScheduled(address,uint48)"
    )]
    pub struct DefaultAdminTransferScheduledFilter {
        #[ethevent(indexed)]
        pub new_admin: ::ethers::core::types::Address,
        pub accept_schedule: u64,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "FeeUpdated", abi = "FeeUpdated((address,uint64))")]
    pub struct FeeUpdatedFilter {
        #[ethevent(indexed)]
        pub new_fee: ::ethers::core::types::H256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "FilterUpdated",
        abi = "FilterUpdated(address,(bool,(address,bool),((address,bool),(uint256,uint256))[],((address,bool),(uint256,uint256)),(uint256,uint256),(uint256,uint256)))"
    )]
    pub struct FilterUpdatedFilter {
        #[ethevent(indexed)]
        pub actor: ::ethers::core::types::Address,
        pub filter: AuthFilter,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "FulfillerUpdated", abi = "FulfillerUpdated(address,bool)")]
    pub struct FulfillerUpdatedFilter {
        #[ethevent(indexed)]
        pub fulfiller: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub valid: bool,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "Paused", abi = "Paused(address)")]
    pub struct PausedFilter {
        pub account: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "RoleAdminChanged",
        abi = "RoleAdminChanged(bytes32,bytes32,bytes32)"
    )]
    pub struct RoleAdminChangedFilter {
        #[ethevent(indexed)]
        pub role: [u8; 32],
        #[ethevent(indexed)]
        pub previous_admin_role: [u8; 32],
        #[ethevent(indexed)]
        pub new_admin_role: [u8; 32],
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "RoleGranted", abi = "RoleGranted(bytes32,address,address)")]
    pub struct RoleGrantedFilter {
        #[ethevent(indexed)]
        pub role: [u8; 32],
        #[ethevent(indexed)]
        pub account: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub sender: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "RoleRevoked", abi = "RoleRevoked(bytes32,address,address)")]
    pub struct RoleRevokedFilter {
        #[ethevent(indexed)]
        pub role: [u8; 32],
        #[ethevent(indexed)]
        pub account: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub sender: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "Unpaused", abi = "Unpaused(address)")]
    pub struct UnpausedFilter {
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's events
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        serde::Serialize,
        serde::Deserialize,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub enum ZoneEvents {
        DefaultAdminDelayChangeCanceledFilter(DefaultAdminDelayChangeCanceledFilter),
        DefaultAdminDelayChangeScheduledFilter(DefaultAdminDelayChangeScheduledFilter),
        DefaultAdminTransferCanceledFilter(DefaultAdminTransferCanceledFilter),
        DefaultAdminTransferScheduledFilter(DefaultAdminTransferScheduledFilter),
        FeeUpdatedFilter(FeeUpdatedFilter),
        FilterUpdatedFilter(FilterUpdatedFilter),
        FulfillerUpdatedFilter(FulfillerUpdatedFilter),
        PausedFilter(PausedFilter),
        RoleAdminChangedFilter(RoleAdminChangedFilter),
        RoleGrantedFilter(RoleGrantedFilter),
        RoleRevokedFilter(RoleRevokedFilter),
        UnpausedFilter(UnpausedFilter),
    }
    impl ::ethers::contract::EthLogDecode for ZoneEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = DefaultAdminDelayChangeCanceledFilter::decode_log(log) {
                return Ok(ZoneEvents::DefaultAdminDelayChangeCanceledFilter(decoded));
            }
            if let Ok(decoded) = DefaultAdminDelayChangeScheduledFilter::decode_log(
                log,
            ) {
                return Ok(ZoneEvents::DefaultAdminDelayChangeScheduledFilter(decoded));
            }
            if let Ok(decoded) = DefaultAdminTransferCanceledFilter::decode_log(log) {
                return Ok(ZoneEvents::DefaultAdminTransferCanceledFilter(decoded));
            }
            if let Ok(decoded) = DefaultAdminTransferScheduledFilter::decode_log(log) {
                return Ok(ZoneEvents::DefaultAdminTransferScheduledFilter(decoded));
            }
            if let Ok(decoded) = FeeUpdatedFilter::decode_log(log) {
                return Ok(ZoneEvents::FeeUpdatedFilter(decoded));
            }
            if let Ok(decoded) = FilterUpdatedFilter::decode_log(log) {
                return Ok(ZoneEvents::FilterUpdatedFilter(decoded));
            }
            if let Ok(decoded) = FulfillerUpdatedFilter::decode_log(log) {
                return Ok(ZoneEvents::FulfillerUpdatedFilter(decoded));
            }
            if let Ok(decoded) = PausedFilter::decode_log(log) {
                return Ok(ZoneEvents::PausedFilter(decoded));
            }
            if let Ok(decoded) = RoleAdminChangedFilter::decode_log(log) {
                return Ok(ZoneEvents::RoleAdminChangedFilter(decoded));
            }
            if let Ok(decoded) = RoleGrantedFilter::decode_log(log) {
                return Ok(ZoneEvents::RoleGrantedFilter(decoded));
            }
            if let Ok(decoded) = RoleRevokedFilter::decode_log(log) {
                return Ok(ZoneEvents::RoleRevokedFilter(decoded));
            }
            if let Ok(decoded) = UnpausedFilter::decode_log(log) {
                return Ok(ZoneEvents::UnpausedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for ZoneEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::DefaultAdminDelayChangeCanceledFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DefaultAdminDelayChangeScheduledFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DefaultAdminTransferCanceledFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DefaultAdminTransferScheduledFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::FeeUpdatedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::FilterUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::FulfillerUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PausedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RoleAdminChangedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RoleGrantedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RoleRevokedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::UnpausedFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<DefaultAdminDelayChangeCanceledFilter> for ZoneEvents {
        fn from(value: DefaultAdminDelayChangeCanceledFilter) -> Self {
            Self::DefaultAdminDelayChangeCanceledFilter(value)
        }
    }
    impl ::core::convert::From<DefaultAdminDelayChangeScheduledFilter> for ZoneEvents {
        fn from(value: DefaultAdminDelayChangeScheduledFilter) -> Self {
            Self::DefaultAdminDelayChangeScheduledFilter(value)
        }
    }
    impl ::core::convert::From<DefaultAdminTransferCanceledFilter> for ZoneEvents {
        fn from(value: DefaultAdminTransferCanceledFilter) -> Self {
            Self::DefaultAdminTransferCanceledFilter(value)
        }
    }
    impl ::core::convert::From<DefaultAdminTransferScheduledFilter> for ZoneEvents {
        fn from(value: DefaultAdminTransferScheduledFilter) -> Self {
            Self::DefaultAdminTransferScheduledFilter(value)
        }
    }
    impl ::core::convert::From<FeeUpdatedFilter> for ZoneEvents {
        fn from(value: FeeUpdatedFilter) -> Self {
            Self::FeeUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<FilterUpdatedFilter> for ZoneEvents {
        fn from(value: FilterUpdatedFilter) -> Self {
            Self::FilterUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<FulfillerUpdatedFilter> for ZoneEvents {
        fn from(value: FulfillerUpdatedFilter) -> Self {
            Self::FulfillerUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<PausedFilter> for ZoneEvents {
        fn from(value: PausedFilter) -> Self {
            Self::PausedFilter(value)
        }
    }
    impl ::core::convert::From<RoleAdminChangedFilter> for ZoneEvents {
        fn from(value: RoleAdminChangedFilter) -> Self {
            Self::RoleAdminChangedFilter(value)
        }
    }
    impl ::core::convert::From<RoleGrantedFilter> for ZoneEvents {
        fn from(value: RoleGrantedFilter) -> Self {
            Self::RoleGrantedFilter(value)
        }
    }
    impl ::core::convert::From<RoleRevokedFilter> for ZoneEvents {
        fn from(value: RoleRevokedFilter) -> Self {
            Self::RoleRevokedFilter(value)
        }
    }
    impl ::core::convert::From<UnpausedFilter> for ZoneEvents {
        fn from(value: UnpausedFilter) -> Self {
            Self::UnpausedFilter(value)
        }
    }
    ///Container type for all input parameters for the `DEFAULT_ADMIN_ROLE` function with signature `DEFAULT_ADMIN_ROLE()` and selector `0xa217fddf`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "DEFAULT_ADMIN_ROLE", abi = "DEFAULT_ADMIN_ROLE()")]
    pub struct DefaultAdminRoleCall;
    ///Container type for all input parameters for the `FULFILLER_ROLE` function with signature `FULFILLER_ROLE()` and selector `0x4fd331c6`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "FULFILLER_ROLE", abi = "FULFILLER_ROLE()")]
    pub struct FulfillerRoleCall;
    ///Container type for all input parameters for the `acceptDefaultAdminTransfer` function with signature `acceptDefaultAdminTransfer()` and selector `0xcefc1429`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "acceptDefaultAdminTransfer", abi = "acceptDefaultAdminTransfer()")]
    pub struct AcceptDefaultAdminTransferCall;
    ///Container type for all input parameters for the `authorizationFilter` function with signature `authorizationFilter(address)` and selector `0xa8e98d85`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "authorizationFilter", abi = "authorizationFilter(address)")]
    pub struct AuthorizationFilterCall {
        pub actor: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `beginDefaultAdminTransfer` function with signature `beginDefaultAdminTransfer(address)` and selector `0x634e93da`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "beginDefaultAdminTransfer",
        abi = "beginDefaultAdminTransfer(address)"
    )]
    pub struct BeginDefaultAdminTransferCall {
        pub new_admin: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `cancelDefaultAdminTransfer` function with signature `cancelDefaultAdminTransfer()` and selector `0xd602b9fd`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "cancelDefaultAdminTransfer", abi = "cancelDefaultAdminTransfer()")]
    pub struct CancelDefaultAdminTransferCall;
    ///Container type for all input parameters for the `changeDefaultAdminDelay` function with signature `changeDefaultAdminDelay(uint48)` and selector `0x649a5ec7`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "changeDefaultAdminDelay", abi = "changeDefaultAdminDelay(uint48)")]
    pub struct ChangeDefaultAdminDelayCall {
        pub new_delay: u64,
    }
    ///Container type for all input parameters for the `defaultAdmin` function with signature `defaultAdmin()` and selector `0x84ef8ffc`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "defaultAdmin", abi = "defaultAdmin()")]
    pub struct DefaultAdminCall;
    ///Container type for all input parameters for the `defaultAdminDelay` function with signature `defaultAdminDelay()` and selector `0xcc8463c8`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "defaultAdminDelay", abi = "defaultAdminDelay()")]
    pub struct DefaultAdminDelayCall;
    ///Container type for all input parameters for the `defaultAdminDelayIncreaseWait` function with signature `defaultAdminDelayIncreaseWait()` and selector `0x022d63fb`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "defaultAdminDelayIncreaseWait",
        abi = "defaultAdminDelayIncreaseWait()"
    )]
    pub struct DefaultAdminDelayIncreaseWaitCall;
    ///Container type for all input parameters for the `fee` function with signature `fee((address,address,address,(address,uint256)[],(address,uint256),uint256,uint256,(address,bytes)[],(address,bytes)[]),address)` and selector `0x732adaae`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "fee",
        abi = "fee((address,address,address,(address,uint256)[],(address,uint256),uint256,uint256,(address,bytes)[],(address,bytes)[]),address)"
    )]
    pub struct FeeCall(pub Order, pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `getRoleAdmin` function with signature `getRoleAdmin(bytes32)` and selector `0x248a9ca3`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getRoleAdmin", abi = "getRoleAdmin(bytes32)")]
    pub struct GetRoleAdminCall {
        pub role: [u8; 32],
    }
    ///Container type for all input parameters for the `grantRole` function with signature `grantRole(bytes32,address)` and selector `0x2f2ff15d`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "grantRole", abi = "grantRole(bytes32,address)")]
    pub struct GrantRoleCall {
        pub role: [u8; 32],
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `hasRole` function with signature `hasRole(bytes32,address)` and selector `0x91d14854`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "hasRole", abi = "hasRole(bytes32,address)")]
    pub struct HasRoleCall {
        pub role: [u8; 32],
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `owner` function with signature `owner()` and selector `0x8da5cb5b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "owner", abi = "owner()")]
    pub struct OwnerCall;
    ///Container type for all input parameters for the `pause` function with signature `pause()` and selector `0x8456cb59`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "pause", abi = "pause()")]
    pub struct PauseCall;
    ///Container type for all input parameters for the `paused` function with signature `paused()` and selector `0x5c975abb`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "paused", abi = "paused()")]
    pub struct PausedCall;
    ///Container type for all input parameters for the `pendingDefaultAdmin` function with signature `pendingDefaultAdmin()` and selector `0xcf6eefb7`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "pendingDefaultAdmin", abi = "pendingDefaultAdmin()")]
    pub struct PendingDefaultAdminCall;
    ///Container type for all input parameters for the `pendingDefaultAdminDelay` function with signature `pendingDefaultAdminDelay()` and selector `0xa1eda53c`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "pendingDefaultAdminDelay", abi = "pendingDefaultAdminDelay()")]
    pub struct PendingDefaultAdminDelayCall;
    ///Container type for all input parameters for the `renounceRole` function with signature `renounceRole(bytes32,address)` and selector `0x36568abe`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "renounceRole", abi = "renounceRole(bytes32,address)")]
    pub struct RenounceRoleCall {
        pub role: [u8; 32],
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `revokeRole` function with signature `revokeRole(bytes32,address)` and selector `0xd547741f`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "revokeRole", abi = "revokeRole(bytes32,address)")]
    pub struct RevokeRoleCall {
        pub role: [u8; 32],
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `rollbackDefaultAdminDelay` function with signature `rollbackDefaultAdminDelay()` and selector `0x0aa6220b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "rollbackDefaultAdminDelay", abi = "rollbackDefaultAdminDelay()")]
    pub struct RollbackDefaultAdminDelayCall;
    ///Container type for all input parameters for the `setAuthorizationFilter` function with signature `setAuthorizationFilter(address,(bool,(address,bool),((address,bool),(uint256,uint256))[],((address,bool),(uint256,uint256)),(uint256,uint256),(uint256,uint256)))` and selector `0x427196b1`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "setAuthorizationFilter",
        abi = "setAuthorizationFilter(address,(bool,(address,bool),((address,bool),(uint256,uint256))[],((address,bool),(uint256,uint256)),(uint256,uint256),(uint256,uint256)))"
    )]
    pub struct SetAuthorizationFilterCall {
        pub actor: ::ethers::core::types::Address,
        pub filter: AuthFilter,
    }
    ///Container type for all input parameters for the `setFee` function with signature `setFee((address,uint64))` and selector `0x62f4c6e9`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "setFee", abi = "setFee((address,uint64))")]
    pub struct SetFeeCall {
        pub new_fee: FeeInfo,
    }
    ///Container type for all input parameters for the `supportsInterface` function with signature `supportsInterface(bytes4)` and selector `0x01ffc9a7`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "supportsInterface", abi = "supportsInterface(bytes4)")]
    pub struct SupportsInterfaceCall {
        pub interface_id: [u8; 4],
    }
    ///Container type for all input parameters for the `unpause` function with signature `unpause()` and selector `0x3f4ba83a`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "unpause", abi = "unpause()")]
    pub struct UnpauseCall;
    ///Container type for all input parameters for the `validate` function with signature `validate((address,address,address,(address,uint256)[],(address,uint256),uint256,uint256,(address,bytes)[],(address,bytes)[]),address)` and selector `0xad73d689`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "validate",
        abi = "validate((address,address,address,(address,uint256)[],(address,uint256),uint256,uint256,(address,bytes)[],(address,bytes)[]),address)"
    )]
    pub struct ValidateCall {
        pub p0: Order,
        pub fulfiller: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's call
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        serde::Serialize,
        serde::Deserialize,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub enum ZoneCalls {
        DefaultAdminRole(DefaultAdminRoleCall),
        FulfillerRole(FulfillerRoleCall),
        AcceptDefaultAdminTransfer(AcceptDefaultAdminTransferCall),
        AuthorizationFilter(AuthorizationFilterCall),
        BeginDefaultAdminTransfer(BeginDefaultAdminTransferCall),
        CancelDefaultAdminTransfer(CancelDefaultAdminTransferCall),
        ChangeDefaultAdminDelay(ChangeDefaultAdminDelayCall),
        DefaultAdmin(DefaultAdminCall),
        DefaultAdminDelay(DefaultAdminDelayCall),
        DefaultAdminDelayIncreaseWait(DefaultAdminDelayIncreaseWaitCall),
        Fee(FeeCall),
        GetRoleAdmin(GetRoleAdminCall),
        GrantRole(GrantRoleCall),
        HasRole(HasRoleCall),
        Owner(OwnerCall),
        Pause(PauseCall),
        Paused(PausedCall),
        PendingDefaultAdmin(PendingDefaultAdminCall),
        PendingDefaultAdminDelay(PendingDefaultAdminDelayCall),
        RenounceRole(RenounceRoleCall),
        RevokeRole(RevokeRoleCall),
        RollbackDefaultAdminDelay(RollbackDefaultAdminDelayCall),
        SetAuthorizationFilter(SetAuthorizationFilterCall),
        SetFee(SetFeeCall),
        SupportsInterface(SupportsInterfaceCall),
        Unpause(UnpauseCall),
        Validate(ValidateCall),
    }
    impl ::ethers::core::abi::AbiDecode for ZoneCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <DefaultAdminRoleCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DefaultAdminRole(decoded));
            }
            if let Ok(decoded) = <FulfillerRoleCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::FulfillerRole(decoded));
            }
            if let Ok(decoded) = <AcceptDefaultAdminTransferCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AcceptDefaultAdminTransfer(decoded));
            }
            if let Ok(decoded) = <AuthorizationFilterCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AuthorizationFilter(decoded));
            }
            if let Ok(decoded) = <BeginDefaultAdminTransferCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::BeginDefaultAdminTransfer(decoded));
            }
            if let Ok(decoded) = <CancelDefaultAdminTransferCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CancelDefaultAdminTransfer(decoded));
            }
            if let Ok(decoded) = <ChangeDefaultAdminDelayCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ChangeDefaultAdminDelay(decoded));
            }
            if let Ok(decoded) = <DefaultAdminCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DefaultAdmin(decoded));
            }
            if let Ok(decoded) = <DefaultAdminDelayCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DefaultAdminDelay(decoded));
            }
            if let Ok(decoded) = <DefaultAdminDelayIncreaseWaitCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DefaultAdminDelayIncreaseWait(decoded));
            }
            if let Ok(decoded) = <FeeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Fee(decoded));
            }
            if let Ok(decoded) = <GetRoleAdminCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetRoleAdmin(decoded));
            }
            if let Ok(decoded) = <GrantRoleCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GrantRole(decoded));
            }
            if let Ok(decoded) = <HasRoleCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::HasRole(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded) = <PauseCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Pause(decoded));
            }
            if let Ok(decoded) = <PausedCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Paused(decoded));
            }
            if let Ok(decoded) = <PendingDefaultAdminCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PendingDefaultAdmin(decoded));
            }
            if let Ok(decoded) = <PendingDefaultAdminDelayCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PendingDefaultAdminDelay(decoded));
            }
            if let Ok(decoded) = <RenounceRoleCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RenounceRole(decoded));
            }
            if let Ok(decoded) = <RevokeRoleCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevokeRole(decoded));
            }
            if let Ok(decoded) = <RollbackDefaultAdminDelayCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RollbackDefaultAdminDelay(decoded));
            }
            if let Ok(decoded) = <SetAuthorizationFilterCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetAuthorizationFilter(decoded));
            }
            if let Ok(decoded) = <SetFeeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetFee(decoded));
            }
            if let Ok(decoded) = <SupportsInterfaceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SupportsInterface(decoded));
            }
            if let Ok(decoded) = <UnpauseCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Unpause(decoded));
            }
            if let Ok(decoded) = <ValidateCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Validate(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ZoneCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::DefaultAdminRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FulfillerRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AcceptDefaultAdminTransfer(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AuthorizationFilter(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BeginDefaultAdminTransfer(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CancelDefaultAdminTransfer(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ChangeDefaultAdminDelay(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DefaultAdmin(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DefaultAdminDelay(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DefaultAdminDelayIncreaseWait(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Fee(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetRoleAdmin(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GrantRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::HasRole(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Pause(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Paused(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PendingDefaultAdmin(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PendingDefaultAdminDelay(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RenounceRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevokeRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RollbackDefaultAdminDelay(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetAuthorizationFilter(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetFee(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SupportsInterface(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Unpause(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Validate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for ZoneCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::DefaultAdminRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::FulfillerRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::AcceptDefaultAdminTransfer(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AuthorizationFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::BeginDefaultAdminTransfer(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CancelDefaultAdminTransfer(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ChangeDefaultAdminDelay(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DefaultAdmin(element) => ::core::fmt::Display::fmt(element, f),
                Self::DefaultAdminDelay(element) => ::core::fmt::Display::fmt(element, f),
                Self::DefaultAdminDelayIncreaseWait(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Fee(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetRoleAdmin(element) => ::core::fmt::Display::fmt(element, f),
                Self::GrantRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::HasRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::Pause(element) => ::core::fmt::Display::fmt(element, f),
                Self::Paused(element) => ::core::fmt::Display::fmt(element, f),
                Self::PendingDefaultAdmin(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PendingDefaultAdminDelay(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RenounceRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevokeRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::RollbackDefaultAdminDelay(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetAuthorizationFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::SupportsInterface(element) => ::core::fmt::Display::fmt(element, f),
                Self::Unpause(element) => ::core::fmt::Display::fmt(element, f),
                Self::Validate(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<DefaultAdminRoleCall> for ZoneCalls {
        fn from(value: DefaultAdminRoleCall) -> Self {
            Self::DefaultAdminRole(value)
        }
    }
    impl ::core::convert::From<FulfillerRoleCall> for ZoneCalls {
        fn from(value: FulfillerRoleCall) -> Self {
            Self::FulfillerRole(value)
        }
    }
    impl ::core::convert::From<AcceptDefaultAdminTransferCall> for ZoneCalls {
        fn from(value: AcceptDefaultAdminTransferCall) -> Self {
            Self::AcceptDefaultAdminTransfer(value)
        }
    }
    impl ::core::convert::From<AuthorizationFilterCall> for ZoneCalls {
        fn from(value: AuthorizationFilterCall) -> Self {
            Self::AuthorizationFilter(value)
        }
    }
    impl ::core::convert::From<BeginDefaultAdminTransferCall> for ZoneCalls {
        fn from(value: BeginDefaultAdminTransferCall) -> Self {
            Self::BeginDefaultAdminTransfer(value)
        }
    }
    impl ::core::convert::From<CancelDefaultAdminTransferCall> for ZoneCalls {
        fn from(value: CancelDefaultAdminTransferCall) -> Self {
            Self::CancelDefaultAdminTransfer(value)
        }
    }
    impl ::core::convert::From<ChangeDefaultAdminDelayCall> for ZoneCalls {
        fn from(value: ChangeDefaultAdminDelayCall) -> Self {
            Self::ChangeDefaultAdminDelay(value)
        }
    }
    impl ::core::convert::From<DefaultAdminCall> for ZoneCalls {
        fn from(value: DefaultAdminCall) -> Self {
            Self::DefaultAdmin(value)
        }
    }
    impl ::core::convert::From<DefaultAdminDelayCall> for ZoneCalls {
        fn from(value: DefaultAdminDelayCall) -> Self {
            Self::DefaultAdminDelay(value)
        }
    }
    impl ::core::convert::From<DefaultAdminDelayIncreaseWaitCall> for ZoneCalls {
        fn from(value: DefaultAdminDelayIncreaseWaitCall) -> Self {
            Self::DefaultAdminDelayIncreaseWait(value)
        }
    }
    impl ::core::convert::From<FeeCall> for ZoneCalls {
        fn from(value: FeeCall) -> Self {
            Self::Fee(value)
        }
    }
    impl ::core::convert::From<GetRoleAdminCall> for ZoneCalls {
        fn from(value: GetRoleAdminCall) -> Self {
            Self::GetRoleAdmin(value)
        }
    }
    impl ::core::convert::From<GrantRoleCall> for ZoneCalls {
        fn from(value: GrantRoleCall) -> Self {
            Self::GrantRole(value)
        }
    }
    impl ::core::convert::From<HasRoleCall> for ZoneCalls {
        fn from(value: HasRoleCall) -> Self {
            Self::HasRole(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for ZoneCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<PauseCall> for ZoneCalls {
        fn from(value: PauseCall) -> Self {
            Self::Pause(value)
        }
    }
    impl ::core::convert::From<PausedCall> for ZoneCalls {
        fn from(value: PausedCall) -> Self {
            Self::Paused(value)
        }
    }
    impl ::core::convert::From<PendingDefaultAdminCall> for ZoneCalls {
        fn from(value: PendingDefaultAdminCall) -> Self {
            Self::PendingDefaultAdmin(value)
        }
    }
    impl ::core::convert::From<PendingDefaultAdminDelayCall> for ZoneCalls {
        fn from(value: PendingDefaultAdminDelayCall) -> Self {
            Self::PendingDefaultAdminDelay(value)
        }
    }
    impl ::core::convert::From<RenounceRoleCall> for ZoneCalls {
        fn from(value: RenounceRoleCall) -> Self {
            Self::RenounceRole(value)
        }
    }
    impl ::core::convert::From<RevokeRoleCall> for ZoneCalls {
        fn from(value: RevokeRoleCall) -> Self {
            Self::RevokeRole(value)
        }
    }
    impl ::core::convert::From<RollbackDefaultAdminDelayCall> for ZoneCalls {
        fn from(value: RollbackDefaultAdminDelayCall) -> Self {
            Self::RollbackDefaultAdminDelay(value)
        }
    }
    impl ::core::convert::From<SetAuthorizationFilterCall> for ZoneCalls {
        fn from(value: SetAuthorizationFilterCall) -> Self {
            Self::SetAuthorizationFilter(value)
        }
    }
    impl ::core::convert::From<SetFeeCall> for ZoneCalls {
        fn from(value: SetFeeCall) -> Self {
            Self::SetFee(value)
        }
    }
    impl ::core::convert::From<SupportsInterfaceCall> for ZoneCalls {
        fn from(value: SupportsInterfaceCall) -> Self {
            Self::SupportsInterface(value)
        }
    }
    impl ::core::convert::From<UnpauseCall> for ZoneCalls {
        fn from(value: UnpauseCall) -> Self {
            Self::Unpause(value)
        }
    }
    impl ::core::convert::From<ValidateCall> for ZoneCalls {
        fn from(value: ValidateCall) -> Self {
            Self::Validate(value)
        }
    }
    ///Container type for all return fields from the `DEFAULT_ADMIN_ROLE` function with signature `DEFAULT_ADMIN_ROLE()` and selector `0xa217fddf`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct DefaultAdminRoleReturn(pub [u8; 32]);
    ///Container type for all return fields from the `FULFILLER_ROLE` function with signature `FULFILLER_ROLE()` and selector `0x4fd331c6`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct FulfillerRoleReturn(pub [u8; 32]);
    ///Container type for all return fields from the `authorizationFilter` function with signature `authorizationFilter(address)` and selector `0xa8e98d85`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct AuthorizationFilterReturn(pub AuthFilter);
    ///Container type for all return fields from the `defaultAdmin` function with signature `defaultAdmin()` and selector `0x84ef8ffc`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct DefaultAdminReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `defaultAdminDelay` function with signature `defaultAdminDelay()` and selector `0xcc8463c8`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct DefaultAdminDelayReturn(pub u64);
    ///Container type for all return fields from the `defaultAdminDelayIncreaseWait` function with signature `defaultAdminDelayIncreaseWait()` and selector `0x022d63fb`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct DefaultAdminDelayIncreaseWaitReturn(pub u64);
    ///Container type for all return fields from the `fee` function with signature `fee((address,address,address,(address,uint256)[],(address,uint256),uint256,uint256,(address,bytes)[],(address,bytes)[]),address)` and selector `0x732adaae`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct FeeReturn(pub FeeInfo);
    ///Container type for all return fields from the `getRoleAdmin` function with signature `getRoleAdmin(bytes32)` and selector `0x248a9ca3`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct GetRoleAdminReturn(pub [u8; 32]);
    ///Container type for all return fields from the `hasRole` function with signature `hasRole(bytes32,address)` and selector `0x91d14854`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct HasRoleReturn(pub bool);
    ///Container type for all return fields from the `owner` function with signature `owner()` and selector `0x8da5cb5b`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct OwnerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `paused` function with signature `paused()` and selector `0x5c975abb`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct PausedReturn(pub bool);
    ///Container type for all return fields from the `pendingDefaultAdmin` function with signature `pendingDefaultAdmin()` and selector `0xcf6eefb7`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct PendingDefaultAdminReturn {
        pub new_admin: ::ethers::core::types::Address,
        pub schedule: u64,
    }
    ///Container type for all return fields from the `pendingDefaultAdminDelay` function with signature `pendingDefaultAdminDelay()` and selector `0xa1eda53c`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct PendingDefaultAdminDelayReturn {
        pub new_delay: u64,
        pub schedule: u64,
    }
    ///Container type for all return fields from the `supportsInterface` function with signature `supportsInterface(bytes4)` and selector `0x01ffc9a7`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct SupportsInterfaceReturn(pub bool);
    ///Container type for all return fields from the `validate` function with signature `validate((address,address,address,(address,uint256)[],(address,uint256),uint256,uint256,(address,bytes)[],(address,bytes)[]),address)` and selector `0xad73d689`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct ValidateReturn(pub bool);
}
