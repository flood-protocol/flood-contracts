pub use main_zone::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types
)]
pub mod main_zone {
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
                    ::std::borrow::ToOwned::to_owned("BOOK_ROLE"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("BOOK_ROLE"),
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
                    ::std::borrow::ToOwned::to_owned("CALLER_ROLE"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("CALLER_ROLE"),
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
                    ::std::borrow::ToOwned::to_owned("CANCELLED_ORDERS"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("CANCELLED_ORDERS"),
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
                    ::std::borrow::ToOwned::to_owned("secondaryZone"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("secondaryZone"),
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
                    ::std::borrow::ToOwned::to_owned("setSecondaryZone"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setSecondaryZone"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newSecondaryZone"),
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
                    ::std::borrow::ToOwned::to_owned("validateOrder"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("validateOrder"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
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
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct IFloodPlain.Order"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("book"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("caller"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("orderHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("validateOrder"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
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
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct IFloodPlain.Order"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("book"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("fulfiller"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("caller"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("orderHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("SecondaryZoneSet"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("SecondaryZoneSet"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newSecondayZone"),
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
                    ::std::borrow::ToOwned::to_owned("CancelledOrder"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("CancelledOrder"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("orderHash"),
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
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static MAINZONE_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15b\0\0\x11W`\0\x80\xFD[P`@Qb\0\x1B\x0C8\x03\x80b\0\x1B\x0C\x839\x81\x01`@\x81\x90Rb\0\x004\x91b\0\x023V[b\x02\xA3\0\x81`\x01`\x01`\xA0\x1B\x03\x81\x16b\0\0\x95W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1E`$\x82\x01R\x7FAccessControl: 0 default admin\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\x01\x80T`\x01`\x01`\xD0\x1B\x03\x16`\x01`\xD0\x1Be\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x02\x17\x90Ub\0\0\xC1`\0\x82b\0\0\xD4V[PP`\x03\x80T`\xFF\x19\x16\x90UPb\0\x02eV[\x81b\0\x01xW`\0b\0\0\xEF`\x02T`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x14b\0\x01\\W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`,`$\x82\x01R\x7FAccessControl: default admin alr`D\x82\x01Rk\x19XY\x1EH\x19\xDC\x98[\x9D\x19Y`\xA2\x1B`d\x82\x01R`\x84\x01b\0\0\x8CV[`\x02\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x17\x90U[b\0\x01\x8F\x82\x82b\0\x01\x93` \x1Bb\0\n\xAA\x17` \x1CV[PPV[`\0\x82\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x84R\x90\x91R\x90 T`\xFF\x16b\0\x01\x8FW`\0\x82\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x84R\x90\x91R\x90 \x80T`\xFF\x19\x16`\x01\x17\x90Ub\0\x01\xEF3\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x83\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4PPV[`\0` \x82\x84\x03\x12\x15b\0\x02FW`\0\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x02^W`\0\x80\xFD[\x93\x92PPPV[a\x18\x97\x80b\0\x02u`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01\xCFW`\x005`\xE0\x1C\x80cwB7\xFC\x11a\x01\x04W\x80c\xA7\xAA\x8Bt\x11a\0\xA2W\x80c\xD5Gt\x1F\x11a\0qW\x80c\xD5Gt\x1F\x14a\x04\nW\x80c\xD6\x02\xB9\xFD\x14a\x04\x1DW\x80c\xD8\x9E\x8D\x94\x14a\x04%W\x80c\xDD\x11\xB2\x8A\x14a\x04LW`\0\x80\xFD[\x80c\xA7\xAA\x8Bt\x14a\x03\xB9W\x80c\xCC\x84c\xC8\x14a\x03\xCCW\x80c\xCE\xFC\x14)\x14a\x03\xD4W\x80c\xCFn\xEF\xB7\x14a\x03\xDCW`\0\x80\xFD[\x80c\x8D\xA5\xCB[\x11a\0\xDEW\x80c\x8D\xA5\xCB[\x14a\x03oW\x80c\x91\xD1HT\x14a\x03wW\x80c\xA1\xED\xA5<\x14a\x03\x8AW\x80c\xA2\x17\xFD\xDF\x14a\x03\xB1W`\0\x80\xFD[\x80cwB7\xFC\x14a\x03/W\x80c\x84V\xCBY\x14a\x03VW\x80c\x84\xEF\x8F\xFC\x14a\x03^W`\0\x80\xFD[\x80c6V\x8A\xBE\x11a\x01qW\x80cO\xD31\xC6\x11a\x01KW\x80cO\xD31\xC6\x14a\x02\xD7W\x80c\\\x97Z\xBB\x14a\x02\xFEW\x80ccN\x93\xDA\x14a\x03\tW\x80cd\x9A^\xC7\x14a\x03\x1CW`\0\x80\xFD[\x80c6V\x8A\xBE\x14a\x02\xA9W\x80c?K\xA8:\x14a\x02\xBCW\x80cMm\xCB\\\x14a\x02\xC4W`\0\x80\xFD[\x80c\x0F\x1B.\xA4\x11a\x01\xADW\x80c\x0F\x1B.\xA4\x14a\x02\"W\x80c\x11\xB1\x02\xB8\x14a\x025W\x80c$\x8A\x9C\xA3\x14a\x02eW\x80c//\xF1]\x14a\x02\x96W`\0\x80\xFD[\x80c\x01\xFF\xC9\xA7\x14a\x01\xD4W\x80c\x02-c\xFB\x14a\x01\xFCW\x80c\n\xA6\"\x0B\x14a\x02\x18W[`\0\x80\xFD[a\x01\xE7a\x01\xE26`\x04a\x14\xAEV[a\x04sV[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[b\x06\x97\x80[`@Qe\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x01\xF3V[a\x02 a\x04\x9EV[\0[a\x02 a\x0206`\x04a\x15\x0CV[a\x04\xB4V[`\x03Ta\x02M\x90a\x01\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01\xF3V[a\x02\x88a\x02s6`\x04a\x15rV[`\0\x90\x81R` \x81\x90R`@\x90 `\x01\x01T\x90V[`@Q\x90\x81R` \x01a\x01\xF3V[a\x02 a\x02\xA46`\x04a\x15\x8BV[a\x05nV[a\x02 a\x02\xB76`\x04a\x15\x8BV[a\x05\xE8V[a\x02 a\x06\xD2V[a\x02 a\x02\xD26`\x04a\x15\xB7V[a\x06\xE5V[a\x02\x88\x7F_\xD8E\x82\xB3\x0B\xAC\xE1\xCB\xB5\xCC\x91\xA7[\x8E\xE4\x8A\x0E\x84\xDA\x1Ed\xC2\xD8\x80\xC8\xC8e\xC8\x13DO\x81V[`\x03T`\xFF\x16a\x01\xE7V[a\x02 a\x03\x176`\x04a\x16\x8AV[a\x07\xC7V[a\x02 a\x03*6`\x04a\x16\xA5V[a\x07\xDBV[a\x02\x88\x7F\x84<:\0\xFA\x95Q\n5\xF4%7\x121\xFD?\xE4d.q\x9C\xB4YQ`v=m\x02YKP\x81V[a\x02 a\x07\xEFV[`\x02T`\x01`\x01`\xA0\x1B\x03\x16a\x02MV[a\x02Ma\x08\x02V[a\x01\xE7a\x03\x856`\x04a\x15\x8BV[a\x08\x1BV[a\x03\x92a\x08DV[`@\x80Qe\xFF\xFF\xFF\xFF\xFF\xFF\x93\x84\x16\x81R\x92\x90\x91\x16` \x83\x01R\x01a\x01\xF3V[a\x02\x88`\0\x81V[a\x02 a\x03\xC76`\x04a\x16\x8AV[a\x08\x98V[a\x02\x01a\t\x1CV[a\x02 a\t{V[a\x03\xE4a\t\xF9V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x83Re\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16` \x83\x01R\x01a\x01\xF3V[a\x02 a\x04\x186`\x04a\x15\x8BV[a\n\x1AV[a\x02 a\n\x97V[a\x02\x88\x7FM\xD7\xD69o\x7FeH\xD0\xA2\x0B\x89Cr\x17\x18'\xD2\xD1\x02\x0Bc\xEED\xC1\xC6\xE1\xE9\xB8\xE6\xCA\x96\x81V[a\x02\x88\x7FF\xDB\xE8\xAE\x0C# \xE02\xF7|D\x8D3\xD3\x8B\xAE>X\xC2\xD6\xB5#'W\r\xEBn=\xB7$g\x81V[`\0`\x01`\x01`\xE0\x1B\x03\x19\x82\x16c\x18\xA4\xC3\xC3`\xE1\x1B\x14\x80a\x04\x98WPa\x04\x98\x82a\x0B.V[\x92\x91PPV[`\0a\x04\xA9\x81a\x0BcV[a\x04\xB1a\x0BmV[PV[a\x04\xBCa\x0BzV[a\x04\xE6\x7F\x84<:\0\xFA\x95Q\n5\xF4%7\x121\xFD?\xE4d.q\x9C\xB4YQ`v=m\x02YKP\x83a\x0B\xC0V[a\x05\x10\x7FF\xDB\xE8\xAE\x0C# \xE02\xF7|D\x8D3\xD3\x8B\xAE>X\xC2\xD6\xB5#'W\r\xEBn=\xB7$g\x84a\x0B\xC0V[a\x05:\x7FM\xD7\xD69o\x7FeH\xD0\xA2\x0B\x89Cr\x17\x18'\xD2\xD1\x02\x0Bc\xEED\xC1\xC6\xE1\xE9\xB8\xE6\xCA\x96\x82a\x08\x1BV[\x15a\x05`W`@Qc\xA1\xC0t\xA7`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01[`@Q\x80\x91\x03\x90\xFD[a\x05ha\x0C\x19V[PPPPV[\x81a\x05\xDAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`6`$\x82\x01R\x7FAccessControl: can't directly gr`D\x82\x01Ruant default admin role`P\x1B`d\x82\x01R`\x84\x01a\x05WV[a\x05\xE4\x82\x82a\x0CMV[PPV[\x81\x15\x80\x15a\x06\x03WP`\x02T`\x01`\x01`\xA0\x1B\x03\x82\x81\x16\x91\x16\x14[\x15a\x06\xC8W`\0\x80a\x06\x13a\t\xF9V[\x90\x92P\x90P`\x01`\x01`\xA0\x1B\x03\x82\x16\x15\x80\x15a\x066WPe\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x15\x15[\x80\x15a\x06IWPBe\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x10[a\x06\xB3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FAccessControl: only can renounce`D\x82\x01Rt in two delayed steps`X\x1B`d\x82\x01R`\x84\x01a\x05WV[PP`\x01\x80Te\xFF\xFF\xFF\xFF\xFF\xFF`\xA0\x1B\x19\x16\x90U[a\x05\xE4\x82\x82a\x0CwV[`\0a\x06\xDD\x81a\x0BcV[a\x04\xB1a\x0C\xF1V[a\x06\xEDa\x0BzV[a\x07\x17\x7F\x84<:\0\xFA\x95Q\n5\xF4%7\x121\xFD?\xE4d.q\x9C\xB4YQ`v=m\x02YKP\x85a\x0B\xC0V[a\x07A\x7F_\xD8E\x82\xB3\x0B\xAC\xE1\xCB\xB5\xCC\x91\xA7[\x8E\xE4\x8A\x0E\x84\xDA\x1Ed\xC2\xD8\x80\xC8\xC8e\xC8\x13DO\x86a\x0B\xC0V[a\x07k\x7FF\xDB\xE8\xAE\x0C# \xE02\xF7|D\x8D3\xD3\x8B\xAE>X\xC2\xD6\xB5#'W\r\xEBn=\xB7$g\x87a\x0B\xC0V[a\x07\x95\x7FM\xD7\xD69o\x7FeH\xD0\xA2\x0B\x89Cr\x17\x18'\xD2\xD1\x02\x0Bc\xEED\xC1\xC6\xE1\xE9\xB8\xE6\xCA\x96\x84a\x08\x1BV[\x15a\x07\xB6W`@Qc\xA1\xC0t\xA7`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R`$\x01a\x05WV[a\x07\xBEa\x0C\x19V[PPPPPPPV[`\0a\x07\xD2\x81a\x0BcV[a\x05\xE4\x82a\rCV[`\0a\x07\xE6\x81a\x0BcV[a\x05\xE4\x82a\r\xB6V[`\0a\x07\xFA\x81a\x0BcV[a\x04\xB1a\x0E&V[`\0a\x08\x16`\x02T`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x90P\x90V[`\0\x91\x82R` \x82\x81R`@\x80\x84 `\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16\x84R\x91\x90R\x90 T`\xFF\x16\x90V[`\x02T`\0\x90`\x01`\xD0\x1B\x90\x04e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x80\x15\x15\x80\x15a\x08pWPBe\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x10\x15[a\x08|W`\0\x80a\x08\x90V[`\x02T`\x01`\xA0\x1B\x90\x04e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81[\x91P\x91P\x90\x91V[`\0a\x08\xA3\x81a\x0BcV[`\x01`\x01`\xA0\x1B\x03\x82\x16\x15a\x08\xC9W\x81`\x01`\x01`\xA0\x1B\x03\x16;`\0\x03a\x08\xC9W`\0\x80\xFD[`\x03\x80Ta\x01\0`\x01`\xA8\x1B\x03\x19\x16a\x01\0`\x01`\x01`\xA0\x1B\x03\x85\x16\x90\x81\x02\x91\x90\x91\x17\x90\x91U`@Q\x7F\x80\xCC\xDFx\xA0]\xE7\xE1\x8F5HB\x14\x11\t\xE9\xCF\xC5\xC461_\xF4r\xCA(Q\x1F\xD0w\xB9\x03\x90`\0\x90\xA2PPV[`\x02T`\0\x90`\x01`\xD0\x1B\x90\x04e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x80\x15\x15\x80\x15a\tGWPBe\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x10[a\tbW`\x01T`\x01`\xD0\x1B\x90\x04e\xFF\xFF\xFF\xFF\xFF\xFF\x16a\tuV[`\x02T`\x01`\xA0\x1B\x90\x04e\xFF\xFF\xFF\xFF\xFF\xFF\x16[\x91PP\x90V[`\0a\t\x85a\t\xF9V[P\x90P3`\x01`\x01`\xA0\x1B\x03\x82\x16\x14a\t\xF1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`(`$\x82\x01R\x7FAccessControl: pending admin mus`D\x82\x01Rg\x1D\x08\x18X\xD8\xD9\\\x1D`\xC2\x1B`d\x82\x01R`\x84\x01a\x05WV[a\x04\xB1a\x0EcV[`\x01T`\x01`\x01`\xA0\x1B\x03\x81\x16\x91`\x01`\xA0\x1B\x90\x91\x04e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[\x81a\n\x8DW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`7`$\x82\x01R\x7FAccessControl: can't directly re`D\x82\x01R\x7Fvoke default admin role\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x05WV[a\x05\xE4\x82\x82a\x0F.V[`\0a\n\xA2\x81a\x0BcV[a\x04\xB1a\x0FSV[a\n\xB4\x82\x82a\x08\x1BV[a\x05\xE4W`\0\x82\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x84R\x90\x91R\x90 \x80T`\xFF\x19\x16`\x01\x17\x90Ua\n\xEA3\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x83\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4PPV[`\0`\x01`\x01`\xE0\x1B\x03\x19\x82\x16cye\xDB\x0B`\xE0\x1B\x14\x80a\x04\x98WPc\x01\xFF\xC9\xA7`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x83\x16\x14a\x04\x98V[a\x04\xB1\x813a\x0B\xC0V[a\x0Bx`\0\x80a\x0F^V[V[`\x03T`\xFF\x16\x15a\x0BxW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01Ro\x14\x18]\\\xD8X\x9B\x19N\x88\x1C\x18]\\\xD9Y`\x82\x1B`D\x82\x01R`d\x01a\x05WV[a\x0B\xCA\x82\x82a\x08\x1BV[a\x05\xE4Wa\x0B\xD7\x81a\x10\x1EV[a\x0B\xE2\x83` a\x100V[`@Q` \x01a\x0B\xF3\x92\x91\x90a\x16\xF1V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90RbF\x1B\xCD`\xE5\x1B\x82Ra\x05W\x91`\x04\x01a\x17fV[`\x03Ta\x01\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x80\x15a\x04\xB1W6`\0\x807`\0\x806`\0\x84Z\xFA\x80a\x05\xE4W=`\0\x80>=`\0\xFD[`\0\x82\x81R` \x81\x90R`@\x90 `\x01\x01Ta\x0Ch\x81a\x0BcV[a\x0Cr\x83\x83a\x11\xD3V[PPPV[`\x01`\x01`\xA0\x1B\x03\x81\x163\x14a\x0C\xE7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`/`$\x82\x01R\x7FAccessControl: can only renounce`D\x82\x01Rn\x1097\xB62\xB9\x9037\xB9\x109\xB2\xB63`\x89\x1B`d\x82\x01R`\x84\x01a\x05WV[a\x05\xE4\x82\x82a\x12}V[a\x0C\xF9a\x12\xB8V[`\x03\x80T`\xFF\x19\x16\x90U\x7F]\xB9\xEE\nI[\xF2\xE6\xFF\x9C\x91\xA7\x83L\x1B\xA4\xFD\xD2D\xA5\xE8\xAANS{\xD3\x8A\xEA\xE4\xB0s\xAA3[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01`@Q\x80\x91\x03\x90\xA1V[`\0a\rMa\t\x1CV[a\rVBa\x13\x01V[a\r`\x91\x90a\x17\xAFV[\x90Pa\rl\x82\x82a\x13lV[`@Qe\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x7F3w\xDCD$\x1Ew\x9D\xD0j\xFA\xB5\xB7\x88\xA3\\\xA5\xF3\xB7x\x83n)\x90\xBD\xB2j*K.^\xD6\x90` \x01`@Q\x80\x91\x03\x90\xA2PPV[`\0a\r\xC1\x82a\x13\xEBV[a\r\xCABa\x13\x01V[a\r\xD4\x91\x90a\x17\xAFV[\x90Pa\r\xE0\x82\x82a\x0F^V[`@\x80Qe\xFF\xFF\xFF\xFF\xFF\xFF\x80\x85\x16\x82R\x83\x16` \x82\x01R\x7F\xF1\x03\x8C\x18\xCF\x84\xA5nC/\xDB\xFA\xF7F\x92K~\xA5\x11\xDF\xE0:e\x06\xA0\xCE\xBAH\x88x\x8D\x9B\x91\x01`@Q\x80\x91\x03\x90\xA1PPV[a\x0E.a\x0BzV[`\x03\x80T`\xFF\x19\x16`\x01\x17\x90U\x7Fb\xE7\x8C\xEA\x01\xBE\xE3 \xCDNB\x02p\xB5\xEAt\0\r\x11\xB0\xC9\xF7GT\xEB\xDB\xFCTK\x05\xA2Xa\r&3\x90V[`\0\x80a\x0Ena\t\xF9V[\x91P\x91Pa\x0E\x83\x81e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x15\x15\x90V[\x80\x15a\x0E\x96WPBe\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x10[a\x0E\xF3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`(`$\x82\x01R\x7FAccessControl: transfer delay no`D\x82\x01Rg\x1D\x08\x1C\x18\\\xDC\xD9Y`\xC2\x1B`d\x82\x01R`\x84\x01a\x05WV[a\x0F\x0F`\0a\x0F\n`\x02T`\x01`\x01`\xA0\x1B\x03\x16\x90V[a\x12}V[a\x0F\x1A`\0\x83a\x11\xD3V[PP`\x01\x80T`\x01`\x01`\xD0\x1B\x03\x19\x16\x90UV[`\0\x82\x81R` \x81\x90R`@\x90 `\x01\x01Ta\x0FI\x81a\x0BcV[a\x0Cr\x83\x83a\x12}V[a\x0Bx`\0\x80a\x13lV[`\x02T`\x01`\xD0\x1B\x90\x04e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x80\x15a\x0F\xE1WBe\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x10\x15a\x0F\xB7W`\x02T`\x01\x80T`\x01`\x01`\xD0\x1B\x03\x16`\x01`\xA0\x1B\x90\x92\x04e\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x01`\xD0\x1B\x02\x91\x90\x91\x17\x90Ua\x0F\xE1V[`@Q\x7F+\x1F\xA2\xED\xAF\xE6\xF7\xB9\xE9|\x1A\x9E\x0C6`\xE6E\xBE\xB2\xDC\xAA-E\xBD\xBF\x9B\xEA\xF5G.\x1E\xC5\x90`\0\x90\xA1[P`\x02\x80T`\x01`\x01`\xA0\x1B\x03\x16`\x01`\xA0\x1Be\xFF\xFF\xFF\xFF\xFF\xFF\x94\x85\x16\x02`\x01`\x01`\xD0\x1B\x03\x16\x17`\x01`\xD0\x1B\x92\x90\x93\x16\x91\x90\x91\x02\x91\x90\x91\x17\x90UV[``a\x04\x98`\x01`\x01`\xA0\x1B\x03\x83\x16`\x14[```\0a\x10?\x83`\x02a\x17\xD5V[a\x10J\x90`\x02a\x17\xECV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x10bWa\x10ba\x17\xFFV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x10\x8CW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\x03`\xFC\x1B\x81`\0\x81Q\x81\x10a\x10\xA7Wa\x10\xA7a\x18\x15V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x0F`\xFB\x1B\x81`\x01\x81Q\x81\x10a\x10\xD6Wa\x10\xD6a\x18\x15V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\0a\x10\xFA\x84`\x02a\x17\xD5V[a\x11\x05\x90`\x01a\x17\xECV[\x90P[`\x01\x81\x11\x15a\x11}Wo\x18\x18\x99\x19\x9A\x1A\x9B\x1B\x9C\x1C\xB0\xB11\xB22\xB3`\x81\x1B\x85`\x0F\x16`\x10\x81\x10a\x119Wa\x119a\x18\x15V[\x1A`\xF8\x1B\x82\x82\x81Q\x81\x10a\x11OWa\x11Oa\x18\x15V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x04\x94\x90\x94\x1C\x93a\x11v\x81a\x18+V[\x90Pa\x11\x08V[P\x83\x15a\x11\xCCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FStrings: hex length insufficient`D\x82\x01R`d\x01a\x05WV[\x93\x92PPPV[\x81a\x12sW`\0a\x11\xEC`\x02T`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x12WW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`,`$\x82\x01R\x7FAccessControl: default admin alr`D\x82\x01Rk\x19XY\x1EH\x19\xDC\x98[\x9D\x19Y`\xA2\x1B`d\x82\x01R`\x84\x01a\x05WV[`\x02\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x17\x90U[a\x05\xE4\x82\x82a\n\xAAV[\x81\x15\x80\x15a\x12\x98WP`\x02T`\x01`\x01`\xA0\x1B\x03\x82\x81\x16\x91\x16\x14[\x15a\x12\xAEW`\x02\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x90U[a\x05\xE4\x82\x82a\x143V[`\x03T`\xFF\x16a\x0BxW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rs\x14\x18]\\\xD8X\x9B\x19N\x88\x1B\x9B\xDD\x08\x1C\x18]\\\xD9Y`b\x1B`D\x82\x01R`d\x01a\x05WV[`\0e\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x13hW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FSafeCast: value doesn't fit in 4`D\x82\x01Re8 bits`\xD0\x1B`d\x82\x01R`\x84\x01a\x05WV[P\x90V[`\0a\x13va\t\xF9V[`\x01\x80Te\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16`\x01`\xA0\x1B\x02`\x01`\x01`\xD0\x1B\x03\x19\x90\x91\x16`\x01`\x01`\xA0\x1B\x03\x88\x16\x17\x17\x90U\x91Pa\x13\xB8\x90P\x81e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x15\x15\x90V[\x15a\x0CrW`@Q\x7F\x88\x86\xEB\xFCBY\xAB\xDB\xC1f\x01\xDD\x8F\xB5g\x8ET\x87\x8FG\xB3\xC3H6\xCF\xC5\x11T\xA9`Q\t\x90`\0\x90\xA1PPPV[`\0\x80a\x13\xF6a\t\x1CV[\x90P\x80e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11a\x14\x1EWa\x14\x19\x83\x82a\x18BV[a\x11\xCCV[a\x11\xCCe\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16b\x06\x97\x80a\x14\x98V[a\x14=\x82\x82a\x08\x1BV[\x15a\x05\xE4W`\0\x82\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x80\x85R\x92R\x80\x83 \x80T`\xFF\x19\x16\x90UQ3\x92\x85\x91\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B\x91\x90\xA4PPV[`\0\x81\x83\x10a\x14\xA7W\x81a\x11\xCCV[P\x90\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x14\xC0W`\0\x80\xFD[\x815`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a\x11\xCCW`\0\x80\xFD[`\0`\xC0\x82\x84\x03\x12\x15a\x14\xEAW`\0\x80\xFD[P\x91\x90PV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x15\x07W`\0\x80\xFD[\x91\x90PV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x15\"W`\0\x80\xFD[\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x159W`\0\x80\xFD[a\x15E\x87\x82\x88\x01a\x14\xD8V[\x94PPa\x15T` \x86\x01a\x14\xF0V[\x92Pa\x15b`@\x86\x01a\x14\xF0V[\x93\x96\x92\x95P\x92\x93``\x015\x92PPV[`\0` \x82\x84\x03\x12\x15a\x15\x84W`\0\x80\xFD[P5\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x15\x9EW`\0\x80\xFD[\x825\x91Pa\x15\xAE` \x84\x01a\x14\xF0V[\x90P\x92P\x92\x90PV[`\0\x80`\0\x80`\0\x80`\0`\xC0\x88\x8A\x03\x12\x15a\x15\xD2W`\0\x80\xFD[\x875g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x15\xEAW`\0\x80\xFD[a\x15\xF6\x8B\x83\x8C\x01a\x14\xD8V[\x98Pa\x16\x04` \x8B\x01a\x14\xF0V[\x97Pa\x16\x12`@\x8B\x01a\x14\xF0V[\x96Pa\x16 ``\x8B\x01a\x14\xF0V[\x95P`\x80\x8A\x015\x94P`\xA0\x8A\x015\x91P\x80\x82\x11\x15a\x16=W`\0\x80\xFD[\x81\x8A\x01\x91P\x8A`\x1F\x83\x01\x12a\x16QW`\0\x80\xFD[\x815\x81\x81\x11\x15a\x16`W`\0\x80\xFD[\x8B` \x82\x85\x01\x01\x11\x15a\x16rW`\0\x80\xFD[` \x83\x01\x94P\x80\x93PPPP\x92\x95\x98\x91\x94\x97P\x92\x95PV[`\0` \x82\x84\x03\x12\x15a\x16\x9CW`\0\x80\xFD[a\x11\xCC\x82a\x14\xF0V[`\0` \x82\x84\x03\x12\x15a\x16\xB7W`\0\x80\xFD[\x815e\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x11\xCCW`\0\x80\xFD[`\0[\x83\x81\x10\x15a\x16\xE8W\x81\x81\x01Q\x83\x82\x01R` \x01a\x16\xD0V[PP`\0\x91\x01RV[\x7FAccessControl: account \0\0\0\0\0\0\0\0\0\x81R`\0\x83Qa\x17)\x81`\x17\x85\x01` \x88\x01a\x16\xCDV[p\x01\x03K\x99\x03kK\x9B\x9BKs9\x03\x93{c)`}\x1B`\x17\x91\x84\x01\x91\x82\x01R\x83Qa\x17Z\x81`(\x84\x01` \x88\x01a\x16\xCDV[\x01`(\x01\x94\x93PPPPV[` \x81R`\0\x82Q\x80` \x84\x01Ra\x17\x85\x81`@\x85\x01` \x87\x01a\x16\xCDV[`\x1F\x01`\x1F\x19\x16\x91\x90\x91\x01`@\x01\x92\x91PPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[e\xFF\xFF\xFF\xFF\xFF\xFF\x81\x81\x16\x83\x82\x16\x01\x90\x80\x82\x11\x15a\x17\xCEWa\x17\xCEa\x17\x99V[P\x92\x91PPV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x04\x98Wa\x04\x98a\x17\x99V[\x80\x82\x01\x80\x82\x11\x15a\x04\x98Wa\x04\x98a\x17\x99V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0\x81a\x18:Wa\x18:a\x17\x99V[P`\0\x19\x01\x90V[e\xFF\xFF\xFF\xFF\xFF\xFF\x82\x81\x16\x82\x82\x16\x03\x90\x80\x82\x11\x15a\x17\xCEWa\x17\xCEa\x17\x99V\xFE\xA2dipfsX\"\x12 \xD1g\xE4\xD2F7\"3\xCC\xD7O\xA2\xDEZ/\x8Fd2%G\xAB#\x10\xD6\xF5m\xB7\t\xB1\xCErddsolcC\0\x08\x11\x003";
    /// The bytecode of the contract.
    pub static MAINZONE_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01\xCFW`\x005`\xE0\x1C\x80cwB7\xFC\x11a\x01\x04W\x80c\xA7\xAA\x8Bt\x11a\0\xA2W\x80c\xD5Gt\x1F\x11a\0qW\x80c\xD5Gt\x1F\x14a\x04\nW\x80c\xD6\x02\xB9\xFD\x14a\x04\x1DW\x80c\xD8\x9E\x8D\x94\x14a\x04%W\x80c\xDD\x11\xB2\x8A\x14a\x04LW`\0\x80\xFD[\x80c\xA7\xAA\x8Bt\x14a\x03\xB9W\x80c\xCC\x84c\xC8\x14a\x03\xCCW\x80c\xCE\xFC\x14)\x14a\x03\xD4W\x80c\xCFn\xEF\xB7\x14a\x03\xDCW`\0\x80\xFD[\x80c\x8D\xA5\xCB[\x11a\0\xDEW\x80c\x8D\xA5\xCB[\x14a\x03oW\x80c\x91\xD1HT\x14a\x03wW\x80c\xA1\xED\xA5<\x14a\x03\x8AW\x80c\xA2\x17\xFD\xDF\x14a\x03\xB1W`\0\x80\xFD[\x80cwB7\xFC\x14a\x03/W\x80c\x84V\xCBY\x14a\x03VW\x80c\x84\xEF\x8F\xFC\x14a\x03^W`\0\x80\xFD[\x80c6V\x8A\xBE\x11a\x01qW\x80cO\xD31\xC6\x11a\x01KW\x80cO\xD31\xC6\x14a\x02\xD7W\x80c\\\x97Z\xBB\x14a\x02\xFEW\x80ccN\x93\xDA\x14a\x03\tW\x80cd\x9A^\xC7\x14a\x03\x1CW`\0\x80\xFD[\x80c6V\x8A\xBE\x14a\x02\xA9W\x80c?K\xA8:\x14a\x02\xBCW\x80cMm\xCB\\\x14a\x02\xC4W`\0\x80\xFD[\x80c\x0F\x1B.\xA4\x11a\x01\xADW\x80c\x0F\x1B.\xA4\x14a\x02\"W\x80c\x11\xB1\x02\xB8\x14a\x025W\x80c$\x8A\x9C\xA3\x14a\x02eW\x80c//\xF1]\x14a\x02\x96W`\0\x80\xFD[\x80c\x01\xFF\xC9\xA7\x14a\x01\xD4W\x80c\x02-c\xFB\x14a\x01\xFCW\x80c\n\xA6\"\x0B\x14a\x02\x18W[`\0\x80\xFD[a\x01\xE7a\x01\xE26`\x04a\x14\xAEV[a\x04sV[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[b\x06\x97\x80[`@Qe\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x01\xF3V[a\x02 a\x04\x9EV[\0[a\x02 a\x0206`\x04a\x15\x0CV[a\x04\xB4V[`\x03Ta\x02M\x90a\x01\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01\xF3V[a\x02\x88a\x02s6`\x04a\x15rV[`\0\x90\x81R` \x81\x90R`@\x90 `\x01\x01T\x90V[`@Q\x90\x81R` \x01a\x01\xF3V[a\x02 a\x02\xA46`\x04a\x15\x8BV[a\x05nV[a\x02 a\x02\xB76`\x04a\x15\x8BV[a\x05\xE8V[a\x02 a\x06\xD2V[a\x02 a\x02\xD26`\x04a\x15\xB7V[a\x06\xE5V[a\x02\x88\x7F_\xD8E\x82\xB3\x0B\xAC\xE1\xCB\xB5\xCC\x91\xA7[\x8E\xE4\x8A\x0E\x84\xDA\x1Ed\xC2\xD8\x80\xC8\xC8e\xC8\x13DO\x81V[`\x03T`\xFF\x16a\x01\xE7V[a\x02 a\x03\x176`\x04a\x16\x8AV[a\x07\xC7V[a\x02 a\x03*6`\x04a\x16\xA5V[a\x07\xDBV[a\x02\x88\x7F\x84<:\0\xFA\x95Q\n5\xF4%7\x121\xFD?\xE4d.q\x9C\xB4YQ`v=m\x02YKP\x81V[a\x02 a\x07\xEFV[`\x02T`\x01`\x01`\xA0\x1B\x03\x16a\x02MV[a\x02Ma\x08\x02V[a\x01\xE7a\x03\x856`\x04a\x15\x8BV[a\x08\x1BV[a\x03\x92a\x08DV[`@\x80Qe\xFF\xFF\xFF\xFF\xFF\xFF\x93\x84\x16\x81R\x92\x90\x91\x16` \x83\x01R\x01a\x01\xF3V[a\x02\x88`\0\x81V[a\x02 a\x03\xC76`\x04a\x16\x8AV[a\x08\x98V[a\x02\x01a\t\x1CV[a\x02 a\t{V[a\x03\xE4a\t\xF9V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x83Re\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16` \x83\x01R\x01a\x01\xF3V[a\x02 a\x04\x186`\x04a\x15\x8BV[a\n\x1AV[a\x02 a\n\x97V[a\x02\x88\x7FM\xD7\xD69o\x7FeH\xD0\xA2\x0B\x89Cr\x17\x18'\xD2\xD1\x02\x0Bc\xEED\xC1\xC6\xE1\xE9\xB8\xE6\xCA\x96\x81V[a\x02\x88\x7FF\xDB\xE8\xAE\x0C# \xE02\xF7|D\x8D3\xD3\x8B\xAE>X\xC2\xD6\xB5#'W\r\xEBn=\xB7$g\x81V[`\0`\x01`\x01`\xE0\x1B\x03\x19\x82\x16c\x18\xA4\xC3\xC3`\xE1\x1B\x14\x80a\x04\x98WPa\x04\x98\x82a\x0B.V[\x92\x91PPV[`\0a\x04\xA9\x81a\x0BcV[a\x04\xB1a\x0BmV[PV[a\x04\xBCa\x0BzV[a\x04\xE6\x7F\x84<:\0\xFA\x95Q\n5\xF4%7\x121\xFD?\xE4d.q\x9C\xB4YQ`v=m\x02YKP\x83a\x0B\xC0V[a\x05\x10\x7FF\xDB\xE8\xAE\x0C# \xE02\xF7|D\x8D3\xD3\x8B\xAE>X\xC2\xD6\xB5#'W\r\xEBn=\xB7$g\x84a\x0B\xC0V[a\x05:\x7FM\xD7\xD69o\x7FeH\xD0\xA2\x0B\x89Cr\x17\x18'\xD2\xD1\x02\x0Bc\xEED\xC1\xC6\xE1\xE9\xB8\xE6\xCA\x96\x82a\x08\x1BV[\x15a\x05`W`@Qc\xA1\xC0t\xA7`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01[`@Q\x80\x91\x03\x90\xFD[a\x05ha\x0C\x19V[PPPPV[\x81a\x05\xDAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`6`$\x82\x01R\x7FAccessControl: can't directly gr`D\x82\x01Ruant default admin role`P\x1B`d\x82\x01R`\x84\x01a\x05WV[a\x05\xE4\x82\x82a\x0CMV[PPV[\x81\x15\x80\x15a\x06\x03WP`\x02T`\x01`\x01`\xA0\x1B\x03\x82\x81\x16\x91\x16\x14[\x15a\x06\xC8W`\0\x80a\x06\x13a\t\xF9V[\x90\x92P\x90P`\x01`\x01`\xA0\x1B\x03\x82\x16\x15\x80\x15a\x066WPe\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x15\x15[\x80\x15a\x06IWPBe\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x10[a\x06\xB3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FAccessControl: only can renounce`D\x82\x01Rt in two delayed steps`X\x1B`d\x82\x01R`\x84\x01a\x05WV[PP`\x01\x80Te\xFF\xFF\xFF\xFF\xFF\xFF`\xA0\x1B\x19\x16\x90U[a\x05\xE4\x82\x82a\x0CwV[`\0a\x06\xDD\x81a\x0BcV[a\x04\xB1a\x0C\xF1V[a\x06\xEDa\x0BzV[a\x07\x17\x7F\x84<:\0\xFA\x95Q\n5\xF4%7\x121\xFD?\xE4d.q\x9C\xB4YQ`v=m\x02YKP\x85a\x0B\xC0V[a\x07A\x7F_\xD8E\x82\xB3\x0B\xAC\xE1\xCB\xB5\xCC\x91\xA7[\x8E\xE4\x8A\x0E\x84\xDA\x1Ed\xC2\xD8\x80\xC8\xC8e\xC8\x13DO\x86a\x0B\xC0V[a\x07k\x7FF\xDB\xE8\xAE\x0C# \xE02\xF7|D\x8D3\xD3\x8B\xAE>X\xC2\xD6\xB5#'W\r\xEBn=\xB7$g\x87a\x0B\xC0V[a\x07\x95\x7FM\xD7\xD69o\x7FeH\xD0\xA2\x0B\x89Cr\x17\x18'\xD2\xD1\x02\x0Bc\xEED\xC1\xC6\xE1\xE9\xB8\xE6\xCA\x96\x84a\x08\x1BV[\x15a\x07\xB6W`@Qc\xA1\xC0t\xA7`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R`$\x01a\x05WV[a\x07\xBEa\x0C\x19V[PPPPPPPV[`\0a\x07\xD2\x81a\x0BcV[a\x05\xE4\x82a\rCV[`\0a\x07\xE6\x81a\x0BcV[a\x05\xE4\x82a\r\xB6V[`\0a\x07\xFA\x81a\x0BcV[a\x04\xB1a\x0E&V[`\0a\x08\x16`\x02T`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x90P\x90V[`\0\x91\x82R` \x82\x81R`@\x80\x84 `\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16\x84R\x91\x90R\x90 T`\xFF\x16\x90V[`\x02T`\0\x90`\x01`\xD0\x1B\x90\x04e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x80\x15\x15\x80\x15a\x08pWPBe\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x10\x15[a\x08|W`\0\x80a\x08\x90V[`\x02T`\x01`\xA0\x1B\x90\x04e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81[\x91P\x91P\x90\x91V[`\0a\x08\xA3\x81a\x0BcV[`\x01`\x01`\xA0\x1B\x03\x82\x16\x15a\x08\xC9W\x81`\x01`\x01`\xA0\x1B\x03\x16;`\0\x03a\x08\xC9W`\0\x80\xFD[`\x03\x80Ta\x01\0`\x01`\xA8\x1B\x03\x19\x16a\x01\0`\x01`\x01`\xA0\x1B\x03\x85\x16\x90\x81\x02\x91\x90\x91\x17\x90\x91U`@Q\x7F\x80\xCC\xDFx\xA0]\xE7\xE1\x8F5HB\x14\x11\t\xE9\xCF\xC5\xC461_\xF4r\xCA(Q\x1F\xD0w\xB9\x03\x90`\0\x90\xA2PPV[`\x02T`\0\x90`\x01`\xD0\x1B\x90\x04e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x80\x15\x15\x80\x15a\tGWPBe\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x10[a\tbW`\x01T`\x01`\xD0\x1B\x90\x04e\xFF\xFF\xFF\xFF\xFF\xFF\x16a\tuV[`\x02T`\x01`\xA0\x1B\x90\x04e\xFF\xFF\xFF\xFF\xFF\xFF\x16[\x91PP\x90V[`\0a\t\x85a\t\xF9V[P\x90P3`\x01`\x01`\xA0\x1B\x03\x82\x16\x14a\t\xF1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`(`$\x82\x01R\x7FAccessControl: pending admin mus`D\x82\x01Rg\x1D\x08\x18X\xD8\xD9\\\x1D`\xC2\x1B`d\x82\x01R`\x84\x01a\x05WV[a\x04\xB1a\x0EcV[`\x01T`\x01`\x01`\xA0\x1B\x03\x81\x16\x91`\x01`\xA0\x1B\x90\x91\x04e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[\x81a\n\x8DW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`7`$\x82\x01R\x7FAccessControl: can't directly re`D\x82\x01R\x7Fvoke default admin role\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x05WV[a\x05\xE4\x82\x82a\x0F.V[`\0a\n\xA2\x81a\x0BcV[a\x04\xB1a\x0FSV[a\n\xB4\x82\x82a\x08\x1BV[a\x05\xE4W`\0\x82\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x84R\x90\x91R\x90 \x80T`\xFF\x19\x16`\x01\x17\x90Ua\n\xEA3\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x83\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4PPV[`\0`\x01`\x01`\xE0\x1B\x03\x19\x82\x16cye\xDB\x0B`\xE0\x1B\x14\x80a\x04\x98WPc\x01\xFF\xC9\xA7`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x83\x16\x14a\x04\x98V[a\x04\xB1\x813a\x0B\xC0V[a\x0Bx`\0\x80a\x0F^V[V[`\x03T`\xFF\x16\x15a\x0BxW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01Ro\x14\x18]\\\xD8X\x9B\x19N\x88\x1C\x18]\\\xD9Y`\x82\x1B`D\x82\x01R`d\x01a\x05WV[a\x0B\xCA\x82\x82a\x08\x1BV[a\x05\xE4Wa\x0B\xD7\x81a\x10\x1EV[a\x0B\xE2\x83` a\x100V[`@Q` \x01a\x0B\xF3\x92\x91\x90a\x16\xF1V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90RbF\x1B\xCD`\xE5\x1B\x82Ra\x05W\x91`\x04\x01a\x17fV[`\x03Ta\x01\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x80\x15a\x04\xB1W6`\0\x807`\0\x806`\0\x84Z\xFA\x80a\x05\xE4W=`\0\x80>=`\0\xFD[`\0\x82\x81R` \x81\x90R`@\x90 `\x01\x01Ta\x0Ch\x81a\x0BcV[a\x0Cr\x83\x83a\x11\xD3V[PPPV[`\x01`\x01`\xA0\x1B\x03\x81\x163\x14a\x0C\xE7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`/`$\x82\x01R\x7FAccessControl: can only renounce`D\x82\x01Rn\x1097\xB62\xB9\x9037\xB9\x109\xB2\xB63`\x89\x1B`d\x82\x01R`\x84\x01a\x05WV[a\x05\xE4\x82\x82a\x12}V[a\x0C\xF9a\x12\xB8V[`\x03\x80T`\xFF\x19\x16\x90U\x7F]\xB9\xEE\nI[\xF2\xE6\xFF\x9C\x91\xA7\x83L\x1B\xA4\xFD\xD2D\xA5\xE8\xAANS{\xD3\x8A\xEA\xE4\xB0s\xAA3[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01`@Q\x80\x91\x03\x90\xA1V[`\0a\rMa\t\x1CV[a\rVBa\x13\x01V[a\r`\x91\x90a\x17\xAFV[\x90Pa\rl\x82\x82a\x13lV[`@Qe\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x7F3w\xDCD$\x1Ew\x9D\xD0j\xFA\xB5\xB7\x88\xA3\\\xA5\xF3\xB7x\x83n)\x90\xBD\xB2j*K.^\xD6\x90` \x01`@Q\x80\x91\x03\x90\xA2PPV[`\0a\r\xC1\x82a\x13\xEBV[a\r\xCABa\x13\x01V[a\r\xD4\x91\x90a\x17\xAFV[\x90Pa\r\xE0\x82\x82a\x0F^V[`@\x80Qe\xFF\xFF\xFF\xFF\xFF\xFF\x80\x85\x16\x82R\x83\x16` \x82\x01R\x7F\xF1\x03\x8C\x18\xCF\x84\xA5nC/\xDB\xFA\xF7F\x92K~\xA5\x11\xDF\xE0:e\x06\xA0\xCE\xBAH\x88x\x8D\x9B\x91\x01`@Q\x80\x91\x03\x90\xA1PPV[a\x0E.a\x0BzV[`\x03\x80T`\xFF\x19\x16`\x01\x17\x90U\x7Fb\xE7\x8C\xEA\x01\xBE\xE3 \xCDNB\x02p\xB5\xEAt\0\r\x11\xB0\xC9\xF7GT\xEB\xDB\xFCTK\x05\xA2Xa\r&3\x90V[`\0\x80a\x0Ena\t\xF9V[\x91P\x91Pa\x0E\x83\x81e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x15\x15\x90V[\x80\x15a\x0E\x96WPBe\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x10[a\x0E\xF3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`(`$\x82\x01R\x7FAccessControl: transfer delay no`D\x82\x01Rg\x1D\x08\x1C\x18\\\xDC\xD9Y`\xC2\x1B`d\x82\x01R`\x84\x01a\x05WV[a\x0F\x0F`\0a\x0F\n`\x02T`\x01`\x01`\xA0\x1B\x03\x16\x90V[a\x12}V[a\x0F\x1A`\0\x83a\x11\xD3V[PP`\x01\x80T`\x01`\x01`\xD0\x1B\x03\x19\x16\x90UV[`\0\x82\x81R` \x81\x90R`@\x90 `\x01\x01Ta\x0FI\x81a\x0BcV[a\x0Cr\x83\x83a\x12}V[a\x0Bx`\0\x80a\x13lV[`\x02T`\x01`\xD0\x1B\x90\x04e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x80\x15a\x0F\xE1WBe\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x10\x15a\x0F\xB7W`\x02T`\x01\x80T`\x01`\x01`\xD0\x1B\x03\x16`\x01`\xA0\x1B\x90\x92\x04e\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x01`\xD0\x1B\x02\x91\x90\x91\x17\x90Ua\x0F\xE1V[`@Q\x7F+\x1F\xA2\xED\xAF\xE6\xF7\xB9\xE9|\x1A\x9E\x0C6`\xE6E\xBE\xB2\xDC\xAA-E\xBD\xBF\x9B\xEA\xF5G.\x1E\xC5\x90`\0\x90\xA1[P`\x02\x80T`\x01`\x01`\xA0\x1B\x03\x16`\x01`\xA0\x1Be\xFF\xFF\xFF\xFF\xFF\xFF\x94\x85\x16\x02`\x01`\x01`\xD0\x1B\x03\x16\x17`\x01`\xD0\x1B\x92\x90\x93\x16\x91\x90\x91\x02\x91\x90\x91\x17\x90UV[``a\x04\x98`\x01`\x01`\xA0\x1B\x03\x83\x16`\x14[```\0a\x10?\x83`\x02a\x17\xD5V[a\x10J\x90`\x02a\x17\xECV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x10bWa\x10ba\x17\xFFV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x10\x8CW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\x03`\xFC\x1B\x81`\0\x81Q\x81\x10a\x10\xA7Wa\x10\xA7a\x18\x15V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x0F`\xFB\x1B\x81`\x01\x81Q\x81\x10a\x10\xD6Wa\x10\xD6a\x18\x15V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\0a\x10\xFA\x84`\x02a\x17\xD5V[a\x11\x05\x90`\x01a\x17\xECV[\x90P[`\x01\x81\x11\x15a\x11}Wo\x18\x18\x99\x19\x9A\x1A\x9B\x1B\x9C\x1C\xB0\xB11\xB22\xB3`\x81\x1B\x85`\x0F\x16`\x10\x81\x10a\x119Wa\x119a\x18\x15V[\x1A`\xF8\x1B\x82\x82\x81Q\x81\x10a\x11OWa\x11Oa\x18\x15V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x04\x94\x90\x94\x1C\x93a\x11v\x81a\x18+V[\x90Pa\x11\x08V[P\x83\x15a\x11\xCCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FStrings: hex length insufficient`D\x82\x01R`d\x01a\x05WV[\x93\x92PPPV[\x81a\x12sW`\0a\x11\xEC`\x02T`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x12WW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`,`$\x82\x01R\x7FAccessControl: default admin alr`D\x82\x01Rk\x19XY\x1EH\x19\xDC\x98[\x9D\x19Y`\xA2\x1B`d\x82\x01R`\x84\x01a\x05WV[`\x02\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x17\x90U[a\x05\xE4\x82\x82a\n\xAAV[\x81\x15\x80\x15a\x12\x98WP`\x02T`\x01`\x01`\xA0\x1B\x03\x82\x81\x16\x91\x16\x14[\x15a\x12\xAEW`\x02\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x90U[a\x05\xE4\x82\x82a\x143V[`\x03T`\xFF\x16a\x0BxW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rs\x14\x18]\\\xD8X\x9B\x19N\x88\x1B\x9B\xDD\x08\x1C\x18]\\\xD9Y`b\x1B`D\x82\x01R`d\x01a\x05WV[`\0e\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x13hW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FSafeCast: value doesn't fit in 4`D\x82\x01Re8 bits`\xD0\x1B`d\x82\x01R`\x84\x01a\x05WV[P\x90V[`\0a\x13va\t\xF9V[`\x01\x80Te\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16`\x01`\xA0\x1B\x02`\x01`\x01`\xD0\x1B\x03\x19\x90\x91\x16`\x01`\x01`\xA0\x1B\x03\x88\x16\x17\x17\x90U\x91Pa\x13\xB8\x90P\x81e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x15\x15\x90V[\x15a\x0CrW`@Q\x7F\x88\x86\xEB\xFCBY\xAB\xDB\xC1f\x01\xDD\x8F\xB5g\x8ET\x87\x8FG\xB3\xC3H6\xCF\xC5\x11T\xA9`Q\t\x90`\0\x90\xA1PPPV[`\0\x80a\x13\xF6a\t\x1CV[\x90P\x80e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11a\x14\x1EWa\x14\x19\x83\x82a\x18BV[a\x11\xCCV[a\x11\xCCe\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16b\x06\x97\x80a\x14\x98V[a\x14=\x82\x82a\x08\x1BV[\x15a\x05\xE4W`\0\x82\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x80\x85R\x92R\x80\x83 \x80T`\xFF\x19\x16\x90UQ3\x92\x85\x91\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B\x91\x90\xA4PPV[`\0\x81\x83\x10a\x14\xA7W\x81a\x11\xCCV[P\x90\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x14\xC0W`\0\x80\xFD[\x815`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a\x11\xCCW`\0\x80\xFD[`\0`\xC0\x82\x84\x03\x12\x15a\x14\xEAW`\0\x80\xFD[P\x91\x90PV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x15\x07W`\0\x80\xFD[\x91\x90PV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x15\"W`\0\x80\xFD[\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x159W`\0\x80\xFD[a\x15E\x87\x82\x88\x01a\x14\xD8V[\x94PPa\x15T` \x86\x01a\x14\xF0V[\x92Pa\x15b`@\x86\x01a\x14\xF0V[\x93\x96\x92\x95P\x92\x93``\x015\x92PPV[`\0` \x82\x84\x03\x12\x15a\x15\x84W`\0\x80\xFD[P5\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x15\x9EW`\0\x80\xFD[\x825\x91Pa\x15\xAE` \x84\x01a\x14\xF0V[\x90P\x92P\x92\x90PV[`\0\x80`\0\x80`\0\x80`\0`\xC0\x88\x8A\x03\x12\x15a\x15\xD2W`\0\x80\xFD[\x875g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x15\xEAW`\0\x80\xFD[a\x15\xF6\x8B\x83\x8C\x01a\x14\xD8V[\x98Pa\x16\x04` \x8B\x01a\x14\xF0V[\x97Pa\x16\x12`@\x8B\x01a\x14\xF0V[\x96Pa\x16 ``\x8B\x01a\x14\xF0V[\x95P`\x80\x8A\x015\x94P`\xA0\x8A\x015\x91P\x80\x82\x11\x15a\x16=W`\0\x80\xFD[\x81\x8A\x01\x91P\x8A`\x1F\x83\x01\x12a\x16QW`\0\x80\xFD[\x815\x81\x81\x11\x15a\x16`W`\0\x80\xFD[\x8B` \x82\x85\x01\x01\x11\x15a\x16rW`\0\x80\xFD[` \x83\x01\x94P\x80\x93PPPP\x92\x95\x98\x91\x94\x97P\x92\x95PV[`\0` \x82\x84\x03\x12\x15a\x16\x9CW`\0\x80\xFD[a\x11\xCC\x82a\x14\xF0V[`\0` \x82\x84\x03\x12\x15a\x16\xB7W`\0\x80\xFD[\x815e\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x11\xCCW`\0\x80\xFD[`\0[\x83\x81\x10\x15a\x16\xE8W\x81\x81\x01Q\x83\x82\x01R` \x01a\x16\xD0V[PP`\0\x91\x01RV[\x7FAccessControl: account \0\0\0\0\0\0\0\0\0\x81R`\0\x83Qa\x17)\x81`\x17\x85\x01` \x88\x01a\x16\xCDV[p\x01\x03K\x99\x03kK\x9B\x9BKs9\x03\x93{c)`}\x1B`\x17\x91\x84\x01\x91\x82\x01R\x83Qa\x17Z\x81`(\x84\x01` \x88\x01a\x16\xCDV[\x01`(\x01\x94\x93PPPPV[` \x81R`\0\x82Q\x80` \x84\x01Ra\x17\x85\x81`@\x85\x01` \x87\x01a\x16\xCDV[`\x1F\x01`\x1F\x19\x16\x91\x90\x91\x01`@\x01\x92\x91PPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[e\xFF\xFF\xFF\xFF\xFF\xFF\x81\x81\x16\x83\x82\x16\x01\x90\x80\x82\x11\x15a\x17\xCEWa\x17\xCEa\x17\x99V[P\x92\x91PPV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x04\x98Wa\x04\x98a\x17\x99V[\x80\x82\x01\x80\x82\x11\x15a\x04\x98Wa\x04\x98a\x17\x99V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0\x81a\x18:Wa\x18:a\x17\x99V[P`\0\x19\x01\x90V[e\xFF\xFF\xFF\xFF\xFF\xFF\x82\x81\x16\x82\x82\x16\x03\x90\x80\x82\x11\x15a\x17\xCEWa\x17\xCEa\x17\x99V\xFE\xA2dipfsX\"\x12 \xD1g\xE4\xD2F7\"3\xCC\xD7O\xA2\xDEZ/\x8Fd2%G\xAB#\x10\xD6\xF5m\xB7\t\xB1\xCErddsolcC\0\x08\x11\x003";
    /// The deployed bytecode of the contract.
    pub static MAINZONE_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct MainZone<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for MainZone<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for MainZone<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for MainZone<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for MainZone<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(MainZone))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> MainZone<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                MAINZONE_ABI.clone(),
                client,
            ))
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
                MAINZONE_ABI.clone(),
                MAINZONE_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `BOOK_ROLE` (0xdd11b28a) function
        pub fn book_role(&self) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([221, 17, 178, 138], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `CALLER_ROLE` (0x774237fc) function
        pub fn caller_role(&self) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([119, 66, 55, 252], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `CANCELLED_ORDERS` (0xd89e8d94) function
        pub fn cancelled_orders(&self) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([216, 158, 141, 148], ())
                .expect("method not found (this should never happen)")
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
        pub fn fulfiller_role(&self) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
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
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([132, 239, 143, 252], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `defaultAdminDelay` (0xcc8463c8) function
        pub fn default_admin_delay(&self) -> ::ethers::contract::builders::ContractCall<M, u64> {
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
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
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
        ) -> ::ethers::contract::builders::ContractCall<M, (::ethers::core::types::Address, u64)>
        {
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
        ///Calls the contract's `secondaryZone` (0x11b102b8) function
        pub fn secondary_zone(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([17, 177, 2, 184], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setSecondaryZone` (0xa7aa8b74) function
        pub fn set_secondary_zone(
            &self,
            new_secondary_zone: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([167, 170, 139, 116], new_secondary_zone)
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
        ///Calls the contract's `validateOrder` (0x0f1b2ea4) function
        pub fn validate_order_0(
            &self,
            p0: Order,
            book: ::ethers::core::types::Address,
            caller: ::ethers::core::types::Address,
            order_hash: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([15, 27, 46, 164], (p0, book, caller, order_hash))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `validateOrder` (0x4d6dcb5c) function
        pub fn validate_order_1(
            &self,
            p0: Order,
            book: ::ethers::core::types::Address,
            fulfiller: ::ethers::core::types::Address,
            caller: ::ethers::core::types::Address,
            order_hash: [u8; 32],
            p5: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [77, 109, 203, 92],
                    (p0, book, fulfiller, caller, order_hash, p5),
                )
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
            let mut event = self.0.event();
            event.filter = event.filter.address(self.address());

            event
        }
        ///Gets the contract's `DefaultAdminDelayChangeScheduled` event
        pub fn default_admin_delay_change_scheduled_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            DefaultAdminDelayChangeScheduledFilter,
        > {
            let mut event = self.0.event();
            event.filter = event.filter.address(self.address());

            event
        }
        ///Gets the contract's `DefaultAdminTransferCanceled` event
        pub fn default_admin_transfer_canceled_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            DefaultAdminTransferCanceledFilter,
        > {
            let mut event = self.0.event();
            event.filter = event.filter.address(self.address());

            event
        }
        ///Gets the contract's `DefaultAdminTransferScheduled` event
        pub fn default_admin_transfer_scheduled_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            DefaultAdminTransferScheduledFilter,
        > {
            let mut event = self.0.event();
            event.filter = event.filter.address(self.address());

            event
        }
        ///Gets the contract's `Paused` event
        pub fn paused_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, PausedFilter> {
            let mut event = self.0.event();
            event.filter = event.filter.address(self.address());

            event
        }
        ///Gets the contract's `RoleAdminChanged` event
        pub fn role_admin_changed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, RoleAdminChangedFilter>
        {
            let mut event = self.0.event();
            event.filter = event.filter.address(self.address());

            event
        }
        ///Gets the contract's `RoleGranted` event
        pub fn role_granted_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, RoleGrantedFilter>
        {
            let mut event = self.0.event();
            event.filter = event.filter.address(self.address());

            event
        }
        ///Gets the contract's `RoleRevoked` event
        pub fn role_revoked_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, RoleRevokedFilter>
        {
            let mut event = self.0.event();
            event.filter = event.filter.address(self.address());

            event
        }
        ///Gets the contract's `SecondaryZoneSet` event
        pub fn secondary_zone_set_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, SecondaryZoneSetFilter>
        {
            let mut event = self.0.event();
            event.filter = event.filter.address(self.address());

            event
        }
        ///Gets the contract's `Unpaused` event
        pub fn unpaused_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, UnpausedFilter> {
            let mut event = self.0.event();
            event.filter = event.filter.address(self.address());

            event
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, MainZoneEvents> {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>> for MainZone<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `CancelledOrder` with signature `CancelledOrder(bytes32)` and selector `0xa1c074a7`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "CancelledOrder", abi = "CancelledOrder(bytes32)")]
    pub struct CancelledOrder {
        pub order_hash: [u8; 32],
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
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
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
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
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
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
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
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
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "Paused", abi = "Paused(address)")]
    pub struct PausedFilter {
        pub account: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
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
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
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
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
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
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "SecondaryZoneSet", abi = "SecondaryZoneSet(address)")]
    pub struct SecondaryZoneSetFilter {
        #[ethevent(indexed)]
        pub new_seconday_zone: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "Unpaused", abi = "Unpaused(address)")]
    pub struct UnpausedFilter {
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum MainZoneEvents {
        DefaultAdminDelayChangeCanceledFilter(DefaultAdminDelayChangeCanceledFilter),
        DefaultAdminDelayChangeScheduledFilter(DefaultAdminDelayChangeScheduledFilter),
        DefaultAdminTransferCanceledFilter(DefaultAdminTransferCanceledFilter),
        DefaultAdminTransferScheduledFilter(DefaultAdminTransferScheduledFilter),
        PausedFilter(PausedFilter),
        RoleAdminChangedFilter(RoleAdminChangedFilter),
        RoleGrantedFilter(RoleGrantedFilter),
        RoleRevokedFilter(RoleRevokedFilter),
        SecondaryZoneSetFilter(SecondaryZoneSetFilter),
        UnpausedFilter(UnpausedFilter),
    }
    impl ::ethers::contract::EthLogDecode for MainZoneEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = DefaultAdminDelayChangeCanceledFilter::decode_log(log) {
                return Ok(MainZoneEvents::DefaultAdminDelayChangeCanceledFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = DefaultAdminDelayChangeScheduledFilter::decode_log(log) {
                return Ok(MainZoneEvents::DefaultAdminDelayChangeScheduledFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = DefaultAdminTransferCanceledFilter::decode_log(log) {
                return Ok(MainZoneEvents::DefaultAdminTransferCanceledFilter(decoded));
            }
            if let Ok(decoded) = DefaultAdminTransferScheduledFilter::decode_log(log) {
                return Ok(MainZoneEvents::DefaultAdminTransferScheduledFilter(decoded));
            }
            if let Ok(decoded) = PausedFilter::decode_log(log) {
                return Ok(MainZoneEvents::PausedFilter(decoded));
            }
            if let Ok(decoded) = RoleAdminChangedFilter::decode_log(log) {
                return Ok(MainZoneEvents::RoleAdminChangedFilter(decoded));
            }
            if let Ok(decoded) = RoleGrantedFilter::decode_log(log) {
                return Ok(MainZoneEvents::RoleGrantedFilter(decoded));
            }
            if let Ok(decoded) = RoleRevokedFilter::decode_log(log) {
                return Ok(MainZoneEvents::RoleRevokedFilter(decoded));
            }
            if let Ok(decoded) = SecondaryZoneSetFilter::decode_log(log) {
                return Ok(MainZoneEvents::SecondaryZoneSetFilter(decoded));
            }
            if let Ok(decoded) = UnpausedFilter::decode_log(log) {
                return Ok(MainZoneEvents::UnpausedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for MainZoneEvents {
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
                Self::PausedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RoleAdminChangedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RoleGrantedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RoleRevokedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::SecondaryZoneSetFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::UnpausedFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<DefaultAdminDelayChangeCanceledFilter> for MainZoneEvents {
        fn from(value: DefaultAdminDelayChangeCanceledFilter) -> Self {
            Self::DefaultAdminDelayChangeCanceledFilter(value)
        }
    }
    impl ::core::convert::From<DefaultAdminDelayChangeScheduledFilter> for MainZoneEvents {
        fn from(value: DefaultAdminDelayChangeScheduledFilter) -> Self {
            Self::DefaultAdminDelayChangeScheduledFilter(value)
        }
    }
    impl ::core::convert::From<DefaultAdminTransferCanceledFilter> for MainZoneEvents {
        fn from(value: DefaultAdminTransferCanceledFilter) -> Self {
            Self::DefaultAdminTransferCanceledFilter(value)
        }
    }
    impl ::core::convert::From<DefaultAdminTransferScheduledFilter> for MainZoneEvents {
        fn from(value: DefaultAdminTransferScheduledFilter) -> Self {
            Self::DefaultAdminTransferScheduledFilter(value)
        }
    }
    impl ::core::convert::From<PausedFilter> for MainZoneEvents {
        fn from(value: PausedFilter) -> Self {
            Self::PausedFilter(value)
        }
    }
    impl ::core::convert::From<RoleAdminChangedFilter> for MainZoneEvents {
        fn from(value: RoleAdminChangedFilter) -> Self {
            Self::RoleAdminChangedFilter(value)
        }
    }
    impl ::core::convert::From<RoleGrantedFilter> for MainZoneEvents {
        fn from(value: RoleGrantedFilter) -> Self {
            Self::RoleGrantedFilter(value)
        }
    }
    impl ::core::convert::From<RoleRevokedFilter> for MainZoneEvents {
        fn from(value: RoleRevokedFilter) -> Self {
            Self::RoleRevokedFilter(value)
        }
    }
    impl ::core::convert::From<SecondaryZoneSetFilter> for MainZoneEvents {
        fn from(value: SecondaryZoneSetFilter) -> Self {
            Self::SecondaryZoneSetFilter(value)
        }
    }
    impl ::core::convert::From<UnpausedFilter> for MainZoneEvents {
        fn from(value: UnpausedFilter) -> Self {
            Self::UnpausedFilter(value)
        }
    }
    ///Container type for all input parameters for the `BOOK_ROLE` function with signature `BOOK_ROLE()` and selector `0xdd11b28a`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "BOOK_ROLE", abi = "BOOK_ROLE()")]
    pub struct BookRoleCall;
    ///Container type for all input parameters for the `CALLER_ROLE` function with signature `CALLER_ROLE()` and selector `0x774237fc`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "CALLER_ROLE", abi = "CALLER_ROLE()")]
    pub struct CallerRoleCall;
    ///Container type for all input parameters for the `CANCELLED_ORDERS` function with signature `CANCELLED_ORDERS()` and selector `0xd89e8d94`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "CANCELLED_ORDERS", abi = "CANCELLED_ORDERS()")]
    pub struct CancelledOrdersCall;
    ///Container type for all input parameters for the `DEFAULT_ADMIN_ROLE` function with signature `DEFAULT_ADMIN_ROLE()` and selector `0xa217fddf`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "DEFAULT_ADMIN_ROLE", abi = "DEFAULT_ADMIN_ROLE()")]
    pub struct DefaultAdminRoleCall;
    ///Container type for all input parameters for the `FULFILLER_ROLE` function with signature `FULFILLER_ROLE()` and selector `0x4fd331c6`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "FULFILLER_ROLE", abi = "FULFILLER_ROLE()")]
    pub struct FulfillerRoleCall;
    ///Container type for all input parameters for the `acceptDefaultAdminTransfer` function with signature `acceptDefaultAdminTransfer()` and selector `0xcefc1429`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "acceptDefaultAdminTransfer",
        abi = "acceptDefaultAdminTransfer()"
    )]
    pub struct AcceptDefaultAdminTransferCall;
    ///Container type for all input parameters for the `beginDefaultAdminTransfer` function with signature `beginDefaultAdminTransfer(address)` and selector `0x634e93da`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
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
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "cancelDefaultAdminTransfer",
        abi = "cancelDefaultAdminTransfer()"
    )]
    pub struct CancelDefaultAdminTransferCall;
    ///Container type for all input parameters for the `changeDefaultAdminDelay` function with signature `changeDefaultAdminDelay(uint48)` and selector `0x649a5ec7`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "changeDefaultAdminDelay",
        abi = "changeDefaultAdminDelay(uint48)"
    )]
    pub struct ChangeDefaultAdminDelayCall {
        pub new_delay: u64,
    }
    ///Container type for all input parameters for the `defaultAdmin` function with signature `defaultAdmin()` and selector `0x84ef8ffc`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "defaultAdmin", abi = "defaultAdmin()")]
    pub struct DefaultAdminCall;
    ///Container type for all input parameters for the `defaultAdminDelay` function with signature `defaultAdminDelay()` and selector `0xcc8463c8`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "defaultAdminDelay", abi = "defaultAdminDelay()")]
    pub struct DefaultAdminDelayCall;
    ///Container type for all input parameters for the `defaultAdminDelayIncreaseWait` function with signature `defaultAdminDelayIncreaseWait()` and selector `0x022d63fb`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "defaultAdminDelayIncreaseWait",
        abi = "defaultAdminDelayIncreaseWait()"
    )]
    pub struct DefaultAdminDelayIncreaseWaitCall;
    ///Container type for all input parameters for the `getRoleAdmin` function with signature `getRoleAdmin(bytes32)` and selector `0x248a9ca3`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
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
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
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
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
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
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "owner", abi = "owner()")]
    pub struct OwnerCall;
    ///Container type for all input parameters for the `pause` function with signature `pause()` and selector `0x8456cb59`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "pause", abi = "pause()")]
    pub struct PauseCall;
    ///Container type for all input parameters for the `paused` function with signature `paused()` and selector `0x5c975abb`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "paused", abi = "paused()")]
    pub struct PausedCall;
    ///Container type for all input parameters for the `pendingDefaultAdmin` function with signature `pendingDefaultAdmin()` and selector `0xcf6eefb7`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "pendingDefaultAdmin", abi = "pendingDefaultAdmin()")]
    pub struct PendingDefaultAdminCall;
    ///Container type for all input parameters for the `pendingDefaultAdminDelay` function with signature `pendingDefaultAdminDelay()` and selector `0xa1eda53c`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "pendingDefaultAdminDelay", abi = "pendingDefaultAdminDelay()")]
    pub struct PendingDefaultAdminDelayCall;
    ///Container type for all input parameters for the `renounceRole` function with signature `renounceRole(bytes32,address)` and selector `0x36568abe`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
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
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
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
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "rollbackDefaultAdminDelay",
        abi = "rollbackDefaultAdminDelay()"
    )]
    pub struct RollbackDefaultAdminDelayCall;
    ///Container type for all input parameters for the `secondaryZone` function with signature `secondaryZone()` and selector `0x11b102b8`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "secondaryZone", abi = "secondaryZone()")]
    pub struct SecondaryZoneCall;
    ///Container type for all input parameters for the `setSecondaryZone` function with signature `setSecondaryZone(address)` and selector `0xa7aa8b74`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "setSecondaryZone", abi = "setSecondaryZone(address)")]
    pub struct SetSecondaryZoneCall {
        pub new_secondary_zone: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `supportsInterface` function with signature `supportsInterface(bytes4)` and selector `0x01ffc9a7`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
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
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "unpause", abi = "unpause()")]
    pub struct UnpauseCall;
    ///Container type for all input parameters for the `validateOrder` function with signature `validateOrder((address,address,(address,uint256)[],(address,uint256)[],uint256,uint256),address,address,bytes32)` and selector `0x0f1b2ea4`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "validateOrder",
        abi = "validateOrder((address,address,(address,uint256)[],(address,uint256)[],uint256,uint256),address,address,bytes32)"
    )]
    pub struct ValidateOrder0Call {
        pub p0: Order,
        pub book: ::ethers::core::types::Address,
        pub caller: ::ethers::core::types::Address,
        pub order_hash: [u8; 32],
    }
    ///Container type for all input parameters for the `validateOrder` function with signature `validateOrder((address,address,(address,uint256)[],(address,uint256)[],uint256,uint256),address,address,address,bytes32,bytes)` and selector `0x4d6dcb5c`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "validateOrder",
        abi = "validateOrder((address,address,(address,uint256)[],(address,uint256)[],uint256,uint256),address,address,address,bytes32,bytes)"
    )]
    pub struct ValidateOrder1Call {
        pub p0: Order,
        pub book: ::ethers::core::types::Address,
        pub fulfiller: ::ethers::core::types::Address,
        pub caller: ::ethers::core::types::Address,
        pub order_hash: [u8; 32],
        pub p5: ::ethers::core::types::Bytes,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum MainZoneCalls {
        BookRole(BookRoleCall),
        CallerRole(CallerRoleCall),
        CancelledOrders(CancelledOrdersCall),
        DefaultAdminRole(DefaultAdminRoleCall),
        FulfillerRole(FulfillerRoleCall),
        AcceptDefaultAdminTransfer(AcceptDefaultAdminTransferCall),
        BeginDefaultAdminTransfer(BeginDefaultAdminTransferCall),
        CancelDefaultAdminTransfer(CancelDefaultAdminTransferCall),
        ChangeDefaultAdminDelay(ChangeDefaultAdminDelayCall),
        DefaultAdmin(DefaultAdminCall),
        DefaultAdminDelay(DefaultAdminDelayCall),
        DefaultAdminDelayIncreaseWait(DefaultAdminDelayIncreaseWaitCall),
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
        SecondaryZone(SecondaryZoneCall),
        SetSecondaryZone(SetSecondaryZoneCall),
        SupportsInterface(SupportsInterfaceCall),
        Unpause(UnpauseCall),
        ValidateOrder0(ValidateOrder0Call),
        ValidateOrder1(ValidateOrder1Call),
    }
    impl ::ethers::core::abi::AbiDecode for MainZoneCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <BookRoleCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::BookRole(decoded));
            }
            if let Ok(decoded) = <CallerRoleCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::CallerRole(decoded));
            }
            if let Ok(decoded) =
                <CancelledOrdersCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::CancelledOrders(decoded));
            }
            if let Ok(decoded) =
                <DefaultAdminRoleCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::DefaultAdminRole(decoded));
            }
            if let Ok(decoded) = <FulfillerRoleCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::FulfillerRole(decoded));
            }
            if let Ok(decoded) =
                <AcceptDefaultAdminTransferCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::AcceptDefaultAdminTransfer(decoded));
            }
            if let Ok(decoded) =
                <BeginDefaultAdminTransferCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::BeginDefaultAdminTransfer(decoded));
            }
            if let Ok(decoded) =
                <CancelDefaultAdminTransferCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::CancelDefaultAdminTransfer(decoded));
            }
            if let Ok(decoded) =
                <ChangeDefaultAdminDelayCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ChangeDefaultAdminDelay(decoded));
            }
            if let Ok(decoded) = <DefaultAdminCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::DefaultAdmin(decoded));
            }
            if let Ok(decoded) =
                <DefaultAdminDelayCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::DefaultAdminDelay(decoded));
            }
            if let Ok(decoded) =
                <DefaultAdminDelayIncreaseWaitCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::DefaultAdminDelayIncreaseWait(decoded));
            }
            if let Ok(decoded) = <GetRoleAdminCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetRoleAdmin(decoded));
            }
            if let Ok(decoded) = <GrantRoleCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GrantRole(decoded));
            }
            if let Ok(decoded) = <HasRoleCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::HasRole(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded) = <PauseCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Pause(decoded));
            }
            if let Ok(decoded) = <PausedCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Paused(decoded));
            }
            if let Ok(decoded) =
                <PendingDefaultAdminCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::PendingDefaultAdmin(decoded));
            }
            if let Ok(decoded) =
                <PendingDefaultAdminDelayCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::PendingDefaultAdminDelay(decoded));
            }
            if let Ok(decoded) = <RenounceRoleCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RenounceRole(decoded));
            }
            if let Ok(decoded) = <RevokeRoleCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RevokeRole(decoded));
            }
            if let Ok(decoded) =
                <RollbackDefaultAdminDelayCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RollbackDefaultAdminDelay(decoded));
            }
            if let Ok(decoded) = <SecondaryZoneCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SecondaryZone(decoded));
            }
            if let Ok(decoded) =
                <SetSecondaryZoneCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetSecondaryZone(decoded));
            }
            if let Ok(decoded) =
                <SupportsInterfaceCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SupportsInterface(decoded));
            }
            if let Ok(decoded) = <UnpauseCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Unpause(decoded));
            }
            if let Ok(decoded) =
                <ValidateOrder0Call as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ValidateOrder0(decoded));
            }
            if let Ok(decoded) =
                <ValidateOrder1Call as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ValidateOrder1(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for MainZoneCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::BookRole(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::CallerRole(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::CancelledOrders(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::DefaultAdminRole(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::FulfillerRole(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::AcceptDefaultAdminTransfer(element) => {
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
                Self::DefaultAdmin(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::DefaultAdminDelay(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::DefaultAdminDelayIncreaseWait(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetRoleAdmin(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GrantRole(element) => ::ethers::core::abi::AbiEncode::encode(element),
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
                Self::RenounceRole(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RevokeRole(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RollbackDefaultAdminDelay(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SecondaryZone(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetSecondaryZone(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SupportsInterface(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Unpause(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ValidateOrder0(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ValidateOrder1(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for MainZoneCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::BookRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::CallerRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::CancelledOrders(element) => ::core::fmt::Display::fmt(element, f),
                Self::DefaultAdminRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::FulfillerRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::AcceptDefaultAdminTransfer(element) => ::core::fmt::Display::fmt(element, f),
                Self::BeginDefaultAdminTransfer(element) => ::core::fmt::Display::fmt(element, f),
                Self::CancelDefaultAdminTransfer(element) => ::core::fmt::Display::fmt(element, f),
                Self::ChangeDefaultAdminDelay(element) => ::core::fmt::Display::fmt(element, f),
                Self::DefaultAdmin(element) => ::core::fmt::Display::fmt(element, f),
                Self::DefaultAdminDelay(element) => ::core::fmt::Display::fmt(element, f),
                Self::DefaultAdminDelayIncreaseWait(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetRoleAdmin(element) => ::core::fmt::Display::fmt(element, f),
                Self::GrantRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::HasRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::Pause(element) => ::core::fmt::Display::fmt(element, f),
                Self::Paused(element) => ::core::fmt::Display::fmt(element, f),
                Self::PendingDefaultAdmin(element) => ::core::fmt::Display::fmt(element, f),
                Self::PendingDefaultAdminDelay(element) => ::core::fmt::Display::fmt(element, f),
                Self::RenounceRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevokeRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::RollbackDefaultAdminDelay(element) => ::core::fmt::Display::fmt(element, f),
                Self::SecondaryZone(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetSecondaryZone(element) => ::core::fmt::Display::fmt(element, f),
                Self::SupportsInterface(element) => ::core::fmt::Display::fmt(element, f),
                Self::Unpause(element) => ::core::fmt::Display::fmt(element, f),
                Self::ValidateOrder0(element) => ::core::fmt::Display::fmt(element, f),
                Self::ValidateOrder1(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<BookRoleCall> for MainZoneCalls {
        fn from(value: BookRoleCall) -> Self {
            Self::BookRole(value)
        }
    }
    impl ::core::convert::From<CallerRoleCall> for MainZoneCalls {
        fn from(value: CallerRoleCall) -> Self {
            Self::CallerRole(value)
        }
    }
    impl ::core::convert::From<CancelledOrdersCall> for MainZoneCalls {
        fn from(value: CancelledOrdersCall) -> Self {
            Self::CancelledOrders(value)
        }
    }
    impl ::core::convert::From<DefaultAdminRoleCall> for MainZoneCalls {
        fn from(value: DefaultAdminRoleCall) -> Self {
            Self::DefaultAdminRole(value)
        }
    }
    impl ::core::convert::From<FulfillerRoleCall> for MainZoneCalls {
        fn from(value: FulfillerRoleCall) -> Self {
            Self::FulfillerRole(value)
        }
    }
    impl ::core::convert::From<AcceptDefaultAdminTransferCall> for MainZoneCalls {
        fn from(value: AcceptDefaultAdminTransferCall) -> Self {
            Self::AcceptDefaultAdminTransfer(value)
        }
    }
    impl ::core::convert::From<BeginDefaultAdminTransferCall> for MainZoneCalls {
        fn from(value: BeginDefaultAdminTransferCall) -> Self {
            Self::BeginDefaultAdminTransfer(value)
        }
    }
    impl ::core::convert::From<CancelDefaultAdminTransferCall> for MainZoneCalls {
        fn from(value: CancelDefaultAdminTransferCall) -> Self {
            Self::CancelDefaultAdminTransfer(value)
        }
    }
    impl ::core::convert::From<ChangeDefaultAdminDelayCall> for MainZoneCalls {
        fn from(value: ChangeDefaultAdminDelayCall) -> Self {
            Self::ChangeDefaultAdminDelay(value)
        }
    }
    impl ::core::convert::From<DefaultAdminCall> for MainZoneCalls {
        fn from(value: DefaultAdminCall) -> Self {
            Self::DefaultAdmin(value)
        }
    }
    impl ::core::convert::From<DefaultAdminDelayCall> for MainZoneCalls {
        fn from(value: DefaultAdminDelayCall) -> Self {
            Self::DefaultAdminDelay(value)
        }
    }
    impl ::core::convert::From<DefaultAdminDelayIncreaseWaitCall> for MainZoneCalls {
        fn from(value: DefaultAdminDelayIncreaseWaitCall) -> Self {
            Self::DefaultAdminDelayIncreaseWait(value)
        }
    }
    impl ::core::convert::From<GetRoleAdminCall> for MainZoneCalls {
        fn from(value: GetRoleAdminCall) -> Self {
            Self::GetRoleAdmin(value)
        }
    }
    impl ::core::convert::From<GrantRoleCall> for MainZoneCalls {
        fn from(value: GrantRoleCall) -> Self {
            Self::GrantRole(value)
        }
    }
    impl ::core::convert::From<HasRoleCall> for MainZoneCalls {
        fn from(value: HasRoleCall) -> Self {
            Self::HasRole(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for MainZoneCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<PauseCall> for MainZoneCalls {
        fn from(value: PauseCall) -> Self {
            Self::Pause(value)
        }
    }
    impl ::core::convert::From<PausedCall> for MainZoneCalls {
        fn from(value: PausedCall) -> Self {
            Self::Paused(value)
        }
    }
    impl ::core::convert::From<PendingDefaultAdminCall> for MainZoneCalls {
        fn from(value: PendingDefaultAdminCall) -> Self {
            Self::PendingDefaultAdmin(value)
        }
    }
    impl ::core::convert::From<PendingDefaultAdminDelayCall> for MainZoneCalls {
        fn from(value: PendingDefaultAdminDelayCall) -> Self {
            Self::PendingDefaultAdminDelay(value)
        }
    }
    impl ::core::convert::From<RenounceRoleCall> for MainZoneCalls {
        fn from(value: RenounceRoleCall) -> Self {
            Self::RenounceRole(value)
        }
    }
    impl ::core::convert::From<RevokeRoleCall> for MainZoneCalls {
        fn from(value: RevokeRoleCall) -> Self {
            Self::RevokeRole(value)
        }
    }
    impl ::core::convert::From<RollbackDefaultAdminDelayCall> for MainZoneCalls {
        fn from(value: RollbackDefaultAdminDelayCall) -> Self {
            Self::RollbackDefaultAdminDelay(value)
        }
    }
    impl ::core::convert::From<SecondaryZoneCall> for MainZoneCalls {
        fn from(value: SecondaryZoneCall) -> Self {
            Self::SecondaryZone(value)
        }
    }
    impl ::core::convert::From<SetSecondaryZoneCall> for MainZoneCalls {
        fn from(value: SetSecondaryZoneCall) -> Self {
            Self::SetSecondaryZone(value)
        }
    }
    impl ::core::convert::From<SupportsInterfaceCall> for MainZoneCalls {
        fn from(value: SupportsInterfaceCall) -> Self {
            Self::SupportsInterface(value)
        }
    }
    impl ::core::convert::From<UnpauseCall> for MainZoneCalls {
        fn from(value: UnpauseCall) -> Self {
            Self::Unpause(value)
        }
    }
    impl ::core::convert::From<ValidateOrder0Call> for MainZoneCalls {
        fn from(value: ValidateOrder0Call) -> Self {
            Self::ValidateOrder0(value)
        }
    }
    impl ::core::convert::From<ValidateOrder1Call> for MainZoneCalls {
        fn from(value: ValidateOrder1Call) -> Self {
            Self::ValidateOrder1(value)
        }
    }
    ///Container type for all return fields from the `BOOK_ROLE` function with signature `BOOK_ROLE()` and selector `0xdd11b28a`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct BookRoleReturn(pub [u8; 32]);
    ///Container type for all return fields from the `CALLER_ROLE` function with signature `CALLER_ROLE()` and selector `0x774237fc`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct CallerRoleReturn(pub [u8; 32]);
    ///Container type for all return fields from the `CANCELLED_ORDERS` function with signature `CANCELLED_ORDERS()` and selector `0xd89e8d94`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct CancelledOrdersReturn(pub [u8; 32]);
    ///Container type for all return fields from the `DEFAULT_ADMIN_ROLE` function with signature `DEFAULT_ADMIN_ROLE()` and selector `0xa217fddf`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct DefaultAdminRoleReturn(pub [u8; 32]);
    ///Container type for all return fields from the `FULFILLER_ROLE` function with signature `FULFILLER_ROLE()` and selector `0x4fd331c6`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct FulfillerRoleReturn(pub [u8; 32]);
    ///Container type for all return fields from the `defaultAdmin` function with signature `defaultAdmin()` and selector `0x84ef8ffc`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct DefaultAdminReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `defaultAdminDelay` function with signature `defaultAdminDelay()` and selector `0xcc8463c8`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct DefaultAdminDelayReturn(pub u64);
    ///Container type for all return fields from the `defaultAdminDelayIncreaseWait` function with signature `defaultAdminDelayIncreaseWait()` and selector `0x022d63fb`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct DefaultAdminDelayIncreaseWaitReturn(pub u64);
    ///Container type for all return fields from the `getRoleAdmin` function with signature `getRoleAdmin(bytes32)` and selector `0x248a9ca3`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetRoleAdminReturn(pub [u8; 32]);
    ///Container type for all return fields from the `hasRole` function with signature `hasRole(bytes32,address)` and selector `0x91d14854`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct HasRoleReturn(pub bool);
    ///Container type for all return fields from the `owner` function with signature `owner()` and selector `0x8da5cb5b`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct OwnerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `paused` function with signature `paused()` and selector `0x5c975abb`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct PausedReturn(pub bool);
    ///Container type for all return fields from the `pendingDefaultAdmin` function with signature `pendingDefaultAdmin()` and selector `0xcf6eefb7`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
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
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct PendingDefaultAdminDelayReturn {
        pub new_delay: u64,
        pub schedule: u64,
    }
    ///Container type for all return fields from the `secondaryZone` function with signature `secondaryZone()` and selector `0x11b102b8`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct SecondaryZoneReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `supportsInterface` function with signature `supportsInterface(bytes4)` and selector `0x01ffc9a7`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct SupportsInterfaceReturn(pub bool);
}
