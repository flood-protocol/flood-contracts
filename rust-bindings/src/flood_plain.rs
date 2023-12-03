pub use flood_plain::*;
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
pub mod flood_plain {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("permit2"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("FALLBACK_SELECTOR"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("FALLBACK_SELECTOR"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        1usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes1"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PERMIT2"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("PERMIT2"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "contract ISignatureTransfer",
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
                    ::std::borrow::ToOwned::to_owned("addDecoder"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("addDecoder"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("decoder"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("id"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("decoders"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("decoders"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
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
                    ::std::borrow::ToOwned::to_owned("etchOrder"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("etchOrder"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "orderWithSignature",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
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
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IFloodPlain.SignedOrder",
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
                    ::std::borrow::ToOwned::to_owned("fulfillOrder"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("fulfillOrder"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("package"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
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
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IFloodPlain.SignedOrder",
                                        ),
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
                                    name: ::std::borrow::ToOwned::to_owned("swapData"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("fulfillOrder"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("package"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
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
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IFloodPlain.SignedOrder",
                                        ),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("fulfillOrders"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("fulfillOrders"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("packages"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Tuple(
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
                                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IFloodPlain.SignedOrder[]",
                                        ),
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
                                    name: ::std::borrow::ToOwned::to_owned("swapData"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
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
                    ::std::borrow::ToOwned::to_owned("getNonceStatus"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getNonceStatus"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("user"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("nonce"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
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
                    ::std::borrow::ToOwned::to_owned("getOrderHash"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getOrderHash"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("order"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getOrderStatus"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getOrderStatus"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("order"),
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
                    ::std::borrow::ToOwned::to_owned("getPermitHash"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getPermitHash"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("order"),
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
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("DecoderAdded"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("DecoderAdded"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("decoderId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("decoder"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OrderEtched"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("OrderEtched"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("orderHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "orderWithSignature",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
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
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
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
                    ::std::borrow::ToOwned::to_owned("OrderFulfilled"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("OrderFulfilled"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("order"),
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
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("fulfiller"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amountOut"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
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
                    ::std::borrow::ToOwned::to_owned("AddressEmptyCode"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("AddressEmptyCode"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("target"),
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
                    ::std::borrow::ToOwned::to_owned("AddressInsufficientBalance"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "AddressInsufficientBalance",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
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
                    ::std::borrow::ToOwned::to_owned("ArrayLengthMismatch"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ArrayLengthMismatch",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("DuplicateItems"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("DuplicateItems"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("FailedInnerCall"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("FailedInnerCall"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InsufficientAmountReceived"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InsufficientAmountReceived",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NotAContract"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("NotAContract"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ReentrancyGuardReentrantCall"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ReentrancyGuardReentrantCall",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SafeERC20FailedOperation"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "SafeERC20FailedOperation",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
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
                    ::std::borrow::ToOwned::to_owned("ZoneDenied"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("ZoneDenied"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
            ]),
            receive: true,
            fallback: true,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static FLOODPLAIN_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xA0`@R4\x80\x15b\0\0\x11W`\0\x80\xFD[P`@Qb\0/\xC08\x03\x80b\0/\xC0\x839\x81\x01`@\x81\x90Rb\0\x004\x91b\0\0JV[`\x01\x80U`\x01`\x01`\xA0\x1B\x03\x16`\x80Rb\0\0|V[`\0` \x82\x84\x03\x12\x15b\0\0]W`\0\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\0uW`\0\x80\xFD[\x93\x92PPPV[`\x80Qa/\x1Ab\0\0\xA6`\09`\0\x81\x81a\x02\x14\x01R\x81\x81a\r\x95\x01Ra\x13\xF5\x01Ra/\x1A`\0\xF3\xFE`\x80`@R`\x046\x10a\0\xABW`\x005`\xE0\x1C\x80ck\"L\xCB\x11a\0dW\x80ck\"L\xCB\x14a\x02NW\x80cv&\x96c\x14a\x02nW\x80cxeJR\x14a\x02\x8EW\x80c\x9DH\x1Bf\x14a\x02\xAEW\x80c\xAF^\x7F5\x14a\x02\xCEW\x80c\xE9\xBA\x1E\x97\x14a\x02\xFEWa\0\xB2V[\x80c=\xD9\x08\xFC\x14a\x01NW\x80cA\xBA\xC9S\x14a\x01\x81W\x80cN\xE3\x99_\x14a\x01\xA1W\x80cO\xF6\x86\t\x14a\x01\xC1W\x80ci`J\xAD\x14a\x01\xEFW\x80cj\xFD\xD8P\x14a\x02\x02Wa\0\xB2V[6a\0\xB2W\0[4\x80\x15a\0\xBEW`\0\x80\xFD[P`\0\x80a\0\xCC`\x01a\x03\x1EV[\x91P\x91P`\0\x80\x83\x81T\x81\x10a\0\xE4Wa\0\xE4a\x1D2V[`\0\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x90P6\x82\x81\x03\x90\x81\x11\x15a\x01\x0CW`\0\x80\xFD[\x80\x83`\x007`\0\x80\x82`\0\x85Z\xFA\x90P=`\0\x80>\x80a\x01+W=`\0\xFD[`\0\x80=`\x000Z\xF4\x90P=`\0\x80>\x80\x80\x15a\x01GW=`\0\xF3[=`\0\xFD[\0[4\x80\x15a\x01ZW`\0\x80\xFD[Pa\x01na\x01i6`\x04a\x1DHV[a\x03]V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01\x8DW`\0\x80\xFD[Pa\x01La\x01\x9C6`\x04a\x1D\x9BV[a\x03nV[4\x80\x15a\x01\xADW`\0\x80\xFD[Pa\x01La\x01\xBC6`\x04a\x1E6V[a\x03\xC0V[4\x80\x15a\x01\xCDW`\0\x80\xFD[Pa\x01\xD6`\0\x81V[`@Q`\x01`\x01`\xF8\x1B\x03\x19\x90\x91\x16\x81R` \x01a\x01xV[a\x01La\x01\xFD6`\x04a\x1D\x9BV[a\x06cV[4\x80\x15a\x02\x0EW`\0\x80\xFD[Pa\x026\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01xV[4\x80\x15a\x02ZW`\0\x80\xFD[Pa\x01na\x02i6`\x04a\x1DHV[a\x08,V[4\x80\x15a\x02zW`\0\x80\xFD[Pa\x01La\x02\x896`\x04a\x1E\xACV[a\x088V[4\x80\x15a\x02\x9AW`\0\x80\xFD[Pa\x026a\x02\xA96`\x04a\x1F]V[a\x0CpV[4\x80\x15a\x02\xBAW`\0\x80\xFD[Pa\x01na\x02\xC96`\x04a\x1FvV[a\x0C\x9AV[4\x80\x15a\x02\xDAW`\0\x80\xFD[Pa\x02\xEEa\x02\xE96`\x04a\x1DHV[a\r\x17V[`@Q\x90\x15\x15\x81R` \x01a\x01xV[4\x80\x15a\x03\nW`\0\x80\xFD[Pa\x02\xEEa\x03\x196`\x04a\x1F\x91V[a\raV[`\0\x80`\0[\x835`\0\x1A`\x7F\x81\x16\x82\x1B\x84\x17\x93P`\x01\x85\x01\x94P\x80`\x07\x1C\x15a\x03HWPa\x03NV[Pa\x03VV[`\x07\x01a\x03$V[P\x90\x92\x90PV[`\0a\x03h\x82a\x0E\tV[\x92\x91PPV[`\0a\x03\x82a\x03}\x83\x80a\x1F\xBBV[a\x0E\tV[\x90P\x80\x7F\xED\xF4\x9C\0=\xE6)\x8FV\x1A\x96\x8A\xDE5\xBD=\xCF\x06\xFF\xA1iy\xDE7]\xBB9x-o\xCB\xE4\x83`@Qa\x03\xB4\x91\x90a\"\xBEV[`@Q\x80\x91\x03\x90\xA2PPV[a\x03\xC8a\x11\xCAV[6a\x03\xD3\x85\x80a\x1F\xBBV[\x90P`\0a\x03\xE0\x82a\x0E\tV[\x90P`\0a\x03\xF4`@\x84\x01` \x85\x01a\x1FvV[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x04\x9DWa\x04\x12`@\x83\x01` \x84\x01a\x1FvV[`\x01`\x01`\xA0\x1B\x03\x16c\xADs\xD6\x89\x833`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x04?\x92\x91\x90a#\x15V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\\W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\x80\x91\x90a#?V[a\x04\x9DW`@Qc\x03\x12(W`\xE5\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x04\xB3a\x04\xAEa\x01\0\x84\x01\x84a#aV[a\x11\xF4V[a\x04\xCB\x82a\x04\xC4` \x89\x01\x89a#\xAAV[\x843a\x128V[`@Qc\x9E_\x81\xF5`\xE0\x1B\x81R`\0\x90`\x01`\x01`\xA0\x1B\x03\x87\x16\x90c\x9E_\x81\xF5\x90a\x05\"\x90\x7F\x13\x8B\xEA\xEB\xD3Fv\xDD\xCA\xAB\xA2\xACu\xBB\xD1D\xC6R\xC8\xC6\xD93\xF9b$\\a\xFF\xEF\x90\xD7-\x90\x87\x903\x90\x8B\x90\x8B\x90`\x04\x01a#\xF0V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x05AW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05e\x91\x90a$>V[\x90P`\xA0\x83\x015\x81\x10\x15a\x05\x8CW`@QcH\x87\x9A\t`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x05\x9E`\xA0\x85\x01`\x80\x86\x01a\x1FvV[\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16a\x05\xD6Wa\x05\xD1\x82a\x05\xC2``\x87\x01`@\x88\x01a\x1FvV[`\x01`\x01`\xA0\x1B\x03\x16\x90a\x15tV[a\x05\xFCV[a\x05\xFC\x87a\x05\xEA``\x87\x01`@\x88\x01a\x1FvV[`\x01`\x01`\xA0\x1B\x03\x84\x16\x91\x90\x85a\x16\x10V[a\x06\ra\x04\xAEa\x01 \x86\x01\x86a#aV[3`\x01`\x01`\xA0\x1B\x03\x16\x7F6b\xC2\xB9tk\xA3\xF8}\xEFd@\x10\xA3\xFA\x9E\x032\xDAS\xA5w\x17\x94\xC7\x9D{l\xCE$\xD9+\x85\x84`@Qa\x06H\x92\x91\x90a$WV[`@Q\x80\x91\x03\x90\xA2PPPPa\x06]`\x01\x80UV[PPPPV[a\x06ka\x11\xCAV[6a\x06v\x82\x80a\x1F\xBBV[\x90P`\0a\x06\x83\x82a\x0E\tV[\x90P`\0a\x06\x97`@\x84\x01` \x85\x01a\x1FvV[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x07@Wa\x06\xB5`@\x83\x01` \x84\x01a\x1FvV[`\x01`\x01`\xA0\x1B\x03\x16c\xADs\xD6\x89\x833`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x06\xE2\x92\x91\x90a#\x15V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06\xFFW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07#\x91\x90a#?V[a\x07@W`@Qc\x03\x12(W`\xE5\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x07Qa\x04\xAEa\x01\0\x84\x01\x84a#aV[a\x07b\x82a\x04\xC4` \x86\x01\x86a#\xAAV[`\0a\x07t`\xA0\x84\x01`\x80\x85\x01a\x1FvV[\x90P`\xA0\x83\x015`\x01`\x01`\xA0\x1B\x03\x82\x16a\x07\xA2Wa\x07\x9D\x81a\x05\xC2``\x87\x01`@\x88\x01a\x1FvV[a\x07\xC8V[a\x07\xC83a\x07\xB6``\x87\x01`@\x88\x01a\x1FvV[`\x01`\x01`\xA0\x1B\x03\x85\x16\x91\x90\x84a\x16\x10V[a\x07\xD9a\x04\xAEa\x01 \x86\x01\x86a#aV[3`\x01`\x01`\xA0\x1B\x03\x16\x7F6b\xC2\xB9tk\xA3\xF8}\xEFd@\x10\xA3\xFA\x9E\x032\xDAS\xA5w\x17\x94\xC7\x9D{l\xCE$\xD9+\x85\x83`@Qa\x08\x14\x92\x91\x90a$WV[`@Q\x80\x91\x03\x90\xA2PPPPa\x08)`\x01\x80UV[PV[`\0a\x03h\x820a\x16jV[a\x08@a\x11\xCAV[`\0\x84`\x01`\x01`@\x1B\x03\x81\x11\x15a\x08ZWa\x08Za$yV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x08\xE8W\x81` \x01[a\x08\xD5`@\x80Qa\x01 \x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x90R\x82\x84\x01\x82\x90R``\x80\x84\x01R\x83Q\x80\x85\x01\x90\x94R\x81\x84R\x83\x01R\x90`\x80\x82\x01\x90\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01``\x81R` \x01``\x81RP\x90V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x08xW\x90P[P\x90P`\0[\x85\x81\x10\x15a\ngW6\x87\x87\x83\x81\x81\x10a\t\tWa\t\ta\x1D2V[\x90P` \x02\x81\x01\x90a\t\x1B\x91\x90a$\x8FV[a\t%\x90\x80a\x1F\xBBV[\x90P`\0a\t9`@\x83\x01` \x84\x01a\x1FvV[`\x01`\x01`\xA0\x1B\x03\x16\x14a\t\xE2Wa\tW`@\x82\x01` \x83\x01a\x1FvV[`\x01`\x01`\xA0\x1B\x03\x16c\xADs\xD6\x89\x82\x88`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\t\x84\x92\x91\x90a#\x15V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t\xA1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\xC5\x91\x90a#?V[a\t\xE2W`@Qc\x03\x12(W`\xE5\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\t\xF3a\x04\xAEa\x01\0\x83\x01\x83a#aV[a\n8\x81\x89\x89\x85\x81\x81\x10a\n\tWa\n\ta\x1D2V[\x90P` \x02\x81\x01\x90a\n\x1B\x91\x90a$\x8FV[a\n)\x90` \x81\x01\x90a#\xAAV[a\n2\x85a\x0E\tV[\x8Aa\x128V[a\nA\x81a'\x1FV[\x83\x83\x81Q\x81\x10a\nSWa\nSa\x1D2V[` \x90\x81\x02\x91\x90\x91\x01\x01RP`\x01\x01a\x08\xEEV[P`@Qc\x9A\xD81\xA1`\xE0\x1B\x81R`\0\x90`\x01`\x01`\xA0\x1B\x03\x86\x16\x90c\x9A\xD81\xA1\x90a\n\xBF\x90\x7F\x13\x8B\xEA\xEB\xD3Fv\xDD\xCA\xAB\xA2\xACu\xBB\xD1D\xC6R\xC8\xC6\xD93\xF9b$\\a\xFF\xEF\x90\xD7-\x90\x86\x903\x90\x8A\x90\x8A\x90`\x04\x01a)-V[`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\n\xDEW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x0B\x06\x91\x90\x81\x01\x90a*\x87V[\x80Q\x90\x91P\x86\x14a\x0B*W`@QcQ%\t\xD3`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0[\x86\x81\x10\x15a\x0C]W6\x88\x88\x83\x81\x81\x10a\x0BHWa\x0BHa\x1D2V[\x90P` \x02\x81\x01\x90a\x0BZ\x91\x90a$\x8FV[a\x0Bd\x90\x80a\x1F\xBBV[\x90P`\0\x83\x83\x81Q\x81\x10a\x0BzWa\x0Bza\x1D2V[` \x02` \x01\x01Q\x90P\x81`\x80\x01` \x015\x81\x10\x15a\x0B\xACW`@QcH\x87\x9A\t`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x0B\xBE`\xA0\x84\x01`\x80\x85\x01a\x1FvV[\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16a\x0B\xE7Wa\x0B\xE2\x82a\x05\xC2``\x86\x01`@\x87\x01a\x1FvV[a\x0B\xFBV[a\x0B\xFB\x89a\x05\xEA``\x86\x01`@\x87\x01a\x1FvV[a\x0C\x0Ca\x04\xAEa\x01 \x85\x01\x85a#aV[\x88`\x01`\x01`\xA0\x1B\x03\x16\x7F6b\xC2\xB9tk\xA3\xF8}\xEFd@\x10\xA3\xFA\x9E\x032\xDAS\xA5w\x17\x94\xC7\x9D{l\xCE$\xD9+\x84\x84`@Qa\x0CG\x92\x91\x90a$WV[`@Q\x80\x91\x03\x90\xA2PPP\x80`\x01\x01\x90Pa\x0B-V[PPPa\x0Ci`\x01\x80UV[PPPPPV[`\0\x81\x81T\x81\x10a\x0C\x80W`\0\x80\xFD[`\0\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x90P\x81V[`\0\x80T`\x01\x81\x01\x82U\x81\x80R\x7F)\r\xEC\xD9T\x8Bb\xA8\xD6\x03E\xA9\x888o\xC8K\xA6\xBC\x95H@\x08\xF66/\x93\x16\x0E\xF3\xE5c\x81\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x85\x16\x90\x81\x17\x90\x91U`@Q\x91\x92\x90\x91\x83\x91\x7FN\x9F\xEF\xD4\xC8\xC2e\xAD\xAD\x06\xDE\x04*\xD2\"D\x11e0n\x8A\xC2>\xA5%\xDE\xE3?@F>d\x91\xA3\x91\x90PV[`\0\x81`\xC0\x015B\x11\x15a\r-WP`\0\x91\x90PV[a\rGa\r=` \x84\x01\x84a\x1FvV[\x83`\xE0\x015a\raV[\x15a\rTWP`\x01\x91\x90PV[P`\0\x91\x90PV[\x91\x90PV[`@Qc\x13\xF8\n\xD1`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x04\x83\x01R`\x08\x83\x90\x1C`$\x83\x01R`\0\x91`\x01`\xFF\x85\x16\x1B\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90cO\xE0+D\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r\xDCW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E\0\x91\x90a$>V[\x16\x15\x93\x92PPPV[`\0\x80a\x0E\x19``\x84\x01\x84a+\x0CV[\x90P\x90P`\0\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0E7Wa\x0E7a$yV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0E`W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x82\x81\x10\x15a\x0E\xBEWa\x0E\x99a\x0E~``\x87\x01\x87a+\x0CV[\x83\x81\x81\x10a\x0E\x8EWa\x0E\x8Ea\x1D2V[\x90P`@\x02\x01a\x19?V[\x82\x82\x81Q\x81\x10a\x0E\xABWa\x0E\xABa\x1D2V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x0EfV[Pa\x0E\xCDa\x01\0\x85\x01\x85a#aV[\x90P\x91P`\0\x82`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0E\xEBWa\x0E\xEBa$yV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0F\x14W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x83\x81\x10\x15a\x0F\x7FWa\x0FZa\x0F3a\x01\0\x88\x01\x88a#aV[\x83\x81\x81\x10a\x0FCWa\x0FCa\x1D2V[\x90P` \x02\x81\x01\x90a\x0FU\x91\x90a$\x8FV[a\x19\xB5V[\x82\x82\x81Q\x81\x10a\x0FlWa\x0Fla\x1D2V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x0F\x1AV[Pa\x0F\x8Ea\x01 \x86\x01\x86a#aV[\x90P\x92P`\0\x83`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0F\xACWa\x0F\xACa$yV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0F\xD5W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x84\x81\x10\x15a\x10\x19Wa\x0F\xF4a\x0F3a\x01 \x89\x01\x89a#aV[\x82\x82\x81Q\x81\x10a\x10\x06Wa\x10\x06a\x1D2V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x0F\xDBV[P`@Q\x80`\xC0\x01`@R\x80`\x95\x81R` \x01a.\x0E`\x95\x919`@Q\x80`@\x01`@R\x80`\x1F\x81R` \x01`\0\x80Q` a.\xC5\x839\x81Q\x91R\x81RP`@Q\x80``\x01`@R\x80`\"\x81R` \x01a.\xA3`\"\x919`@Q` \x01a\x10\x82\x93\x92\x91\x90a+UV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x80Q` \x91\x82\x01 \x90a\x10\xA7\x90\x88\x01\x88a\x1FvV[a\x10\xB7`@\x89\x01` \x8A\x01a\x1FvV[a\x10\xC7``\x8A\x01`@\x8B\x01a\x1FvV[\x86`@Q` \x01a\x10\xD8\x91\x90a+\x98V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 a\x10\xFA\x8B`\x80\x01a\x19?V[\x8B`\xC0\x015\x8C`\xE0\x015\x89`@Q` \x01a\x11\x15\x91\x90a+\x98V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x89`@Q` \x01a\x11<\x91\x90a+\x98V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x90\x83\x01\x9B\x90\x9BR`\x01`\x01`\xA0\x1B\x03\x99\x8A\x16\x90\x82\x01R\x96\x88\x16``\x88\x01R\x96\x90\x94\x16`\x80\x86\x01R`\xA0\x85\x01\x92\x90\x92R`\xC0\x84\x01R`\xE0\x83\x01Ra\x01\0\x82\x01Ra\x01 \x81\x01\x91\x90\x91Ra\x01@\x81\x01\x91\x90\x91Ra\x01`\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x94PPPPP\x91\x90PV[`\x02`\x01T\x03a\x11\xEDW`@Qc>\xE5\xAE\xB5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02`\x01UV[`\0[\x81\x81\x10\x15a\x123Wa\x12+\x83\x83\x83\x81\x81\x10a\x12\x14Wa\x12\x14a\x1D2V[\x90P` \x02\x81\x01\x90a\x12&\x91\x90a$\x8FV[a\x1AUV[`\x01\x01a\x11\xF7V[PPPV[6`\0a\x12H``\x88\x01\x88a+\x0CV[\x91P\x91Pa\x12V\x82\x82a\x1A\xDAV[\x15a\x12tW`@Qcva\xC1\xBB`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a\x12\x8EWa\x12\x8Ea$yV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x12\xD3W\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x12\xACW\x90P[P\x90P`\0\x82`\x01`\x01`@\x1B\x03\x81\x11\x15a\x12\xF0Wa\x12\xF0a$yV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x135W\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x13\x0EW\x90P[P\x90P`\0[\x83\x81\x10\x15a\x13\xF2W6\x85\x85\x83\x81\x81\x10a\x13VWa\x13Va\x1D2V[`@\x80Q\x80\x82\x01\x82R\x91\x02\x92\x90\x92\x01\x92P\x81\x90Pa\x13w` \x84\x01\x84a\x1FvV[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x82` \x015\x81RP\x84\x83\x81Q\x81\x10a\x13\x9FWa\x13\x9Fa\x1D2V[` \x02` \x01\x01\x81\x90RP`@Q\x80`@\x01`@R\x80\x88`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x82` \x015\x81RP\x83\x83\x81Q\x81\x10a\x13\xDEWa\x13\xDEa\x1D2V[` \x90\x81\x02\x91\x90\x91\x01\x01RP`\x01\x01a\x13;V[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xFE\x8E\xC1\xA7`@Q\x80``\x01`@R\x80\x85\x81R` \x01\x8C`\xE0\x015\x81R` \x01\x8C`\xC0\x015\x81RP\x83\x8C`\0\x01` \x81\x01\x90a\x14Y\x91\x90a\x1FvV[\x8A`@Q\x80`@\x01`@R\x80`\x0E\x81R` \x01mOrder witness)`\x90\x1B\x81RP`@Q\x80`@\x01`@R\x80`\x1F\x81R` \x01`\0\x80Q` a.\xC5\x839\x81Q\x91R\x81RP`@Q\x80``\x01`@R\x80`\"\x81R` \x01a.\xA3`\"\x919`@Q\x80`\xC0\x01`@R\x80`\x95\x81R` \x01a.\x0E`\x95\x919`@Q\x80``\x01`@R\x80`.\x81R` \x01a-\xE0`.\x919`@Q` \x01a\x15\x04\x95\x94\x93\x92\x91\x90a+\xCEV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x8E\x8E`@Q\x88c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x157\x97\x96\x95\x94\x93\x92\x91\x90a,9V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x15QW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x15eW=`\0\x80>=`\0\xFD[PPPPPPPPPPPPPV[\x80G\x10\x15a\x15\x9CW`@Qc\xCDx`Y`\xE0\x1B\x81R0`\x04\x82\x01R`$\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x82`\x01`\x01`\xA0\x1B\x03\x16\x82`@Q`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x15\xE9W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x15\xEEV[``\x91P[PP\x90P\x80a\x123W`@Qc\n\x12\xF5!`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`$\x83\x01R\x84\x16`D\x82\x01R`d\x80\x82\x01\x84\x90R\x82Q\x80\x83\x03\x90\x91\x01\x81R`\x84\x90\x91\x01\x90\x91R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c#\xB8r\xDD`\xE0\x1B\x17\x90Ra\x06]\x90\x85\x90a\x1B\x98V[`\x006\x81a\x16{``\x86\x01\x86a+\x0CV[\x90\x92P\x90P`\0\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a\x16\x9AWa\x16\x9Aa$yV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x16\xC3W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0\x82`\x01`\x01`@\x1B\x03\x81\x11\x15a\x16\xE0Wa\x16\xE0a$yV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x17\tW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x83\x81\x10\x15a\x17\xFBW6\x85\x85\x83\x81\x81\x10a\x17*Wa\x17*a\x1D2V[\x90P`@\x02\x01\x90Pa\x17;\x81a\x19?V[\x83\x83\x81Q\x81\x10a\x17MWa\x17Ma\x1D2V[` \x02` \x01\x01\x81\x81RPP\x7Fa\x83X\xAC=\xB8\xDC'O\x0C\xD8\x82\x9D\xA7\xE24\xBDH\xCDs\xC4\xA7@\xAE\xDE\x1A\xDE\xC9\x84m\x06\xA1`@Q\x80`@\x01`@R\x80\x83`\0\x01` \x81\x01\x90a\x17\x98\x91\x90a\x1FvV[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x83` \x015\x81RP`@Q` \x01a\x17\xBF\x92\x91\x90a-\x1DV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x84\x83\x81Q\x81\x10a\x17\xE7Wa\x17\xE7a\x1D2V[` \x90\x81\x02\x91\x90\x91\x01\x01RP`\x01\x01a\x17\x0FV[P`@Q\x80`\xA0\x01`@R\x80`y\x81R` \x01a-g`y\x919`@Q\x80`@\x01`@R\x80`\x1F\x81R` \x01`\0\x80Q` a.\xC5\x839\x81Q\x91R\x81RP`@Q\x80``\x01`@R\x80`\"\x81R` \x01a.\xA3`\"\x919`@Q\x80`\xC0\x01`@R\x80`\x95\x81R` \x01a.\x0E`\x95\x919`@Q\x80``\x01`@R\x80`.\x81R` \x01a-\xE0`.\x919`@Q` \x01a\x18\x98\x95\x94\x93\x92\x91\x90a+\xCEV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x82`@Q` \x01a\x18\xBF\x91\x90a+\x98V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x87\x89`\xE0\x015\x8A`\xC0\x015a\x18\xE9\x8Ca\x0E\tV[`@\x80Q` \x81\x01\x97\x90\x97R\x86\x01\x94\x90\x94R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16``\x85\x01R`\x80\x84\x01R`\xA0\x83\x01R`\xC0\x82\x01R`\xE0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x94PPPPP\x92\x91PPV[`\0`@Q\x80``\x01`@R\x80`\"\x81R` \x01a.\xA3`\"\x919\x80Q` \x91\x82\x01 \x90a\x19o\x90\x84\x01\x84a\x1FvV[`@\x80Q` \x81\x81\x01\x94\x90\x94R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x90\x82\x01R\x90\x83\x015``\x82\x01R`\x80\x01[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[`@\x80Q\x80\x82\x01\x90\x91R`\x1F\x81R`\0\x80Q` a.\xC5\x839\x81Q\x91R` \x91\x82\x01R`\0\x90\x7F\\\x84\xFBFu\x90\xFC=W\xE4\xDD\x07\xBF\xDF\xFA\xFAc\xF0)\xF4n\x07\xFF\xEE\xE8X\xBD\xE2\xDD/\xA3\x1F\x90a\x1A\t\x90\x84\x01\x84a\x1FvV[a\x1A\x16` \x85\x01\x85a#\xAAV[`@Qa\x1A$\x92\x91\x90a-DV[`@Q\x90\x81\x90\x03\x81 a\x19\x98\x93\x92\x91` \x01\x92\x83R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16` \x83\x01R`@\x82\x01R``\x01\x90V[`\0a\x1Ad` \x83\x01\x83a\x1FvV[\x90P6`\0a\x1Av` \x85\x01\x85a#\xAAV[\x90\x92P\x90P\x815` \x1B\x7F\xECt\x15\x14,\xB9\x89\"5T]S\x8AD.\xBB9\xAD79&\xCC\x06\x9D\xDB\xA3\x9E\x01\0\0\0\0c\xFF\xFF\xFF\xFF\x19\x82\x16\x01a\x1A\xB3W`\0\x80\xFD[`@Q\x82\x84\x827`\0\x80\x84\x83`\0\x89Z\xF1a\x1A\xD2W=`\0\x80>=`\0\xFD[PPPPPPV[`\0\x81`\x01\x81\x11\x15a\x1B\x8EW`\0`\0\x19\x82\x01\x81[\x81\x81\x10\x15a\x1B\x8AW\x86\x86\x82\x81\x81\x10a\x1B\tWa\x1B\ta\x1D2V[a\x1B\x1F\x92` `@\x90\x92\x02\x01\x90\x81\x01\x91Pa\x1FvV[\x92P`\x01\x01\x80[\x84\x81\x10\x15a\x1B\x84W\x87\x87\x82\x81\x81\x10a\x1B@Wa\x1B@a\x1D2V[a\x1BV\x92` `@\x90\x92\x02\x01\x90\x81\x01\x91Pa\x1FvV[`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x03a\x1B|W`\x01\x95PPPPPPa\x03hV[`\x01\x01a\x1B&V[Pa\x1A\xEFV[PPP[P`\0\x93\x92PPPV[`\0a\x1B\xAD`\x01`\x01`\xA0\x1B\x03\x84\x16\x83a\x1B\xFBV[\x90P\x80Q`\0\x14\x15\x80\x15a\x1B\xD2WP\x80\x80` \x01\x90Q\x81\x01\x90a\x1B\xD0\x91\x90a#?V[\x15[\x15a\x123W`@QcRt\xAF\xE7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16`\x04\x82\x01R`$\x01a\x15\x93V[``a\x1C\t\x83\x83`\0a\x1C\x10V[\x93\x92PPPV[``\x81G\x10\x15a\x1C5W`@Qc\xCDx`Y`\xE0\x1B\x81R0`\x04\x82\x01R`$\x01a\x15\x93V[`\0\x80\x85`\x01`\x01`\xA0\x1B\x03\x16\x84\x86`@Qa\x1CQ\x91\x90a-TV[`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x1C\x8EW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x1C\x93V[``\x91P[P\x91P\x91Pa\x1C\xA3\x86\x83\x83a\x1C\xADV[\x96\x95PPPPPPV[``\x82a\x1C\xC2Wa\x1C\xBD\x82a\x1D\tV[a\x1C\tV[\x81Q\x15\x80\x15a\x1C\xD9WP`\x01`\x01`\xA0\x1B\x03\x84\x16;\x15[\x15a\x1D\x02W`@Qc\x99\x96\xB3\x15`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`$\x01a\x15\x93V[P\x80a\x1C\tV[\x80Q\x15a\x1D\x19W\x80Q\x80\x82` \x01\xFD[`@Qc\n\x12\xF5!`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15a\x1DZW`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1DpW`\0\x80\xFD[\x82\x01a\x01@\x81\x85\x03\x12\x15a\x1C\tW`\0\x80\xFD[`\0`@\x82\x84\x03\x12\x15a\x1D\x95W`\0\x80\xFD[P\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x1D\xADW`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1D\xC3W`\0\x80\xFD[a\x1D\xCF\x84\x82\x85\x01a\x1D\x83V[\x94\x93PPPPV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\r\\W`\0\x80\xFD[`\0\x80\x83`\x1F\x84\x01\x12a\x1E\0W`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1E\x17W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\x1E/W`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x80``\x85\x87\x03\x12\x15a\x1ELW`\0\x80\xFD[\x845`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x1EcW`\0\x80\xFD[a\x1Eo\x88\x83\x89\x01a\x1D\x83V[\x95Pa\x1E}` \x88\x01a\x1D\xD7V[\x94P`@\x87\x015\x91P\x80\x82\x11\x15a\x1E\x93W`\0\x80\xFD[Pa\x1E\xA0\x87\x82\x88\x01a\x1D\xEEV[\x95\x98\x94\x97P\x95PPPPV[`\0\x80`\0\x80`\0``\x86\x88\x03\x12\x15a\x1E\xC4W`\0\x80\xFD[\x855`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x1E\xDBW`\0\x80\xFD[\x81\x88\x01\x91P\x88`\x1F\x83\x01\x12a\x1E\xEFW`\0\x80\xFD[\x815\x81\x81\x11\x15a\x1E\xFEW`\0\x80\xFD[\x89` \x82`\x05\x1B\x85\x01\x01\x11\x15a\x1F\x13W`\0\x80\xFD[` \x83\x01\x97P\x80\x96PPa\x1F)` \x89\x01a\x1D\xD7V[\x94P`@\x88\x015\x91P\x80\x82\x11\x15a\x1F?W`\0\x80\xFD[Pa\x1FL\x88\x82\x89\x01a\x1D\xEEV[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x1FoW`\0\x80\xFD[P5\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x1F\x88W`\0\x80\xFD[a\x1C\t\x82a\x1D\xD7V[`\0\x80`@\x83\x85\x03\x12\x15a\x1F\xA4W`\0\x80\xFD[a\x1F\xAD\x83a\x1D\xD7V[\x94` \x93\x90\x93\x015\x93PPPV[`\0\x825a\x01>\x19\x836\x03\x01\x81\x12a\x1F\xD2W`\0\x80\xFD[\x91\x90\x91\x01\x92\x91PPV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a\x1F\xF3W`\0\x80\xFD[\x83\x01` \x81\x01\x92P5\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15a \x12W`\0\x80\xFD[\x80`\x06\x1B6\x03\x82\x13\x15a\x1E/W`\0\x80\xFD[`\x01`\x01`\xA0\x1B\x03a 5\x82a\x1D\xD7V[\x16\x82R` \x90\x81\x015\x91\x01RV[\x81\x83R` \x83\x01\x92P`\0\x81`\0[\x84\x81\x10\x15a wWa d\x86\x83a $V[`@\x95\x86\x01\x95\x91\x90\x91\x01\x90`\x01\x01a RV[P\x93\x94\x93PPPPV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a \x98W`\0\x80\xFD[\x83\x01` \x81\x01\x92P5\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15a \xB7W`\0\x80\xFD[\x80`\x05\x1B6\x03\x82\x13\x15a\x1E/W`\0\x80\xFD[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a \xE0W`\0\x80\xFD[\x83\x01` \x81\x01\x92P5\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15a \xFFW`\0\x80\xFD[\x806\x03\x82\x13\x15a\x1E/W`\0\x80\xFD[\x81\x83R\x81\x81` \x85\x017P`\0\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[`\0\x83\x83\x85R` \x80\x86\x01\x95P\x80\x85`\x05\x1B\x83\x01\x01\x84`\0\x80[\x88\x81\x10\x15a!\xC3W\x85\x84\x03`\x1F\x19\x01\x8AR\x8256\x89\x90\x03`>\x19\x01\x81\x12a!vW\x82\x83\xFD[\x88\x01`@`\x01`\x01`\xA0\x1B\x03a!\x8B\x83a\x1D\xD7V[\x16\x86Ra!\x9A\x87\x83\x01\x83a \xC9V[\x92P\x81\x88\x88\x01Ra!\xAE\x82\x88\x01\x84\x83a!\x0EV[\x9C\x88\x01\x9C\x96PPP\x92\x85\x01\x92P`\x01\x01a!QV[P\x91\x98\x97PPPPPPPPV[`\0a\x01@a!\xF0\x84a!\xE3\x85a\x1D\xD7V[`\x01`\x01`\xA0\x1B\x03\x16\x90RV[a!\xFC` \x84\x01a\x1D\xD7V[`\x01`\x01`\xA0\x1B\x03\x16` \x85\x01Ra\"\x16`@\x84\x01a\x1D\xD7V[`\x01`\x01`\xA0\x1B\x03\x16`@\x85\x01Ra\"1``\x84\x01\x84a\x1F\xDCV[\x82``\x87\x01Ra\"D\x83\x87\x01\x82\x84a CV[\x92PPPa\"X`\x80\x85\x01`\x80\x85\x01a $V[`\xC0\x83\x015`\xC0\x85\x01R`\xE0\x83\x015`\xE0\x85\x01Ra\x01\0a\"{\x81\x85\x01\x85a \x81V[\x86\x84\x03\x83\x88\x01Ra\"\x8D\x84\x82\x84a!7V[\x93PPPPa\x01 a\"\xA1\x81\x85\x01\x85a \x81V[\x86\x84\x03\x83\x88\x01Ra\"\xB3\x84\x82\x84a!7V[\x97\x96PPPPPPPV[` \x81R`\0\x825a\x01>\x19\x846\x03\x01\x81\x12a\"\xD9W`\0\x80\xFD[`@` \x84\x01Ra\"\xEF``\x84\x01\x85\x83\x01a!\xD1V[\x90Pa\"\xFE` \x85\x01\x85a \xC9V[\x84\x83\x03`\x1F\x19\x01`@\x86\x01Ra\x1C\xA3\x83\x82\x84a!\x0EV[`@\x81R`\0a#(`@\x83\x01\x85a!\xD1V[\x90P`\x01\x80`\xA0\x1B\x03\x83\x16` \x83\x01R\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a#QW`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x1C\tW`\0\x80\xFD[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a#xW`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15a#\x92W`\0\x80\xFD[` \x01\x91P`\x05\x81\x90\x1B6\x03\x82\x13\x15a\x1E/W`\0\x80\xFD[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a#\xC1W`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15a#\xDBW`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a\x1E/W`\0\x80\xFD[c\xFF\xFF\xFF\xFF\x19\x86\x16\x81R`\x80` \x82\x01R`\0a$\x10`\x80\x83\x01\x87a!\xD1V[`\x01`\x01`\xA0\x1B\x03\x86\x16`@\x84\x01R\x82\x81\x03``\x84\x01Ra$2\x81\x85\x87a!\x0EV[\x98\x97PPPPPPPPV[`\0` \x82\x84\x03\x12\x15a$PW`\0\x80\xFD[PQ\x91\x90PV[`@\x81R`\0a$j`@\x83\x01\x85a!\xD1V[\x90P\x82` \x83\x01R\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0\x825`>\x19\x836\x03\x01\x81\x12a\x1F\xD2W`\0\x80\xFD[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a$\xC7Wa$\xC7a$yV[`@R\x90V[`@Qa\x01 \x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a$\xC7Wa$\xC7a$yV[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a%\x18Wa%\x18a$yV[`@R\x91\x90PV[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15a%9Wa%9a$yV[P`\x05\x1B` \x01\x90V[`\0`@\x82\x84\x03\x12\x15a%UW`\0\x80\xFD[a%]a$\xA5V[\x90Pa%h\x82a\x1D\xD7V[\x81R` \x82\x015` \x82\x01R\x92\x91PPV[`\0\x82`\x1F\x83\x01\x12a%\x8BW`\0\x80\xFD[\x815` a%\xA0a%\x9B\x83a% V[a$\xF0V[\x80\x83\x82R` \x82\x01\x91P` \x84`\x06\x1B\x87\x01\x01\x93P\x86\x84\x11\x15a%\xC2W`\0\x80\xFD[` \x86\x01[\x84\x81\x10\x15a%\xE7Wa%\xD9\x88\x82a%CV[\x83R\x91\x83\x01\x91`@\x01a%\xC7V[P\x96\x95PPPPPPV[`\0`\x1F\x83`\x1F\x84\x01\x12a&\x05W`\0\x80\xFD[\x825` a&\x15a%\x9B\x83a% V[\x82\x81R`\x05\x92\x90\x92\x1B\x85\x01\x81\x01\x91\x81\x81\x01\x90\x87\x84\x11\x15a&4W`\0\x80\xFD[\x82\x87\x01[\x84\x81\x10\x15a'\x13W\x805`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a&XW`\0\x80\x81\xFD[\x90\x89\x01\x90`@`\x1F\x19\x83\x8D\x03\x81\x01\x82\x13\x15a&sW`\0\x80\x81\xFD[a&{a$\xA5V[a&\x86\x89\x86\x01a\x1D\xD7V[\x81R\x82\x85\x015\x84\x81\x11\x15a&\x9AW`\0\x80\x81\xFD[\x80\x86\x01\x95PP\x8D`?\x86\x01\x12a&\xB0W`\0\x80\x81\xFD[\x88\x85\x015\x84\x81\x11\x15a&\xC4Wa&\xC4a$yV[a&\xD3\x8A\x84\x8E\x84\x01\x16\x01a$\xF0V[\x94P\x80\x85R\x8E\x84\x82\x88\x01\x01\x11\x15a&\xECW`\0\x92P\x82\x83\xFD[\x80\x84\x87\x01\x8B\x87\x017`\0\x90\x85\x01\x8A\x01R\x80\x89\x01\x93\x90\x93RPP\x84RP\x91\x83\x01\x91\x83\x01a&8V[P\x97\x96PPPPPPPV[`\0a\x01@\x826\x03\x12\x15a'2W`\0\x80\xFD[a':a$\xCDV[a'C\x83a\x1D\xD7V[\x81Ra'Q` \x84\x01a\x1D\xD7V[` \x82\x01Ra'b`@\x84\x01a\x1D\xD7V[`@\x82\x01R``\x83\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a'\x81W`\0\x80\xFD[a'\x8D6\x83\x87\x01a%zV[``\x84\x01Ra'\x9F6`\x80\x87\x01a%CV[`\x80\x84\x01R`\xC0\x85\x015`\xA0\x84\x01R`\xE0\x85\x015`\xC0\x84\x01Ra\x01\0\x91P\x81\x85\x015\x81\x81\x11\x15a'\xCEW`\0\x80\xFD[a'\xDA6\x82\x88\x01a%\xF2V[`\xE0\x85\x01RPa\x01 \x85\x015\x81\x81\x11\x15a'\xF3W`\0\x80\xFD[a'\xFF6\x82\x88\x01a%\xF2V[\x83\x85\x01RPPP\x80\x91PP\x91\x90PV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P` \x84\x01`\0[\x83\x81\x10\x15a(]Wa(J\x87\x83Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x82R` \x90\x81\x01Q\x91\x01RV[`@\x96\x90\x96\x01\x95\x90\x82\x01\x90`\x01\x01a($V[P\x94\x95\x94PPPPPV[`\0[\x83\x81\x10\x15a(\x83W\x81\x81\x01Q\x83\x82\x01R` \x01a(kV[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra(\xA4\x81` \x86\x01` \x86\x01a(hV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[`\0\x82\x82Q\x80\x85R` \x80\x86\x01\x95P\x80\x82`\x05\x1B\x84\x01\x01\x81\x86\x01`\0[\x84\x81\x10\x15a) W\x85\x83\x03`\x1F\x19\x01\x89R\x81Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x84R\x84\x01Q`@\x85\x85\x01\x81\x90Ra)\x0C\x81\x86\x01\x83a(\x8CV[\x9A\x86\x01\x9A\x94PPP\x90\x83\x01\x90`\x01\x01a(\xD5V[P\x90\x97\x96PPPPPPPV[`\0`\x80\x80\x83\x01c\xFF\xFF\xFF\xFF\x19\x89\x16\x84R` `\x80\x81\x86\x01R\x81\x89Q\x80\x84R`\xA0\x93P`\xA0\x87\x01\x91P`\xA0\x81`\x05\x1B\x88\x01\x01\x83\x8C\x01`\0[\x83\x81\x10\x15a*RW\x89\x83\x03`\x9F\x19\x01\x85R\x81Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x84Ra\x01@\x81\x88\x01Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x86\x8A\x01RP`@\x82\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x87\x83\x01RPP``\x80\x83\x01Q\x82\x82\x88\x01Ra)\xC8\x83\x88\x01\x82a(\x0FV[\x92PPP\x89\x82\x01Qa)\xEF\x8B\x87\x01\x82\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x82R` \x90\x81\x01Q\x91\x01RV[P\x81\x89\x01Q`\xC0\x86\x81\x01\x91\x90\x91R\x82\x01Q`\xE0\x80\x87\x01\x91\x90\x91R\x82\x01Q\x85\x82\x03a\x01\0\x80\x88\x01\x91\x90\x91Ra*#\x83\x83a(\xB8V[\x93\x01Q\x86\x84\x03a\x01 \x88\x01R\x92\x91Pa*>\x90P\x81\x83a(\xB8V[\x96\x88\x01\x96\x94PPP\x90\x85\x01\x90`\x01\x01a)eV[PP`\x01`\x01`\xA0\x1B\x03\x8B\x16`@\x89\x01R\x87\x81\x03``\x89\x01Ra*v\x81\x8A\x8Ca!\x0EV[\x9D\x9CPPPPPPPPPPPPPV[`\0` \x80\x83\x85\x03\x12\x15a*\x9AW`\0\x80\xFD[\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15a*\xB0W`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a*\xC1W`\0\x80\xFD[\x80Qa*\xCFa%\x9B\x82a% V[\x81\x81R`\x05\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x87\x83\x11\x15a*\xEEW`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15a\"\xB3W\x83Q\x82R\x92\x84\x01\x92\x90\x84\x01\x90a*\xF3V[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a+#W`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15a+=W`\0\x80\xFD[` \x01\x91P`\x06\x81\x90\x1B6\x03\x82\x13\x15a\x1E/W`\0\x80\xFD[`\0\x84Qa+g\x81\x84` \x89\x01a(hV[\x84Q\x90\x83\x01\x90a+{\x81\x83` \x89\x01a(hV[\x84Q\x91\x01\x90a+\x8E\x81\x83` \x88\x01a(hV[\x01\x95\x94PPPPPV[\x81Q`\0\x90\x82\x90` \x80\x86\x01\x84[\x83\x81\x10\x15a+\xC2W\x81Q\x85R\x93\x82\x01\x93\x90\x82\x01\x90`\x01\x01a+\xA6V[P\x92\x96\x95PPPPPPV[`\0\x86Qa+\xE0\x81\x84` \x8B\x01a(hV[\x86Q\x90\x83\x01\x90a+\xF4\x81\x83` \x8B\x01a(hV[\x86Q\x91\x01\x90a,\x07\x81\x83` \x8A\x01a(hV[\x85Q\x91\x01\x90a,\x1A\x81\x83` \x89\x01a(hV[\x84Q\x91\x01\x90a,-\x81\x83` \x88\x01a(hV[\x01\x97\x96PPPPPPPV[`\xC0\x81R`\0a\x01 \x82\x01\x89Q```\xC0\x85\x01R\x81\x81Q\x80\x84Ra\x01@\x86\x01\x91P` \x93P` \x83\x01\x92P`\0[\x81\x81\x10\x15a,\xA0Wa,\x8D\x83\x85Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x82R` \x90\x81\x01Q\x91\x01RV[\x92\x84\x01\x92`@\x92\x90\x92\x01\x91`\x01\x01a,gV[PP` \x8C\x01Q`\xE0\x86\x01R`@\x8C\x01Qa\x01\0\x86\x01R\x84\x81\x03` \x86\x01Ra,\xC9\x81\x8Ca(\x0FV[\x92PPPa,\xE2`@\x84\x01\x89`\x01`\x01`\xA0\x1B\x03\x16\x90RV[\x86``\x84\x01R\x82\x81\x03`\x80\x84\x01Ra,\xFA\x81\x87a(\x8CV[\x90P\x82\x81\x03`\xA0\x84\x01Ra-\x0F\x81\x85\x87a!\x0EV[\x9A\x99PPPPPPPPPPV[\x82\x81R``\x81\x01a\x1C\t` \x83\x01\x84\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x82R` \x90\x81\x01Q\x91\x01RV[\x81\x83\x827`\0\x91\x01\x90\x81R\x91\x90PV[`\0\x82Qa\x1F\xD2\x81\x84` \x87\x01a(hV\xFEPermitBatchWitnessTransferFrom(TokenPermissions[] permitted,address spender,uint256 nonce,uint256 deadline,Order witness)TokenPermissions(address token,uint256 amount)Order(address offerer,address zone,address recipient,Item[] offer,Item consideration,uint256 deadline,uint256 nonce,Hook[] preHooks,Hook[] postHooks)Item(address token,uint256 amount)Hook(address target,bytes data)\0\xA2dipfsX\"\x12 tne~^\xB3%\x0C0\xFCK. \x02+_\xFA\xE2m\xD7x\xFD\t[\x84T\x0B\x82\x06\x0F\x85adsolcC\0\x08\x17\x003";
    /// The bytecode of the contract.
    pub static FLOODPLAIN_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\0\xABW`\x005`\xE0\x1C\x80ck\"L\xCB\x11a\0dW\x80ck\"L\xCB\x14a\x02NW\x80cv&\x96c\x14a\x02nW\x80cxeJR\x14a\x02\x8EW\x80c\x9DH\x1Bf\x14a\x02\xAEW\x80c\xAF^\x7F5\x14a\x02\xCEW\x80c\xE9\xBA\x1E\x97\x14a\x02\xFEWa\0\xB2V[\x80c=\xD9\x08\xFC\x14a\x01NW\x80cA\xBA\xC9S\x14a\x01\x81W\x80cN\xE3\x99_\x14a\x01\xA1W\x80cO\xF6\x86\t\x14a\x01\xC1W\x80ci`J\xAD\x14a\x01\xEFW\x80cj\xFD\xD8P\x14a\x02\x02Wa\0\xB2V[6a\0\xB2W\0[4\x80\x15a\0\xBEW`\0\x80\xFD[P`\0\x80a\0\xCC`\x01a\x03\x1EV[\x91P\x91P`\0\x80\x83\x81T\x81\x10a\0\xE4Wa\0\xE4a\x1D2V[`\0\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x90P6\x82\x81\x03\x90\x81\x11\x15a\x01\x0CW`\0\x80\xFD[\x80\x83`\x007`\0\x80\x82`\0\x85Z\xFA\x90P=`\0\x80>\x80a\x01+W=`\0\xFD[`\0\x80=`\x000Z\xF4\x90P=`\0\x80>\x80\x80\x15a\x01GW=`\0\xF3[=`\0\xFD[\0[4\x80\x15a\x01ZW`\0\x80\xFD[Pa\x01na\x01i6`\x04a\x1DHV[a\x03]V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01\x8DW`\0\x80\xFD[Pa\x01La\x01\x9C6`\x04a\x1D\x9BV[a\x03nV[4\x80\x15a\x01\xADW`\0\x80\xFD[Pa\x01La\x01\xBC6`\x04a\x1E6V[a\x03\xC0V[4\x80\x15a\x01\xCDW`\0\x80\xFD[Pa\x01\xD6`\0\x81V[`@Q`\x01`\x01`\xF8\x1B\x03\x19\x90\x91\x16\x81R` \x01a\x01xV[a\x01La\x01\xFD6`\x04a\x1D\x9BV[a\x06cV[4\x80\x15a\x02\x0EW`\0\x80\xFD[Pa\x026\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01xV[4\x80\x15a\x02ZW`\0\x80\xFD[Pa\x01na\x02i6`\x04a\x1DHV[a\x08,V[4\x80\x15a\x02zW`\0\x80\xFD[Pa\x01La\x02\x896`\x04a\x1E\xACV[a\x088V[4\x80\x15a\x02\x9AW`\0\x80\xFD[Pa\x026a\x02\xA96`\x04a\x1F]V[a\x0CpV[4\x80\x15a\x02\xBAW`\0\x80\xFD[Pa\x01na\x02\xC96`\x04a\x1FvV[a\x0C\x9AV[4\x80\x15a\x02\xDAW`\0\x80\xFD[Pa\x02\xEEa\x02\xE96`\x04a\x1DHV[a\r\x17V[`@Q\x90\x15\x15\x81R` \x01a\x01xV[4\x80\x15a\x03\nW`\0\x80\xFD[Pa\x02\xEEa\x03\x196`\x04a\x1F\x91V[a\raV[`\0\x80`\0[\x835`\0\x1A`\x7F\x81\x16\x82\x1B\x84\x17\x93P`\x01\x85\x01\x94P\x80`\x07\x1C\x15a\x03HWPa\x03NV[Pa\x03VV[`\x07\x01a\x03$V[P\x90\x92\x90PV[`\0a\x03h\x82a\x0E\tV[\x92\x91PPV[`\0a\x03\x82a\x03}\x83\x80a\x1F\xBBV[a\x0E\tV[\x90P\x80\x7F\xED\xF4\x9C\0=\xE6)\x8FV\x1A\x96\x8A\xDE5\xBD=\xCF\x06\xFF\xA1iy\xDE7]\xBB9x-o\xCB\xE4\x83`@Qa\x03\xB4\x91\x90a\"\xBEV[`@Q\x80\x91\x03\x90\xA2PPV[a\x03\xC8a\x11\xCAV[6a\x03\xD3\x85\x80a\x1F\xBBV[\x90P`\0a\x03\xE0\x82a\x0E\tV[\x90P`\0a\x03\xF4`@\x84\x01` \x85\x01a\x1FvV[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x04\x9DWa\x04\x12`@\x83\x01` \x84\x01a\x1FvV[`\x01`\x01`\xA0\x1B\x03\x16c\xADs\xD6\x89\x833`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x04?\x92\x91\x90a#\x15V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\\W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\x80\x91\x90a#?V[a\x04\x9DW`@Qc\x03\x12(W`\xE5\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x04\xB3a\x04\xAEa\x01\0\x84\x01\x84a#aV[a\x11\xF4V[a\x04\xCB\x82a\x04\xC4` \x89\x01\x89a#\xAAV[\x843a\x128V[`@Qc\x9E_\x81\xF5`\xE0\x1B\x81R`\0\x90`\x01`\x01`\xA0\x1B\x03\x87\x16\x90c\x9E_\x81\xF5\x90a\x05\"\x90\x7F\x13\x8B\xEA\xEB\xD3Fv\xDD\xCA\xAB\xA2\xACu\xBB\xD1D\xC6R\xC8\xC6\xD93\xF9b$\\a\xFF\xEF\x90\xD7-\x90\x87\x903\x90\x8B\x90\x8B\x90`\x04\x01a#\xF0V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x05AW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05e\x91\x90a$>V[\x90P`\xA0\x83\x015\x81\x10\x15a\x05\x8CW`@QcH\x87\x9A\t`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x05\x9E`\xA0\x85\x01`\x80\x86\x01a\x1FvV[\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16a\x05\xD6Wa\x05\xD1\x82a\x05\xC2``\x87\x01`@\x88\x01a\x1FvV[`\x01`\x01`\xA0\x1B\x03\x16\x90a\x15tV[a\x05\xFCV[a\x05\xFC\x87a\x05\xEA``\x87\x01`@\x88\x01a\x1FvV[`\x01`\x01`\xA0\x1B\x03\x84\x16\x91\x90\x85a\x16\x10V[a\x06\ra\x04\xAEa\x01 \x86\x01\x86a#aV[3`\x01`\x01`\xA0\x1B\x03\x16\x7F6b\xC2\xB9tk\xA3\xF8}\xEFd@\x10\xA3\xFA\x9E\x032\xDAS\xA5w\x17\x94\xC7\x9D{l\xCE$\xD9+\x85\x84`@Qa\x06H\x92\x91\x90a$WV[`@Q\x80\x91\x03\x90\xA2PPPPa\x06]`\x01\x80UV[PPPPV[a\x06ka\x11\xCAV[6a\x06v\x82\x80a\x1F\xBBV[\x90P`\0a\x06\x83\x82a\x0E\tV[\x90P`\0a\x06\x97`@\x84\x01` \x85\x01a\x1FvV[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x07@Wa\x06\xB5`@\x83\x01` \x84\x01a\x1FvV[`\x01`\x01`\xA0\x1B\x03\x16c\xADs\xD6\x89\x833`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x06\xE2\x92\x91\x90a#\x15V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06\xFFW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07#\x91\x90a#?V[a\x07@W`@Qc\x03\x12(W`\xE5\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x07Qa\x04\xAEa\x01\0\x84\x01\x84a#aV[a\x07b\x82a\x04\xC4` \x86\x01\x86a#\xAAV[`\0a\x07t`\xA0\x84\x01`\x80\x85\x01a\x1FvV[\x90P`\xA0\x83\x015`\x01`\x01`\xA0\x1B\x03\x82\x16a\x07\xA2Wa\x07\x9D\x81a\x05\xC2``\x87\x01`@\x88\x01a\x1FvV[a\x07\xC8V[a\x07\xC83a\x07\xB6``\x87\x01`@\x88\x01a\x1FvV[`\x01`\x01`\xA0\x1B\x03\x85\x16\x91\x90\x84a\x16\x10V[a\x07\xD9a\x04\xAEa\x01 \x86\x01\x86a#aV[3`\x01`\x01`\xA0\x1B\x03\x16\x7F6b\xC2\xB9tk\xA3\xF8}\xEFd@\x10\xA3\xFA\x9E\x032\xDAS\xA5w\x17\x94\xC7\x9D{l\xCE$\xD9+\x85\x83`@Qa\x08\x14\x92\x91\x90a$WV[`@Q\x80\x91\x03\x90\xA2PPPPa\x08)`\x01\x80UV[PV[`\0a\x03h\x820a\x16jV[a\x08@a\x11\xCAV[`\0\x84`\x01`\x01`@\x1B\x03\x81\x11\x15a\x08ZWa\x08Za$yV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x08\xE8W\x81` \x01[a\x08\xD5`@\x80Qa\x01 \x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x90R\x82\x84\x01\x82\x90R``\x80\x84\x01R\x83Q\x80\x85\x01\x90\x94R\x81\x84R\x83\x01R\x90`\x80\x82\x01\x90\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01``\x81R` \x01``\x81RP\x90V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x08xW\x90P[P\x90P`\0[\x85\x81\x10\x15a\ngW6\x87\x87\x83\x81\x81\x10a\t\tWa\t\ta\x1D2V[\x90P` \x02\x81\x01\x90a\t\x1B\x91\x90a$\x8FV[a\t%\x90\x80a\x1F\xBBV[\x90P`\0a\t9`@\x83\x01` \x84\x01a\x1FvV[`\x01`\x01`\xA0\x1B\x03\x16\x14a\t\xE2Wa\tW`@\x82\x01` \x83\x01a\x1FvV[`\x01`\x01`\xA0\x1B\x03\x16c\xADs\xD6\x89\x82\x88`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\t\x84\x92\x91\x90a#\x15V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t\xA1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\xC5\x91\x90a#?V[a\t\xE2W`@Qc\x03\x12(W`\xE5\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\t\xF3a\x04\xAEa\x01\0\x83\x01\x83a#aV[a\n8\x81\x89\x89\x85\x81\x81\x10a\n\tWa\n\ta\x1D2V[\x90P` \x02\x81\x01\x90a\n\x1B\x91\x90a$\x8FV[a\n)\x90` \x81\x01\x90a#\xAAV[a\n2\x85a\x0E\tV[\x8Aa\x128V[a\nA\x81a'\x1FV[\x83\x83\x81Q\x81\x10a\nSWa\nSa\x1D2V[` \x90\x81\x02\x91\x90\x91\x01\x01RP`\x01\x01a\x08\xEEV[P`@Qc\x9A\xD81\xA1`\xE0\x1B\x81R`\0\x90`\x01`\x01`\xA0\x1B\x03\x86\x16\x90c\x9A\xD81\xA1\x90a\n\xBF\x90\x7F\x13\x8B\xEA\xEB\xD3Fv\xDD\xCA\xAB\xA2\xACu\xBB\xD1D\xC6R\xC8\xC6\xD93\xF9b$\\a\xFF\xEF\x90\xD7-\x90\x86\x903\x90\x8A\x90\x8A\x90`\x04\x01a)-V[`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\n\xDEW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x0B\x06\x91\x90\x81\x01\x90a*\x87V[\x80Q\x90\x91P\x86\x14a\x0B*W`@QcQ%\t\xD3`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0[\x86\x81\x10\x15a\x0C]W6\x88\x88\x83\x81\x81\x10a\x0BHWa\x0BHa\x1D2V[\x90P` \x02\x81\x01\x90a\x0BZ\x91\x90a$\x8FV[a\x0Bd\x90\x80a\x1F\xBBV[\x90P`\0\x83\x83\x81Q\x81\x10a\x0BzWa\x0Bza\x1D2V[` \x02` \x01\x01Q\x90P\x81`\x80\x01` \x015\x81\x10\x15a\x0B\xACW`@QcH\x87\x9A\t`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x0B\xBE`\xA0\x84\x01`\x80\x85\x01a\x1FvV[\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16a\x0B\xE7Wa\x0B\xE2\x82a\x05\xC2``\x86\x01`@\x87\x01a\x1FvV[a\x0B\xFBV[a\x0B\xFB\x89a\x05\xEA``\x86\x01`@\x87\x01a\x1FvV[a\x0C\x0Ca\x04\xAEa\x01 \x85\x01\x85a#aV[\x88`\x01`\x01`\xA0\x1B\x03\x16\x7F6b\xC2\xB9tk\xA3\xF8}\xEFd@\x10\xA3\xFA\x9E\x032\xDAS\xA5w\x17\x94\xC7\x9D{l\xCE$\xD9+\x84\x84`@Qa\x0CG\x92\x91\x90a$WV[`@Q\x80\x91\x03\x90\xA2PPP\x80`\x01\x01\x90Pa\x0B-V[PPPa\x0Ci`\x01\x80UV[PPPPPV[`\0\x81\x81T\x81\x10a\x0C\x80W`\0\x80\xFD[`\0\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x90P\x81V[`\0\x80T`\x01\x81\x01\x82U\x81\x80R\x7F)\r\xEC\xD9T\x8Bb\xA8\xD6\x03E\xA9\x888o\xC8K\xA6\xBC\x95H@\x08\xF66/\x93\x16\x0E\xF3\xE5c\x81\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x85\x16\x90\x81\x17\x90\x91U`@Q\x91\x92\x90\x91\x83\x91\x7FN\x9F\xEF\xD4\xC8\xC2e\xAD\xAD\x06\xDE\x04*\xD2\"D\x11e0n\x8A\xC2>\xA5%\xDE\xE3?@F>d\x91\xA3\x91\x90PV[`\0\x81`\xC0\x015B\x11\x15a\r-WP`\0\x91\x90PV[a\rGa\r=` \x84\x01\x84a\x1FvV[\x83`\xE0\x015a\raV[\x15a\rTWP`\x01\x91\x90PV[P`\0\x91\x90PV[\x91\x90PV[`@Qc\x13\xF8\n\xD1`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x04\x83\x01R`\x08\x83\x90\x1C`$\x83\x01R`\0\x91`\x01`\xFF\x85\x16\x1B\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90cO\xE0+D\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r\xDCW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E\0\x91\x90a$>V[\x16\x15\x93\x92PPPV[`\0\x80a\x0E\x19``\x84\x01\x84a+\x0CV[\x90P\x90P`\0\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0E7Wa\x0E7a$yV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0E`W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x82\x81\x10\x15a\x0E\xBEWa\x0E\x99a\x0E~``\x87\x01\x87a+\x0CV[\x83\x81\x81\x10a\x0E\x8EWa\x0E\x8Ea\x1D2V[\x90P`@\x02\x01a\x19?V[\x82\x82\x81Q\x81\x10a\x0E\xABWa\x0E\xABa\x1D2V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x0EfV[Pa\x0E\xCDa\x01\0\x85\x01\x85a#aV[\x90P\x91P`\0\x82`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0E\xEBWa\x0E\xEBa$yV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0F\x14W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x83\x81\x10\x15a\x0F\x7FWa\x0FZa\x0F3a\x01\0\x88\x01\x88a#aV[\x83\x81\x81\x10a\x0FCWa\x0FCa\x1D2V[\x90P` \x02\x81\x01\x90a\x0FU\x91\x90a$\x8FV[a\x19\xB5V[\x82\x82\x81Q\x81\x10a\x0FlWa\x0Fla\x1D2V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x0F\x1AV[Pa\x0F\x8Ea\x01 \x86\x01\x86a#aV[\x90P\x92P`\0\x83`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0F\xACWa\x0F\xACa$yV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0F\xD5W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x84\x81\x10\x15a\x10\x19Wa\x0F\xF4a\x0F3a\x01 \x89\x01\x89a#aV[\x82\x82\x81Q\x81\x10a\x10\x06Wa\x10\x06a\x1D2V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x0F\xDBV[P`@Q\x80`\xC0\x01`@R\x80`\x95\x81R` \x01a.\x0E`\x95\x919`@Q\x80`@\x01`@R\x80`\x1F\x81R` \x01`\0\x80Q` a.\xC5\x839\x81Q\x91R\x81RP`@Q\x80``\x01`@R\x80`\"\x81R` \x01a.\xA3`\"\x919`@Q` \x01a\x10\x82\x93\x92\x91\x90a+UV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x80Q` \x91\x82\x01 \x90a\x10\xA7\x90\x88\x01\x88a\x1FvV[a\x10\xB7`@\x89\x01` \x8A\x01a\x1FvV[a\x10\xC7``\x8A\x01`@\x8B\x01a\x1FvV[\x86`@Q` \x01a\x10\xD8\x91\x90a+\x98V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 a\x10\xFA\x8B`\x80\x01a\x19?V[\x8B`\xC0\x015\x8C`\xE0\x015\x89`@Q` \x01a\x11\x15\x91\x90a+\x98V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x89`@Q` \x01a\x11<\x91\x90a+\x98V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x90\x83\x01\x9B\x90\x9BR`\x01`\x01`\xA0\x1B\x03\x99\x8A\x16\x90\x82\x01R\x96\x88\x16``\x88\x01R\x96\x90\x94\x16`\x80\x86\x01R`\xA0\x85\x01\x92\x90\x92R`\xC0\x84\x01R`\xE0\x83\x01Ra\x01\0\x82\x01Ra\x01 \x81\x01\x91\x90\x91Ra\x01@\x81\x01\x91\x90\x91Ra\x01`\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x94PPPPP\x91\x90PV[`\x02`\x01T\x03a\x11\xEDW`@Qc>\xE5\xAE\xB5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02`\x01UV[`\0[\x81\x81\x10\x15a\x123Wa\x12+\x83\x83\x83\x81\x81\x10a\x12\x14Wa\x12\x14a\x1D2V[\x90P` \x02\x81\x01\x90a\x12&\x91\x90a$\x8FV[a\x1AUV[`\x01\x01a\x11\xF7V[PPPV[6`\0a\x12H``\x88\x01\x88a+\x0CV[\x91P\x91Pa\x12V\x82\x82a\x1A\xDAV[\x15a\x12tW`@Qcva\xC1\xBB`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a\x12\x8EWa\x12\x8Ea$yV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x12\xD3W\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x12\xACW\x90P[P\x90P`\0\x82`\x01`\x01`@\x1B\x03\x81\x11\x15a\x12\xF0Wa\x12\xF0a$yV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x135W\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x13\x0EW\x90P[P\x90P`\0[\x83\x81\x10\x15a\x13\xF2W6\x85\x85\x83\x81\x81\x10a\x13VWa\x13Va\x1D2V[`@\x80Q\x80\x82\x01\x82R\x91\x02\x92\x90\x92\x01\x92P\x81\x90Pa\x13w` \x84\x01\x84a\x1FvV[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x82` \x015\x81RP\x84\x83\x81Q\x81\x10a\x13\x9FWa\x13\x9Fa\x1D2V[` \x02` \x01\x01\x81\x90RP`@Q\x80`@\x01`@R\x80\x88`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x82` \x015\x81RP\x83\x83\x81Q\x81\x10a\x13\xDEWa\x13\xDEa\x1D2V[` \x90\x81\x02\x91\x90\x91\x01\x01RP`\x01\x01a\x13;V[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xFE\x8E\xC1\xA7`@Q\x80``\x01`@R\x80\x85\x81R` \x01\x8C`\xE0\x015\x81R` \x01\x8C`\xC0\x015\x81RP\x83\x8C`\0\x01` \x81\x01\x90a\x14Y\x91\x90a\x1FvV[\x8A`@Q\x80`@\x01`@R\x80`\x0E\x81R` \x01mOrder witness)`\x90\x1B\x81RP`@Q\x80`@\x01`@R\x80`\x1F\x81R` \x01`\0\x80Q` a.\xC5\x839\x81Q\x91R\x81RP`@Q\x80``\x01`@R\x80`\"\x81R` \x01a.\xA3`\"\x919`@Q\x80`\xC0\x01`@R\x80`\x95\x81R` \x01a.\x0E`\x95\x919`@Q\x80``\x01`@R\x80`.\x81R` \x01a-\xE0`.\x919`@Q` \x01a\x15\x04\x95\x94\x93\x92\x91\x90a+\xCEV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x8E\x8E`@Q\x88c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x157\x97\x96\x95\x94\x93\x92\x91\x90a,9V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x15QW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x15eW=`\0\x80>=`\0\xFD[PPPPPPPPPPPPPV[\x80G\x10\x15a\x15\x9CW`@Qc\xCDx`Y`\xE0\x1B\x81R0`\x04\x82\x01R`$\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x82`\x01`\x01`\xA0\x1B\x03\x16\x82`@Q`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x15\xE9W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x15\xEEV[``\x91P[PP\x90P\x80a\x123W`@Qc\n\x12\xF5!`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`$\x83\x01R\x84\x16`D\x82\x01R`d\x80\x82\x01\x84\x90R\x82Q\x80\x83\x03\x90\x91\x01\x81R`\x84\x90\x91\x01\x90\x91R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c#\xB8r\xDD`\xE0\x1B\x17\x90Ra\x06]\x90\x85\x90a\x1B\x98V[`\x006\x81a\x16{``\x86\x01\x86a+\x0CV[\x90\x92P\x90P`\0\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a\x16\x9AWa\x16\x9Aa$yV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x16\xC3W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0\x82`\x01`\x01`@\x1B\x03\x81\x11\x15a\x16\xE0Wa\x16\xE0a$yV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x17\tW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x83\x81\x10\x15a\x17\xFBW6\x85\x85\x83\x81\x81\x10a\x17*Wa\x17*a\x1D2V[\x90P`@\x02\x01\x90Pa\x17;\x81a\x19?V[\x83\x83\x81Q\x81\x10a\x17MWa\x17Ma\x1D2V[` \x02` \x01\x01\x81\x81RPP\x7Fa\x83X\xAC=\xB8\xDC'O\x0C\xD8\x82\x9D\xA7\xE24\xBDH\xCDs\xC4\xA7@\xAE\xDE\x1A\xDE\xC9\x84m\x06\xA1`@Q\x80`@\x01`@R\x80\x83`\0\x01` \x81\x01\x90a\x17\x98\x91\x90a\x1FvV[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x83` \x015\x81RP`@Q` \x01a\x17\xBF\x92\x91\x90a-\x1DV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x84\x83\x81Q\x81\x10a\x17\xE7Wa\x17\xE7a\x1D2V[` \x90\x81\x02\x91\x90\x91\x01\x01RP`\x01\x01a\x17\x0FV[P`@Q\x80`\xA0\x01`@R\x80`y\x81R` \x01a-g`y\x919`@Q\x80`@\x01`@R\x80`\x1F\x81R` \x01`\0\x80Q` a.\xC5\x839\x81Q\x91R\x81RP`@Q\x80``\x01`@R\x80`\"\x81R` \x01a.\xA3`\"\x919`@Q\x80`\xC0\x01`@R\x80`\x95\x81R` \x01a.\x0E`\x95\x919`@Q\x80``\x01`@R\x80`.\x81R` \x01a-\xE0`.\x919`@Q` \x01a\x18\x98\x95\x94\x93\x92\x91\x90a+\xCEV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x82`@Q` \x01a\x18\xBF\x91\x90a+\x98V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x87\x89`\xE0\x015\x8A`\xC0\x015a\x18\xE9\x8Ca\x0E\tV[`@\x80Q` \x81\x01\x97\x90\x97R\x86\x01\x94\x90\x94R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16``\x85\x01R`\x80\x84\x01R`\xA0\x83\x01R`\xC0\x82\x01R`\xE0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x94PPPPP\x92\x91PPV[`\0`@Q\x80``\x01`@R\x80`\"\x81R` \x01a.\xA3`\"\x919\x80Q` \x91\x82\x01 \x90a\x19o\x90\x84\x01\x84a\x1FvV[`@\x80Q` \x81\x81\x01\x94\x90\x94R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x90\x82\x01R\x90\x83\x015``\x82\x01R`\x80\x01[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[`@\x80Q\x80\x82\x01\x90\x91R`\x1F\x81R`\0\x80Q` a.\xC5\x839\x81Q\x91R` \x91\x82\x01R`\0\x90\x7F\\\x84\xFBFu\x90\xFC=W\xE4\xDD\x07\xBF\xDF\xFA\xFAc\xF0)\xF4n\x07\xFF\xEE\xE8X\xBD\xE2\xDD/\xA3\x1F\x90a\x1A\t\x90\x84\x01\x84a\x1FvV[a\x1A\x16` \x85\x01\x85a#\xAAV[`@Qa\x1A$\x92\x91\x90a-DV[`@Q\x90\x81\x90\x03\x81 a\x19\x98\x93\x92\x91` \x01\x92\x83R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16` \x83\x01R`@\x82\x01R``\x01\x90V[`\0a\x1Ad` \x83\x01\x83a\x1FvV[\x90P6`\0a\x1Av` \x85\x01\x85a#\xAAV[\x90\x92P\x90P\x815` \x1B\x7F\xECt\x15\x14,\xB9\x89\"5T]S\x8AD.\xBB9\xAD79&\xCC\x06\x9D\xDB\xA3\x9E\x01\0\0\0\0c\xFF\xFF\xFF\xFF\x19\x82\x16\x01a\x1A\xB3W`\0\x80\xFD[`@Q\x82\x84\x827`\0\x80\x84\x83`\0\x89Z\xF1a\x1A\xD2W=`\0\x80>=`\0\xFD[PPPPPPV[`\0\x81`\x01\x81\x11\x15a\x1B\x8EW`\0`\0\x19\x82\x01\x81[\x81\x81\x10\x15a\x1B\x8AW\x86\x86\x82\x81\x81\x10a\x1B\tWa\x1B\ta\x1D2V[a\x1B\x1F\x92` `@\x90\x92\x02\x01\x90\x81\x01\x91Pa\x1FvV[\x92P`\x01\x01\x80[\x84\x81\x10\x15a\x1B\x84W\x87\x87\x82\x81\x81\x10a\x1B@Wa\x1B@a\x1D2V[a\x1BV\x92` `@\x90\x92\x02\x01\x90\x81\x01\x91Pa\x1FvV[`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x03a\x1B|W`\x01\x95PPPPPPa\x03hV[`\x01\x01a\x1B&V[Pa\x1A\xEFV[PPP[P`\0\x93\x92PPPV[`\0a\x1B\xAD`\x01`\x01`\xA0\x1B\x03\x84\x16\x83a\x1B\xFBV[\x90P\x80Q`\0\x14\x15\x80\x15a\x1B\xD2WP\x80\x80` \x01\x90Q\x81\x01\x90a\x1B\xD0\x91\x90a#?V[\x15[\x15a\x123W`@QcRt\xAF\xE7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16`\x04\x82\x01R`$\x01a\x15\x93V[``a\x1C\t\x83\x83`\0a\x1C\x10V[\x93\x92PPPV[``\x81G\x10\x15a\x1C5W`@Qc\xCDx`Y`\xE0\x1B\x81R0`\x04\x82\x01R`$\x01a\x15\x93V[`\0\x80\x85`\x01`\x01`\xA0\x1B\x03\x16\x84\x86`@Qa\x1CQ\x91\x90a-TV[`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x1C\x8EW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x1C\x93V[``\x91P[P\x91P\x91Pa\x1C\xA3\x86\x83\x83a\x1C\xADV[\x96\x95PPPPPPV[``\x82a\x1C\xC2Wa\x1C\xBD\x82a\x1D\tV[a\x1C\tV[\x81Q\x15\x80\x15a\x1C\xD9WP`\x01`\x01`\xA0\x1B\x03\x84\x16;\x15[\x15a\x1D\x02W`@Qc\x99\x96\xB3\x15`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`$\x01a\x15\x93V[P\x80a\x1C\tV[\x80Q\x15a\x1D\x19W\x80Q\x80\x82` \x01\xFD[`@Qc\n\x12\xF5!`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15a\x1DZW`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1DpW`\0\x80\xFD[\x82\x01a\x01@\x81\x85\x03\x12\x15a\x1C\tW`\0\x80\xFD[`\0`@\x82\x84\x03\x12\x15a\x1D\x95W`\0\x80\xFD[P\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x1D\xADW`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1D\xC3W`\0\x80\xFD[a\x1D\xCF\x84\x82\x85\x01a\x1D\x83V[\x94\x93PPPPV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\r\\W`\0\x80\xFD[`\0\x80\x83`\x1F\x84\x01\x12a\x1E\0W`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1E\x17W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\x1E/W`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x80``\x85\x87\x03\x12\x15a\x1ELW`\0\x80\xFD[\x845`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x1EcW`\0\x80\xFD[a\x1Eo\x88\x83\x89\x01a\x1D\x83V[\x95Pa\x1E}` \x88\x01a\x1D\xD7V[\x94P`@\x87\x015\x91P\x80\x82\x11\x15a\x1E\x93W`\0\x80\xFD[Pa\x1E\xA0\x87\x82\x88\x01a\x1D\xEEV[\x95\x98\x94\x97P\x95PPPPV[`\0\x80`\0\x80`\0``\x86\x88\x03\x12\x15a\x1E\xC4W`\0\x80\xFD[\x855`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x1E\xDBW`\0\x80\xFD[\x81\x88\x01\x91P\x88`\x1F\x83\x01\x12a\x1E\xEFW`\0\x80\xFD[\x815\x81\x81\x11\x15a\x1E\xFEW`\0\x80\xFD[\x89` \x82`\x05\x1B\x85\x01\x01\x11\x15a\x1F\x13W`\0\x80\xFD[` \x83\x01\x97P\x80\x96PPa\x1F)` \x89\x01a\x1D\xD7V[\x94P`@\x88\x015\x91P\x80\x82\x11\x15a\x1F?W`\0\x80\xFD[Pa\x1FL\x88\x82\x89\x01a\x1D\xEEV[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x1FoW`\0\x80\xFD[P5\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x1F\x88W`\0\x80\xFD[a\x1C\t\x82a\x1D\xD7V[`\0\x80`@\x83\x85\x03\x12\x15a\x1F\xA4W`\0\x80\xFD[a\x1F\xAD\x83a\x1D\xD7V[\x94` \x93\x90\x93\x015\x93PPPV[`\0\x825a\x01>\x19\x836\x03\x01\x81\x12a\x1F\xD2W`\0\x80\xFD[\x91\x90\x91\x01\x92\x91PPV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a\x1F\xF3W`\0\x80\xFD[\x83\x01` \x81\x01\x92P5\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15a \x12W`\0\x80\xFD[\x80`\x06\x1B6\x03\x82\x13\x15a\x1E/W`\0\x80\xFD[`\x01`\x01`\xA0\x1B\x03a 5\x82a\x1D\xD7V[\x16\x82R` \x90\x81\x015\x91\x01RV[\x81\x83R` \x83\x01\x92P`\0\x81`\0[\x84\x81\x10\x15a wWa d\x86\x83a $V[`@\x95\x86\x01\x95\x91\x90\x91\x01\x90`\x01\x01a RV[P\x93\x94\x93PPPPV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a \x98W`\0\x80\xFD[\x83\x01` \x81\x01\x92P5\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15a \xB7W`\0\x80\xFD[\x80`\x05\x1B6\x03\x82\x13\x15a\x1E/W`\0\x80\xFD[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a \xE0W`\0\x80\xFD[\x83\x01` \x81\x01\x92P5\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15a \xFFW`\0\x80\xFD[\x806\x03\x82\x13\x15a\x1E/W`\0\x80\xFD[\x81\x83R\x81\x81` \x85\x017P`\0\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[`\0\x83\x83\x85R` \x80\x86\x01\x95P\x80\x85`\x05\x1B\x83\x01\x01\x84`\0\x80[\x88\x81\x10\x15a!\xC3W\x85\x84\x03`\x1F\x19\x01\x8AR\x8256\x89\x90\x03`>\x19\x01\x81\x12a!vW\x82\x83\xFD[\x88\x01`@`\x01`\x01`\xA0\x1B\x03a!\x8B\x83a\x1D\xD7V[\x16\x86Ra!\x9A\x87\x83\x01\x83a \xC9V[\x92P\x81\x88\x88\x01Ra!\xAE\x82\x88\x01\x84\x83a!\x0EV[\x9C\x88\x01\x9C\x96PPP\x92\x85\x01\x92P`\x01\x01a!QV[P\x91\x98\x97PPPPPPPPV[`\0a\x01@a!\xF0\x84a!\xE3\x85a\x1D\xD7V[`\x01`\x01`\xA0\x1B\x03\x16\x90RV[a!\xFC` \x84\x01a\x1D\xD7V[`\x01`\x01`\xA0\x1B\x03\x16` \x85\x01Ra\"\x16`@\x84\x01a\x1D\xD7V[`\x01`\x01`\xA0\x1B\x03\x16`@\x85\x01Ra\"1``\x84\x01\x84a\x1F\xDCV[\x82``\x87\x01Ra\"D\x83\x87\x01\x82\x84a CV[\x92PPPa\"X`\x80\x85\x01`\x80\x85\x01a $V[`\xC0\x83\x015`\xC0\x85\x01R`\xE0\x83\x015`\xE0\x85\x01Ra\x01\0a\"{\x81\x85\x01\x85a \x81V[\x86\x84\x03\x83\x88\x01Ra\"\x8D\x84\x82\x84a!7V[\x93PPPPa\x01 a\"\xA1\x81\x85\x01\x85a \x81V[\x86\x84\x03\x83\x88\x01Ra\"\xB3\x84\x82\x84a!7V[\x97\x96PPPPPPPV[` \x81R`\0\x825a\x01>\x19\x846\x03\x01\x81\x12a\"\xD9W`\0\x80\xFD[`@` \x84\x01Ra\"\xEF``\x84\x01\x85\x83\x01a!\xD1V[\x90Pa\"\xFE` \x85\x01\x85a \xC9V[\x84\x83\x03`\x1F\x19\x01`@\x86\x01Ra\x1C\xA3\x83\x82\x84a!\x0EV[`@\x81R`\0a#(`@\x83\x01\x85a!\xD1V[\x90P`\x01\x80`\xA0\x1B\x03\x83\x16` \x83\x01R\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a#QW`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x1C\tW`\0\x80\xFD[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a#xW`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15a#\x92W`\0\x80\xFD[` \x01\x91P`\x05\x81\x90\x1B6\x03\x82\x13\x15a\x1E/W`\0\x80\xFD[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a#\xC1W`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15a#\xDBW`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a\x1E/W`\0\x80\xFD[c\xFF\xFF\xFF\xFF\x19\x86\x16\x81R`\x80` \x82\x01R`\0a$\x10`\x80\x83\x01\x87a!\xD1V[`\x01`\x01`\xA0\x1B\x03\x86\x16`@\x84\x01R\x82\x81\x03``\x84\x01Ra$2\x81\x85\x87a!\x0EV[\x98\x97PPPPPPPPV[`\0` \x82\x84\x03\x12\x15a$PW`\0\x80\xFD[PQ\x91\x90PV[`@\x81R`\0a$j`@\x83\x01\x85a!\xD1V[\x90P\x82` \x83\x01R\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0\x825`>\x19\x836\x03\x01\x81\x12a\x1F\xD2W`\0\x80\xFD[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a$\xC7Wa$\xC7a$yV[`@R\x90V[`@Qa\x01 \x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a$\xC7Wa$\xC7a$yV[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a%\x18Wa%\x18a$yV[`@R\x91\x90PV[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15a%9Wa%9a$yV[P`\x05\x1B` \x01\x90V[`\0`@\x82\x84\x03\x12\x15a%UW`\0\x80\xFD[a%]a$\xA5V[\x90Pa%h\x82a\x1D\xD7V[\x81R` \x82\x015` \x82\x01R\x92\x91PPV[`\0\x82`\x1F\x83\x01\x12a%\x8BW`\0\x80\xFD[\x815` a%\xA0a%\x9B\x83a% V[a$\xF0V[\x80\x83\x82R` \x82\x01\x91P` \x84`\x06\x1B\x87\x01\x01\x93P\x86\x84\x11\x15a%\xC2W`\0\x80\xFD[` \x86\x01[\x84\x81\x10\x15a%\xE7Wa%\xD9\x88\x82a%CV[\x83R\x91\x83\x01\x91`@\x01a%\xC7V[P\x96\x95PPPPPPV[`\0`\x1F\x83`\x1F\x84\x01\x12a&\x05W`\0\x80\xFD[\x825` a&\x15a%\x9B\x83a% V[\x82\x81R`\x05\x92\x90\x92\x1B\x85\x01\x81\x01\x91\x81\x81\x01\x90\x87\x84\x11\x15a&4W`\0\x80\xFD[\x82\x87\x01[\x84\x81\x10\x15a'\x13W\x805`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a&XW`\0\x80\x81\xFD[\x90\x89\x01\x90`@`\x1F\x19\x83\x8D\x03\x81\x01\x82\x13\x15a&sW`\0\x80\x81\xFD[a&{a$\xA5V[a&\x86\x89\x86\x01a\x1D\xD7V[\x81R\x82\x85\x015\x84\x81\x11\x15a&\x9AW`\0\x80\x81\xFD[\x80\x86\x01\x95PP\x8D`?\x86\x01\x12a&\xB0W`\0\x80\x81\xFD[\x88\x85\x015\x84\x81\x11\x15a&\xC4Wa&\xC4a$yV[a&\xD3\x8A\x84\x8E\x84\x01\x16\x01a$\xF0V[\x94P\x80\x85R\x8E\x84\x82\x88\x01\x01\x11\x15a&\xECW`\0\x92P\x82\x83\xFD[\x80\x84\x87\x01\x8B\x87\x017`\0\x90\x85\x01\x8A\x01R\x80\x89\x01\x93\x90\x93RPP\x84RP\x91\x83\x01\x91\x83\x01a&8V[P\x97\x96PPPPPPPV[`\0a\x01@\x826\x03\x12\x15a'2W`\0\x80\xFD[a':a$\xCDV[a'C\x83a\x1D\xD7V[\x81Ra'Q` \x84\x01a\x1D\xD7V[` \x82\x01Ra'b`@\x84\x01a\x1D\xD7V[`@\x82\x01R``\x83\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a'\x81W`\0\x80\xFD[a'\x8D6\x83\x87\x01a%zV[``\x84\x01Ra'\x9F6`\x80\x87\x01a%CV[`\x80\x84\x01R`\xC0\x85\x015`\xA0\x84\x01R`\xE0\x85\x015`\xC0\x84\x01Ra\x01\0\x91P\x81\x85\x015\x81\x81\x11\x15a'\xCEW`\0\x80\xFD[a'\xDA6\x82\x88\x01a%\xF2V[`\xE0\x85\x01RPa\x01 \x85\x015\x81\x81\x11\x15a'\xF3W`\0\x80\xFD[a'\xFF6\x82\x88\x01a%\xF2V[\x83\x85\x01RPPP\x80\x91PP\x91\x90PV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P` \x84\x01`\0[\x83\x81\x10\x15a(]Wa(J\x87\x83Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x82R` \x90\x81\x01Q\x91\x01RV[`@\x96\x90\x96\x01\x95\x90\x82\x01\x90`\x01\x01a($V[P\x94\x95\x94PPPPPV[`\0[\x83\x81\x10\x15a(\x83W\x81\x81\x01Q\x83\x82\x01R` \x01a(kV[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra(\xA4\x81` \x86\x01` \x86\x01a(hV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[`\0\x82\x82Q\x80\x85R` \x80\x86\x01\x95P\x80\x82`\x05\x1B\x84\x01\x01\x81\x86\x01`\0[\x84\x81\x10\x15a) W\x85\x83\x03`\x1F\x19\x01\x89R\x81Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x84R\x84\x01Q`@\x85\x85\x01\x81\x90Ra)\x0C\x81\x86\x01\x83a(\x8CV[\x9A\x86\x01\x9A\x94PPP\x90\x83\x01\x90`\x01\x01a(\xD5V[P\x90\x97\x96PPPPPPPV[`\0`\x80\x80\x83\x01c\xFF\xFF\xFF\xFF\x19\x89\x16\x84R` `\x80\x81\x86\x01R\x81\x89Q\x80\x84R`\xA0\x93P`\xA0\x87\x01\x91P`\xA0\x81`\x05\x1B\x88\x01\x01\x83\x8C\x01`\0[\x83\x81\x10\x15a*RW\x89\x83\x03`\x9F\x19\x01\x85R\x81Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x84Ra\x01@\x81\x88\x01Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x86\x8A\x01RP`@\x82\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x87\x83\x01RPP``\x80\x83\x01Q\x82\x82\x88\x01Ra)\xC8\x83\x88\x01\x82a(\x0FV[\x92PPP\x89\x82\x01Qa)\xEF\x8B\x87\x01\x82\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x82R` \x90\x81\x01Q\x91\x01RV[P\x81\x89\x01Q`\xC0\x86\x81\x01\x91\x90\x91R\x82\x01Q`\xE0\x80\x87\x01\x91\x90\x91R\x82\x01Q\x85\x82\x03a\x01\0\x80\x88\x01\x91\x90\x91Ra*#\x83\x83a(\xB8V[\x93\x01Q\x86\x84\x03a\x01 \x88\x01R\x92\x91Pa*>\x90P\x81\x83a(\xB8V[\x96\x88\x01\x96\x94PPP\x90\x85\x01\x90`\x01\x01a)eV[PP`\x01`\x01`\xA0\x1B\x03\x8B\x16`@\x89\x01R\x87\x81\x03``\x89\x01Ra*v\x81\x8A\x8Ca!\x0EV[\x9D\x9CPPPPPPPPPPPPPV[`\0` \x80\x83\x85\x03\x12\x15a*\x9AW`\0\x80\xFD[\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15a*\xB0W`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a*\xC1W`\0\x80\xFD[\x80Qa*\xCFa%\x9B\x82a% V[\x81\x81R`\x05\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x87\x83\x11\x15a*\xEEW`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15a\"\xB3W\x83Q\x82R\x92\x84\x01\x92\x90\x84\x01\x90a*\xF3V[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a+#W`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15a+=W`\0\x80\xFD[` \x01\x91P`\x06\x81\x90\x1B6\x03\x82\x13\x15a\x1E/W`\0\x80\xFD[`\0\x84Qa+g\x81\x84` \x89\x01a(hV[\x84Q\x90\x83\x01\x90a+{\x81\x83` \x89\x01a(hV[\x84Q\x91\x01\x90a+\x8E\x81\x83` \x88\x01a(hV[\x01\x95\x94PPPPPV[\x81Q`\0\x90\x82\x90` \x80\x86\x01\x84[\x83\x81\x10\x15a+\xC2W\x81Q\x85R\x93\x82\x01\x93\x90\x82\x01\x90`\x01\x01a+\xA6V[P\x92\x96\x95PPPPPPV[`\0\x86Qa+\xE0\x81\x84` \x8B\x01a(hV[\x86Q\x90\x83\x01\x90a+\xF4\x81\x83` \x8B\x01a(hV[\x86Q\x91\x01\x90a,\x07\x81\x83` \x8A\x01a(hV[\x85Q\x91\x01\x90a,\x1A\x81\x83` \x89\x01a(hV[\x84Q\x91\x01\x90a,-\x81\x83` \x88\x01a(hV[\x01\x97\x96PPPPPPPV[`\xC0\x81R`\0a\x01 \x82\x01\x89Q```\xC0\x85\x01R\x81\x81Q\x80\x84Ra\x01@\x86\x01\x91P` \x93P` \x83\x01\x92P`\0[\x81\x81\x10\x15a,\xA0Wa,\x8D\x83\x85Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x82R` \x90\x81\x01Q\x91\x01RV[\x92\x84\x01\x92`@\x92\x90\x92\x01\x91`\x01\x01a,gV[PP` \x8C\x01Q`\xE0\x86\x01R`@\x8C\x01Qa\x01\0\x86\x01R\x84\x81\x03` \x86\x01Ra,\xC9\x81\x8Ca(\x0FV[\x92PPPa,\xE2`@\x84\x01\x89`\x01`\x01`\xA0\x1B\x03\x16\x90RV[\x86``\x84\x01R\x82\x81\x03`\x80\x84\x01Ra,\xFA\x81\x87a(\x8CV[\x90P\x82\x81\x03`\xA0\x84\x01Ra-\x0F\x81\x85\x87a!\x0EV[\x9A\x99PPPPPPPPPPV[\x82\x81R``\x81\x01a\x1C\t` \x83\x01\x84\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x82R` \x90\x81\x01Q\x91\x01RV[\x81\x83\x827`\0\x91\x01\x90\x81R\x91\x90PV[`\0\x82Qa\x1F\xD2\x81\x84` \x87\x01a(hV\xFEPermitBatchWitnessTransferFrom(TokenPermissions[] permitted,address spender,uint256 nonce,uint256 deadline,Order witness)TokenPermissions(address token,uint256 amount)Order(address offerer,address zone,address recipient,Item[] offer,Item consideration,uint256 deadline,uint256 nonce,Hook[] preHooks,Hook[] postHooks)Item(address token,uint256 amount)Hook(address target,bytes data)\0\xA2dipfsX\"\x12 tne~^\xB3%\x0C0\xFCK. \x02+_\xFA\xE2m\xD7x\xFD\t[\x84T\x0B\x82\x06\x0F\x85adsolcC\0\x08\x17\x003";
    /// The deployed bytecode of the contract.
    pub static FLOODPLAIN_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct FloodPlain<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for FloodPlain<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for FloodPlain<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for FloodPlain<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for FloodPlain<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(FloodPlain)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> FloodPlain<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    FLOODPLAIN_ABI.clone(),
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
                FLOODPLAIN_ABI.clone(),
                FLOODPLAIN_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `FALLBACK_SELECTOR` (0x4ff68609) function
        pub fn fallback_selector(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 1]> {
            self.0
                .method_hash([79, 246, 134, 9], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `PERMIT2` (0x6afdd850) function
        pub fn permit2(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([106, 253, 216, 80], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `addDecoder` (0x9d481b66) function
        pub fn add_decoder(
            &self,
            decoder: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([157, 72, 27, 102], decoder)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `decoders` (0x78654a52) function
        pub fn decoders(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([120, 101, 74, 82], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `etchOrder` (0x41bac953) function
        pub fn etch_order(
            &self,
            order_with_signature: SignedOrder,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([65, 186, 201, 83], (order_with_signature,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `fulfillOrder` (0x4ee3995f) function
        pub fn fulfill_order_with_fulfiller(
            &self,
            package: SignedOrder,
            fulfiller: ::ethers::core::types::Address,
            swap_data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([78, 227, 153, 95], (package, fulfiller, swap_data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `fulfillOrder` (0x69604aad) function
        pub fn fulfill_order(
            &self,
            package: SignedOrder,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([105, 96, 74, 173], (package,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `fulfillOrders` (0x76269663) function
        pub fn fulfill_orders(
            &self,
            packages: ::std::vec::Vec<SignedOrder>,
            fulfiller: ::ethers::core::types::Address,
            swap_data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([118, 38, 150, 99], (packages, fulfiller, swap_data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getNonceStatus` (0xe9ba1e97) function
        pub fn get_nonce_status(
            &self,
            user: ::ethers::core::types::Address,
            nonce: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([233, 186, 30, 151], (user, nonce))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getOrderHash` (0x3dd908fc) function
        pub fn get_order_hash(
            &self,
            order: Order,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([61, 217, 8, 252], (order,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getOrderStatus` (0xaf5e7f35) function
        pub fn get_order_status(
            &self,
            order: Order,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([175, 94, 127, 53], (order,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getPermitHash` (0x6b224ccb) function
        pub fn get_permit_hash(
            &self,
            order: Order,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([107, 34, 76, 203], (order,))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `DecoderAdded` event
        pub fn decoder_added_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            DecoderAddedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `OrderEtched` event
        pub fn order_etched_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OrderEtchedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `OrderFulfilled` event
        pub fn order_fulfilled_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OrderFulfilledFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            FloodPlainEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for FloodPlain<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `AddressEmptyCode` with signature `AddressEmptyCode(address)` and selector `0x9996b315`
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
    #[etherror(name = "AddressEmptyCode", abi = "AddressEmptyCode(address)")]
    pub struct AddressEmptyCode {
        pub target: ::ethers::core::types::Address,
    }
    ///Custom Error type `AddressInsufficientBalance` with signature `AddressInsufficientBalance(address)` and selector `0xcd786059`
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
        name = "AddressInsufficientBalance",
        abi = "AddressInsufficientBalance(address)"
    )]
    pub struct AddressInsufficientBalance {
        pub account: ::ethers::core::types::Address,
    }
    ///Custom Error type `ArrayLengthMismatch` with signature `ArrayLengthMismatch()` and selector `0xa24a13a6`
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
    #[etherror(name = "ArrayLengthMismatch", abi = "ArrayLengthMismatch()")]
    pub struct ArrayLengthMismatch;
    ///Custom Error type `DuplicateItems` with signature `DuplicateItems()` and selector `0xecc38376`
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
    #[etherror(name = "DuplicateItems", abi = "DuplicateItems()")]
    pub struct DuplicateItems;
    ///Custom Error type `FailedInnerCall` with signature `FailedInnerCall()` and selector `0x1425ea42`
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
    #[etherror(name = "FailedInnerCall", abi = "FailedInnerCall()")]
    pub struct FailedInnerCall;
    ///Custom Error type `InsufficientAmountReceived` with signature `InsufficientAmountReceived()` and selector `0x910f3412`
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
        name = "InsufficientAmountReceived",
        abi = "InsufficientAmountReceived()"
    )]
    pub struct InsufficientAmountReceived;
    ///Custom Error type `NotAContract` with signature `NotAContract()` and selector `0x09ee12d5`
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
    #[etherror(name = "NotAContract", abi = "NotAContract()")]
    pub struct NotAContract;
    ///Custom Error type `ReentrancyGuardReentrantCall` with signature `ReentrancyGuardReentrantCall()` and selector `0x3ee5aeb5`
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
        name = "ReentrancyGuardReentrantCall",
        abi = "ReentrancyGuardReentrantCall()"
    )]
    pub struct ReentrancyGuardReentrantCall;
    ///Custom Error type `SafeERC20FailedOperation` with signature `SafeERC20FailedOperation(address)` and selector `0x5274afe7`
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
        name = "SafeERC20FailedOperation",
        abi = "SafeERC20FailedOperation(address)"
    )]
    pub struct SafeERC20FailedOperation {
        pub token: ::ethers::core::types::Address,
    }
    ///Custom Error type `ZoneDenied` with signature `ZoneDenied()` and selector `0x62450ae0`
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
    #[etherror(name = "ZoneDenied", abi = "ZoneDenied()")]
    pub struct ZoneDenied;
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
    pub enum FloodPlainErrors {
        AddressEmptyCode(AddressEmptyCode),
        AddressInsufficientBalance(AddressInsufficientBalance),
        ArrayLengthMismatch(ArrayLengthMismatch),
        DuplicateItems(DuplicateItems),
        FailedInnerCall(FailedInnerCall),
        InsufficientAmountReceived(InsufficientAmountReceived),
        NotAContract(NotAContract),
        ReentrancyGuardReentrantCall(ReentrancyGuardReentrantCall),
        SafeERC20FailedOperation(SafeERC20FailedOperation),
        ZoneDenied(ZoneDenied),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for FloodPlainErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <AddressEmptyCode as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AddressEmptyCode(decoded));
            }
            if let Ok(decoded) = <AddressInsufficientBalance as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AddressInsufficientBalance(decoded));
            }
            if let Ok(decoded) = <ArrayLengthMismatch as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ArrayLengthMismatch(decoded));
            }
            if let Ok(decoded) = <DuplicateItems as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DuplicateItems(decoded));
            }
            if let Ok(decoded) = <FailedInnerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::FailedInnerCall(decoded));
            }
            if let Ok(decoded) = <InsufficientAmountReceived as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InsufficientAmountReceived(decoded));
            }
            if let Ok(decoded) = <NotAContract as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::NotAContract(decoded));
            }
            if let Ok(decoded) = <ReentrancyGuardReentrantCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ReentrancyGuardReentrantCall(decoded));
            }
            if let Ok(decoded) = <SafeERC20FailedOperation as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SafeERC20FailedOperation(decoded));
            }
            if let Ok(decoded) = <ZoneDenied as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ZoneDenied(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for FloodPlainErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::AddressEmptyCode(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AddressInsufficientBalance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ArrayLengthMismatch(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DuplicateItems(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FailedInnerCall(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InsufficientAmountReceived(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NotAContract(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ReentrancyGuardReentrantCall(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SafeERC20FailedOperation(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ZoneDenied(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for FloodPlainErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <AddressEmptyCode as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <AddressInsufficientBalance as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ArrayLengthMismatch as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <DuplicateItems as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <FailedInnerCall as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InsufficientAmountReceived as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <NotAContract as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <ReentrancyGuardReentrantCall as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <SafeERC20FailedOperation as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ZoneDenied as ::ethers::contract::EthError>::selector() => true,
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for FloodPlainErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AddressEmptyCode(element) => ::core::fmt::Display::fmt(element, f),
                Self::AddressInsufficientBalance(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ArrayLengthMismatch(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DuplicateItems(element) => ::core::fmt::Display::fmt(element, f),
                Self::FailedInnerCall(element) => ::core::fmt::Display::fmt(element, f),
                Self::InsufficientAmountReceived(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::NotAContract(element) => ::core::fmt::Display::fmt(element, f),
                Self::ReentrancyGuardReentrantCall(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SafeERC20FailedOperation(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ZoneDenied(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for FloodPlainErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<AddressEmptyCode> for FloodPlainErrors {
        fn from(value: AddressEmptyCode) -> Self {
            Self::AddressEmptyCode(value)
        }
    }
    impl ::core::convert::From<AddressInsufficientBalance> for FloodPlainErrors {
        fn from(value: AddressInsufficientBalance) -> Self {
            Self::AddressInsufficientBalance(value)
        }
    }
    impl ::core::convert::From<ArrayLengthMismatch> for FloodPlainErrors {
        fn from(value: ArrayLengthMismatch) -> Self {
            Self::ArrayLengthMismatch(value)
        }
    }
    impl ::core::convert::From<DuplicateItems> for FloodPlainErrors {
        fn from(value: DuplicateItems) -> Self {
            Self::DuplicateItems(value)
        }
    }
    impl ::core::convert::From<FailedInnerCall> for FloodPlainErrors {
        fn from(value: FailedInnerCall) -> Self {
            Self::FailedInnerCall(value)
        }
    }
    impl ::core::convert::From<InsufficientAmountReceived> for FloodPlainErrors {
        fn from(value: InsufficientAmountReceived) -> Self {
            Self::InsufficientAmountReceived(value)
        }
    }
    impl ::core::convert::From<NotAContract> for FloodPlainErrors {
        fn from(value: NotAContract) -> Self {
            Self::NotAContract(value)
        }
    }
    impl ::core::convert::From<ReentrancyGuardReentrantCall> for FloodPlainErrors {
        fn from(value: ReentrancyGuardReentrantCall) -> Self {
            Self::ReentrancyGuardReentrantCall(value)
        }
    }
    impl ::core::convert::From<SafeERC20FailedOperation> for FloodPlainErrors {
        fn from(value: SafeERC20FailedOperation) -> Self {
            Self::SafeERC20FailedOperation(value)
        }
    }
    impl ::core::convert::From<ZoneDenied> for FloodPlainErrors {
        fn from(value: ZoneDenied) -> Self {
            Self::ZoneDenied(value)
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
    #[ethevent(name = "DecoderAdded", abi = "DecoderAdded(uint256,address)")]
    pub struct DecoderAddedFilter {
        #[ethevent(indexed)]
        pub decoder_id: ::ethers::core::types::U256,
        #[ethevent(indexed)]
        pub decoder: ::ethers::core::types::Address,
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
        name = "OrderEtched",
        abi = "OrderEtched(bytes32,((address,address,address,(address,uint256)[],(address,uint256),uint256,uint256,(address,bytes)[],(address,bytes)[]),bytes))"
    )]
    pub struct OrderEtchedFilter {
        #[ethevent(indexed)]
        pub order_hash: [u8; 32],
        pub order_with_signature: SignedOrder,
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
        name = "OrderFulfilled",
        abi = "OrderFulfilled((address,address,address,(address,uint256)[],(address,uint256),uint256,uint256,(address,bytes)[],(address,bytes)[]),address,uint256)"
    )]
    pub struct OrderFulfilledFilter {
        pub order: Order,
        #[ethevent(indexed)]
        pub fulfiller: ::ethers::core::types::Address,
        pub amount_out: ::ethers::core::types::U256,
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
    pub enum FloodPlainEvents {
        DecoderAddedFilter(DecoderAddedFilter),
        OrderEtchedFilter(OrderEtchedFilter),
        OrderFulfilledFilter(OrderFulfilledFilter),
    }
    impl ::ethers::contract::EthLogDecode for FloodPlainEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = DecoderAddedFilter::decode_log(log) {
                return Ok(FloodPlainEvents::DecoderAddedFilter(decoded));
            }
            if let Ok(decoded) = OrderEtchedFilter::decode_log(log) {
                return Ok(FloodPlainEvents::OrderEtchedFilter(decoded));
            }
            if let Ok(decoded) = OrderFulfilledFilter::decode_log(log) {
                return Ok(FloodPlainEvents::OrderFulfilledFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for FloodPlainEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::DecoderAddedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OrderEtchedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::OrderFulfilledFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<DecoderAddedFilter> for FloodPlainEvents {
        fn from(value: DecoderAddedFilter) -> Self {
            Self::DecoderAddedFilter(value)
        }
    }
    impl ::core::convert::From<OrderEtchedFilter> for FloodPlainEvents {
        fn from(value: OrderEtchedFilter) -> Self {
            Self::OrderEtchedFilter(value)
        }
    }
    impl ::core::convert::From<OrderFulfilledFilter> for FloodPlainEvents {
        fn from(value: OrderFulfilledFilter) -> Self {
            Self::OrderFulfilledFilter(value)
        }
    }
    ///Container type for all input parameters for the `FALLBACK_SELECTOR` function with signature `FALLBACK_SELECTOR()` and selector `0x4ff68609`
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
    #[ethcall(name = "FALLBACK_SELECTOR", abi = "FALLBACK_SELECTOR()")]
    pub struct FallbackSelectorCall;
    ///Container type for all input parameters for the `PERMIT2` function with signature `PERMIT2()` and selector `0x6afdd850`
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
    #[ethcall(name = "PERMIT2", abi = "PERMIT2()")]
    pub struct Permit2Call;
    ///Container type for all input parameters for the `addDecoder` function with signature `addDecoder(address)` and selector `0x9d481b66`
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
    #[ethcall(name = "addDecoder", abi = "addDecoder(address)")]
    pub struct AddDecoderCall {
        pub decoder: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `decoders` function with signature `decoders(uint256)` and selector `0x78654a52`
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
    #[ethcall(name = "decoders", abi = "decoders(uint256)")]
    pub struct DecodersCall(pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `etchOrder` function with signature `etchOrder(((address,address,address,(address,uint256)[],(address,uint256),uint256,uint256,(address,bytes)[],(address,bytes)[]),bytes))` and selector `0x41bac953`
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
        name = "etchOrder",
        abi = "etchOrder(((address,address,address,(address,uint256)[],(address,uint256),uint256,uint256,(address,bytes)[],(address,bytes)[]),bytes))"
    )]
    pub struct EtchOrderCall {
        pub order_with_signature: SignedOrder,
    }
    ///Container type for all input parameters for the `fulfillOrder` function with signature `fulfillOrder(((address,address,address,(address,uint256)[],(address,uint256),uint256,uint256,(address,bytes)[],(address,bytes)[]),bytes),address,bytes)` and selector `0x4ee3995f`
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
        name = "fulfillOrder",
        abi = "fulfillOrder(((address,address,address,(address,uint256)[],(address,uint256),uint256,uint256,(address,bytes)[],(address,bytes)[]),bytes),address,bytes)"
    )]
    pub struct FulfillOrderWithFulfillerCall {
        pub package: SignedOrder,
        pub fulfiller: ::ethers::core::types::Address,
        pub swap_data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `fulfillOrder` function with signature `fulfillOrder(((address,address,address,(address,uint256)[],(address,uint256),uint256,uint256,(address,bytes)[],(address,bytes)[]),bytes))` and selector `0x69604aad`
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
        name = "fulfillOrder",
        abi = "fulfillOrder(((address,address,address,(address,uint256)[],(address,uint256),uint256,uint256,(address,bytes)[],(address,bytes)[]),bytes))"
    )]
    pub struct FulfillOrderCall {
        pub package: SignedOrder,
    }
    ///Container type for all input parameters for the `fulfillOrders` function with signature `fulfillOrders(((address,address,address,(address,uint256)[],(address,uint256),uint256,uint256,(address,bytes)[],(address,bytes)[]),bytes)[],address,bytes)` and selector `0x76269663`
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
        name = "fulfillOrders",
        abi = "fulfillOrders(((address,address,address,(address,uint256)[],(address,uint256),uint256,uint256,(address,bytes)[],(address,bytes)[]),bytes)[],address,bytes)"
    )]
    pub struct FulfillOrdersCall {
        pub packages: ::std::vec::Vec<SignedOrder>,
        pub fulfiller: ::ethers::core::types::Address,
        pub swap_data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `getNonceStatus` function with signature `getNonceStatus(address,uint256)` and selector `0xe9ba1e97`
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
    #[ethcall(name = "getNonceStatus", abi = "getNonceStatus(address,uint256)")]
    pub struct GetNonceStatusCall {
        pub user: ::ethers::core::types::Address,
        pub nonce: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getOrderHash` function with signature `getOrderHash((address,address,address,(address,uint256)[],(address,uint256),uint256,uint256,(address,bytes)[],(address,bytes)[]))` and selector `0x3dd908fc`
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
        name = "getOrderHash",
        abi = "getOrderHash((address,address,address,(address,uint256)[],(address,uint256),uint256,uint256,(address,bytes)[],(address,bytes)[]))"
    )]
    pub struct GetOrderHashCall {
        pub order: Order,
    }
    ///Container type for all input parameters for the `getOrderStatus` function with signature `getOrderStatus((address,address,address,(address,uint256)[],(address,uint256),uint256,uint256,(address,bytes)[],(address,bytes)[]))` and selector `0xaf5e7f35`
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
        name = "getOrderStatus",
        abi = "getOrderStatus((address,address,address,(address,uint256)[],(address,uint256),uint256,uint256,(address,bytes)[],(address,bytes)[]))"
    )]
    pub struct GetOrderStatusCall {
        pub order: Order,
    }
    ///Container type for all input parameters for the `getPermitHash` function with signature `getPermitHash((address,address,address,(address,uint256)[],(address,uint256),uint256,uint256,(address,bytes)[],(address,bytes)[]))` and selector `0x6b224ccb`
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
        name = "getPermitHash",
        abi = "getPermitHash((address,address,address,(address,uint256)[],(address,uint256),uint256,uint256,(address,bytes)[],(address,bytes)[]))"
    )]
    pub struct GetPermitHashCall {
        pub order: Order,
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
    pub enum FloodPlainCalls {
        FallbackSelector(FallbackSelectorCall),
        Permit2(Permit2Call),
        AddDecoder(AddDecoderCall),
        Decoders(DecodersCall),
        EtchOrder(EtchOrderCall),
        FulfillOrderWithFulfiller(FulfillOrderWithFulfillerCall),
        FulfillOrder(FulfillOrderCall),
        FulfillOrders(FulfillOrdersCall),
        GetNonceStatus(GetNonceStatusCall),
        GetOrderHash(GetOrderHashCall),
        GetOrderStatus(GetOrderStatusCall),
        GetPermitHash(GetPermitHashCall),
    }
    impl ::ethers::core::abi::AbiDecode for FloodPlainCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <FallbackSelectorCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::FallbackSelector(decoded));
            }
            if let Ok(decoded) = <Permit2Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Permit2(decoded));
            }
            if let Ok(decoded) = <AddDecoderCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AddDecoder(decoded));
            }
            if let Ok(decoded) = <DecodersCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Decoders(decoded));
            }
            if let Ok(decoded) = <EtchOrderCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EtchOrder(decoded));
            }
            if let Ok(decoded) = <FulfillOrderWithFulfillerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::FulfillOrderWithFulfiller(decoded));
            }
            if let Ok(decoded) = <FulfillOrderCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::FulfillOrder(decoded));
            }
            if let Ok(decoded) = <FulfillOrdersCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::FulfillOrders(decoded));
            }
            if let Ok(decoded) = <GetNonceStatusCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetNonceStatus(decoded));
            }
            if let Ok(decoded) = <GetOrderHashCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetOrderHash(decoded));
            }
            if let Ok(decoded) = <GetOrderStatusCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetOrderStatus(decoded));
            }
            if let Ok(decoded) = <GetPermitHashCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetPermitHash(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for FloodPlainCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::FallbackSelector(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Permit2(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::AddDecoder(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Decoders(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EtchOrder(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FulfillOrderWithFulfiller(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FulfillOrder(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FulfillOrders(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetNonceStatus(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetOrderHash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetOrderStatus(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetPermitHash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for FloodPlainCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::FallbackSelector(element) => ::core::fmt::Display::fmt(element, f),
                Self::Permit2(element) => ::core::fmt::Display::fmt(element, f),
                Self::AddDecoder(element) => ::core::fmt::Display::fmt(element, f),
                Self::Decoders(element) => ::core::fmt::Display::fmt(element, f),
                Self::EtchOrder(element) => ::core::fmt::Display::fmt(element, f),
                Self::FulfillOrderWithFulfiller(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::FulfillOrder(element) => ::core::fmt::Display::fmt(element, f),
                Self::FulfillOrders(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetNonceStatus(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetOrderHash(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetOrderStatus(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPermitHash(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<FallbackSelectorCall> for FloodPlainCalls {
        fn from(value: FallbackSelectorCall) -> Self {
            Self::FallbackSelector(value)
        }
    }
    impl ::core::convert::From<Permit2Call> for FloodPlainCalls {
        fn from(value: Permit2Call) -> Self {
            Self::Permit2(value)
        }
    }
    impl ::core::convert::From<AddDecoderCall> for FloodPlainCalls {
        fn from(value: AddDecoderCall) -> Self {
            Self::AddDecoder(value)
        }
    }
    impl ::core::convert::From<DecodersCall> for FloodPlainCalls {
        fn from(value: DecodersCall) -> Self {
            Self::Decoders(value)
        }
    }
    impl ::core::convert::From<EtchOrderCall> for FloodPlainCalls {
        fn from(value: EtchOrderCall) -> Self {
            Self::EtchOrder(value)
        }
    }
    impl ::core::convert::From<FulfillOrderWithFulfillerCall> for FloodPlainCalls {
        fn from(value: FulfillOrderWithFulfillerCall) -> Self {
            Self::FulfillOrderWithFulfiller(value)
        }
    }
    impl ::core::convert::From<FulfillOrderCall> for FloodPlainCalls {
        fn from(value: FulfillOrderCall) -> Self {
            Self::FulfillOrder(value)
        }
    }
    impl ::core::convert::From<FulfillOrdersCall> for FloodPlainCalls {
        fn from(value: FulfillOrdersCall) -> Self {
            Self::FulfillOrders(value)
        }
    }
    impl ::core::convert::From<GetNonceStatusCall> for FloodPlainCalls {
        fn from(value: GetNonceStatusCall) -> Self {
            Self::GetNonceStatus(value)
        }
    }
    impl ::core::convert::From<GetOrderHashCall> for FloodPlainCalls {
        fn from(value: GetOrderHashCall) -> Self {
            Self::GetOrderHash(value)
        }
    }
    impl ::core::convert::From<GetOrderStatusCall> for FloodPlainCalls {
        fn from(value: GetOrderStatusCall) -> Self {
            Self::GetOrderStatus(value)
        }
    }
    impl ::core::convert::From<GetPermitHashCall> for FloodPlainCalls {
        fn from(value: GetPermitHashCall) -> Self {
            Self::GetPermitHash(value)
        }
    }
    ///Container type for all return fields from the `FALLBACK_SELECTOR` function with signature `FALLBACK_SELECTOR()` and selector `0x4ff68609`
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
    pub struct FallbackSelectorReturn(pub [u8; 1]);
    ///Container type for all return fields from the `PERMIT2` function with signature `PERMIT2()` and selector `0x6afdd850`
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
    pub struct Permit2Return(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `addDecoder` function with signature `addDecoder(address)` and selector `0x9d481b66`
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
    pub struct AddDecoderReturn {
        pub id: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `decoders` function with signature `decoders(uint256)` and selector `0x78654a52`
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
    pub struct DecodersReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getNonceStatus` function with signature `getNonceStatus(address,uint256)` and selector `0xe9ba1e97`
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
    pub struct GetNonceStatusReturn(pub bool);
    ///Container type for all return fields from the `getOrderHash` function with signature `getOrderHash((address,address,address,(address,uint256)[],(address,uint256),uint256,uint256,(address,bytes)[],(address,bytes)[]))` and selector `0x3dd908fc`
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
    pub struct GetOrderHashReturn(pub [u8; 32]);
    ///Container type for all return fields from the `getOrderStatus` function with signature `getOrderStatus((address,address,address,(address,uint256)[],(address,uint256),uint256,uint256,(address,bytes)[],(address,bytes)[]))` and selector `0xaf5e7f35`
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
    pub struct GetOrderStatusReturn(pub bool);
    ///Container type for all return fields from the `getPermitHash` function with signature `getPermitHash((address,address,address,(address,uint256)[],(address,uint256),uint256,uint256,(address,bytes)[],(address,bytes)[]))` and selector `0x6b224ccb`
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
    pub struct GetPermitHashReturn(pub [u8; 32]);
}
