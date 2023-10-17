pub use decoder_with_registry::*;
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
pub mod decoder_with_registry {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("acceptOwnership"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("acceptOwnership"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("fulfillers"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("fulfillers"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                ::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ::ethers::core::abi::ethabi::ParamType::Address,
                                    ],),
                                ),
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct IdToAddress[]"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("owner"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("owner"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("pendingOwner"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("pendingOwner"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("renounceOwnership"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("renounceOwnership"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setFulfiller"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setFulfiller"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("fulfillerId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint8"),
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
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setToken"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setToken"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("tokenId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint16"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("token"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setZone"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setZone"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("zoneId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint8"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("zone"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("tokens"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("tokens"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                ::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ::ethers::core::abi::ethabi::ParamType::Address,
                                    ],),
                                ),
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct IdToAddress[]"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("transferOwnership"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("transferOwnership"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("newOwner"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("zones"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("zones"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                ::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ::ethers::core::abi::ethabi::ParamType::Address,
                                    ],),
                                ),
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct IdToAddress[]"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("OwnershipTransferStarted"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("OwnershipTransferStarted",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("previousOwner"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("newOwner"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OwnershipTransferred"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("OwnershipTransferred",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("previousOwner"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("newOwner"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
            ]),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: true,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static DECODERWITHREGISTRY_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa\0\x1A3a\0\x1FV[a\0\x96V[`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x90Ua\0C\x81a\0F` \x90\x81\x1Ba\x06\xCC\x17\x90\x1CV[PV[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x84U`@Q\x91\x90\x92\x16\x92\x83\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PPV[a\r'\x80a\0\xA5`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xA9W`\x005`\xE0\x1C\x80c\x87g\x17\x1C\x11a\0qW\x80c\x87g\x17\x1C\x14a\x04\xC8W\x80c\x8D\xA5\xCB[\x14a\x04\xDBW\x80c\x9Dc\x84\x8A\x14a\x05\0W\x80c\xD7\xB4\x1AG\x14a\x05\x08W\x80c\xE3\x0C9x\x14a\x05\x1BW\x80c\xF2\xFD\xE3\x8B\x14a\x05,Wa\0\xA9V[\x80c\x03\xD2PI\x14a\x04}W\x80c\x1BJq\x8A\x14a\x04\x9BW\x80c/m \x03\x14a\x04\xB0W\x80cqP\x18\xA6\x14a\x04\xB8W\x80cy\xBAP\x97\x14a\x04\xC0W[`\x006``a\0\xF9`@Q\x80`\xC0\x01`@R\x80`\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01`\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01``\x81R` \x01``\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[`\0\x805``\x90\x81\x1C\x83R\x90`\x14a\x01\x16`\x02\x825`\xF8\x1Ca\x05?V[`\x01`\x01`\xA0\x1B\x03\x16` \x85\x01R`\x01\x01`\0\x815`\xF8\x1C`\x01\x92\x83\x01\x92\x90\x91P`\x0F`\x04\x83\x90\x1C\x16\x01`\xFF\x16\x825`\x08\x82\x02a\x01\0\x03\x1C`\xA0\x87\x01R\x91\x82\x01\x91`\x0F\x82\x16`\x01\x01`\xFF\x16\x835`\x08\x82\x02a\x01\0\x03\x1C`\x80\x88\x01R\x92\x83\x01\x92a\x01\x88\x845`\xF8\x1C`\x05\x90`\xFF\x16a\x05?V[\x94P\x83`\x01\x01\x93PPPP`\0a\x01\xA0\x825`\xF8\x1C\x90V[`\x01\x92\x83\x01\x92`\xFF\x82\x16\x92P`\x0F`\x04\x83\x90\x1C\x81\x16\x82\x01\x92\x16\x01`\0\x82g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x01\xD5Wa\x01\xD5a\n:V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x02\x1AW\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x01\xF3W\x90P[P\x90P`\0\x82g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x028Wa\x028a\n:V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x02}W\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x02VW\x90P[P\x90P`\0\x80[\x85\x81\x10\x15a\x03 Wa\x02\xA1\x885`\xF0\x1C[`\x08\x90a\xFF\xFF\x16a\x05?V[\x84\x82\x81Q\x81\x10a\x02\xB3Wa\x02\xB3a\nPV[` \x90\x81\x02\x91\x90\x91\x01\x01Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90R`\x02\x97\x90\x97\x01\x96a\x02\xDD\x885`\xF8\x1C\x90V[`\x01\x98\x89\x01\x98\x01`\xFF\x16\x91P\x875`\x08\x83\x02a\x01\0\x03\x1C\x84\x82\x81Q\x81\x10a\x03\x06Wa\x03\x06a\nPV[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q\x01R\x96\x81\x01\x96`\x01\x01a\x02\x84V[P`@\x8A\x01\x83\x90R`\0[\x84\x81\x10\x15a\x03\xBFWa\x03@\x885`\xF0\x1Ca\x02\x95V[\x83\x82\x81Q\x81\x10a\x03RWa\x03Ra\nPV[` \x90\x81\x02\x91\x90\x91\x01\x01Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90R`\x02\x97\x90\x97\x01\x96a\x03|\x885`\xF8\x1C\x90V[`\x01\x98\x89\x01\x98\x01`\xFF\x16\x91P\x875`\x08\x83\x02a\x01\0\x03\x1C\x83\x82\x81Q\x81\x10a\x03\xA5Wa\x03\xA5a\nPV[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q\x01R\x96\x81\x01\x96`\x01\x01a\x03+V[PP``\x89\x81\x01\x91\x90\x91R`@\x80Q`\x01\x88\x015` \x82\x01R`!\x88\x015\x81\x83\x01R\x875`\x01`\x01`\xF8\x1B\x03\x19\x16\x92\x81\x01\x92\x90\x92R\x80Q`A\x81\x84\x03\x81\x01\x82R`a\x90\x93\x01\x90\x91R\x97P\x90\x94\x01\x93Pc\x06M[\xC3`\xE0\x1B\x92P\x86\x91P\x85\x90P\x84a\x04+\x8A\x86\x81\x8Ea\nfV[`@Q`$\x01a\x04?\x95\x94\x93\x92\x91\x90a\x0BNV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16`\x01`\x01`\xE0\x1B\x03\x19\x93\x90\x93\x16\x92\x90\x92\x17\x82RQ\x97P\x95PPPPPP\xF3[a\x04\x85a\x05TV[`@Qa\x04\x92\x91\x90a\x0C\x02V[`@Q\x80\x91\x03\x90\xF3[a\x04\xAEa\x04\xA96`\x04a\x0CvV[a\x05eV[\0[a\x04\x85a\x05\x81V[a\x04\xAEa\x05\x8DV[a\x04\xAEa\x05\xA1V[a\x04\xAEa\x04\xD66`\x04a\x0C\xB1V[a\x06 V[`\0T`\x01`\x01`\xA0\x1B\x03\x16[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x04\x92V[a\x04\x85a\x068V[a\x04\xAEa\x05\x166`\x04a\x0CvV[a\x06DV[`\x01T`\x01`\x01`\xA0\x1B\x03\x16a\x04\xE8V[a\x04\xAEa\x05:6`\x04a\x0C\xD6V[a\x06[V[`\0a\x05K\x83\x83a\x07\x1CV[\x90P[\x92\x91PPV[``a\x05``\x02a\x07\x8CV[\x90P\x90V[a\x05ma\x08pV[a\x05|`\x02`\xFF\x84\x16\x83a\x08\xCAV[PPPV[``a\x05``\x05a\x07\x8CV[a\x05\x95a\x08pV[a\x05\x9F`\0a\x08\xE8V[V[`\x01T3\x90`\x01`\x01`\xA0\x1B\x03\x16\x81\x14a\x06\x14W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`)`$\x82\x01R\x7FOwnable2Step: caller is not the `D\x82\x01Rh72\xBB\x907\xBB\xB72\xB9`\xB9\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[a\x06\x1D\x81a\x08\xE8V[PV[a\x06(a\x08pV[a\x05|`\x08a\xFF\xFF\x84\x16\x83a\x08\xCAV[``a\x05``\x08a\x07\x8CV[a\x06La\x08pV[a\x05|`\x05`\xFF\x84\x16\x83a\x08\xCAV[a\x06ca\x08pV[`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x83\x16`\x01`\x01`\xA0\x1B\x03\x19\x90\x91\x16\x81\x17\x90\x91Ua\x06\x94`\0T`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x7F8\xD1k\x8C\xAC\"\xD9\x9F\xC7\xC1$\xB9\xCD\r\xE2\xD3\xFA\x1F\xAE\xF4 \xBF\xE7\x91\xD8\xC3b\xD7e\xE2'\0`@Q`@Q\x80\x91\x03\x90\xA3PV[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x84U`@Q\x91\x90\x92\x16\x92\x83\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PPV[`\0\x81\x81R`\x02\x83\x01` R`@\x81 T\x80\x15\x15\x80a\x07@WPa\x07@\x84\x84a\t\x01V[a\x05KW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1E`$\x82\x01R\x7FEnumerableMap: nonexistent key\0\0`D\x82\x01R`d\x01a\x06\x0BV[```\0a\x07\x99\x83a\t\rV[\x90P`\0\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x07\xB6Wa\x07\xB6a\n:V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x07\xFBW\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x07\xD4W\x90P[P\x90P`\0[\x82\x81\x10\x15a\x08hWa\x08\x13\x85\x82a\t\x18V[\x83\x83\x81Q\x81\x10a\x08%Wa\x08%a\nPV[` \x02` \x01\x01Q`\0\x01\x84\x84\x81Q\x81\x10a\x08BWa\x08Ba\nPV[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x92\x01\x91\x90\x91RR`\x01\x01a\x08\x01V[P\x93\x92PPPV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x05\x9FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x06\x0BV[`\0a\x08\xE0\x84\x84`\x01`\x01`\xA0\x1B\x03\x85\x16a\t4V[\x94\x93PPPPV[`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x90Ua\x06\x1D\x81a\x06\xCCV[`\0a\x05K\x83\x83a\tQV[`\0a\x05N\x82a\tiV[`\0\x80\x80\x80a\t'\x86\x86a\ttV[\x90\x97\x90\x96P\x94PPPPPV[`\0\x82\x81R`\x02\x84\x01` R`@\x81 \x82\x90Ua\x08\xE0\x84\x84a\t\x9FV[`\0\x81\x81R`\x01\x83\x01` R`@\x81 T\x15\x15a\x05KV[`\0a\x05N\x82a\t\xABV[`\0\x80\x80a\t\x82\x85\x85a\t\xB5V[`\0\x81\x81R`\x02\x96\x90\x96\x01` R`@\x90\x95 T\x94\x95\x93PPPPV[`\0a\x05K\x83\x83a\t\xC1V[`\0a\x05N\x82T\x90V[`\0a\x05K\x83\x83a\n\x10V[`\0\x81\x81R`\x01\x83\x01` R`@\x81 Ta\n\x08WP\x81T`\x01\x81\x81\x01\x84U`\0\x84\x81R` \x80\x82 \x90\x93\x01\x84\x90U\x84T\x84\x82R\x82\x86\x01\x90\x93R`@\x90 \x91\x90\x91Ua\x05NV[P`\0a\x05NV[`\0\x82`\0\x01\x82\x81T\x81\x10a\n'Wa\n'a\nPV[\x90`\0R` `\0 \x01T\x90P\x92\x91PPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0\x80\x85\x85\x11\x15a\nvW`\0\x80\xFD[\x83\x86\x11\x15a\n\x83W`\0\x80\xFD[PP\x82\x01\x93\x91\x90\x92\x03\x91PV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a\n\xD4W\x81Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x88R\x83\x01Q\x83\x88\x01R`@\x90\x96\x01\x95\x90\x82\x01\x90`\x01\x01a\n\xA4V[P\x94\x95\x94PPPPPV[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15a\x0B\x05W` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a\n\xE9V[P`\0` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[\x81\x83R\x81\x81` \x85\x017P`\0\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[`\x80\x81R`\0`\x01\x80`\xA0\x1B\x03\x80\x88Q\x16`\x80\x84\x01R\x80` \x89\x01Q\x16`\xA0\x84\x01RP`@\x87\x01Q`\xC0\x80\x84\x01Ra\x0B\x8Aa\x01@\x84\x01\x82a\n\x90V[\x90P``\x88\x01Q`\x7F\x19\x84\x83\x03\x01`\xE0\x85\x01Ra\x0B\xA7\x82\x82a\n\x90V[\x91PP`\x80\x88\x01Qa\x01\0\x84\x01R`\xA0\x88\x01Qa\x01 \x84\x01R\x82\x81\x03` \x84\x01Ra\x0B\xD2\x81\x88a\n\xDFV[`\x01`\x01`\xA0\x1B\x03\x87\x16`@\x85\x01R\x90P\x82\x81\x03``\x84\x01Ra\x0B\xF6\x81\x85\x87a\x0B%V[\x98\x97PPPPPPPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90`@\x90\x81\x85\x01\x90\x86\x84\x01\x85[\x82\x81\x10\x15a\x0CMW\x81Q\x80Q\x85R\x86\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x86\x85\x01R\x92\x84\x01\x92\x90\x85\x01\x90`\x01\x01a\x0C\x1FV[P\x91\x97\x96PPPPPPPV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x0CqW`\0\x80\xFD[\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x0C\x89W`\0\x80\xFD[\x825`\xFF\x81\x16\x81\x14a\x0C\x9AW`\0\x80\xFD[\x91Pa\x0C\xA8` \x84\x01a\x0CZV[\x90P\x92P\x92\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x0C\xC4W`\0\x80\xFD[\x825a\xFF\xFF\x81\x16\x81\x14a\x0C\x9AW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x0C\xE8W`\0\x80\xFD[a\x05K\x82a\x0CZV\xFE\xA2dipfsX\"\x12 \xE9O<o\xAAm\xCD\xB3\xAA\xB9\x02T\x88\x1C\x9BZ\x18z\x17\x97\xF9p\xBAW}\xB2\xD7T\x18\x15\x84\x10dsolcC\0\x08\x11\x003";
    /// The bytecode of the contract.
    pub static DECODERWITHREGISTRY_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xA9W`\x005`\xE0\x1C\x80c\x87g\x17\x1C\x11a\0qW\x80c\x87g\x17\x1C\x14a\x04\xC8W\x80c\x8D\xA5\xCB[\x14a\x04\xDBW\x80c\x9Dc\x84\x8A\x14a\x05\0W\x80c\xD7\xB4\x1AG\x14a\x05\x08W\x80c\xE3\x0C9x\x14a\x05\x1BW\x80c\xF2\xFD\xE3\x8B\x14a\x05,Wa\0\xA9V[\x80c\x03\xD2PI\x14a\x04}W\x80c\x1BJq\x8A\x14a\x04\x9BW\x80c/m \x03\x14a\x04\xB0W\x80cqP\x18\xA6\x14a\x04\xB8W\x80cy\xBAP\x97\x14a\x04\xC0W[`\x006``a\0\xF9`@Q\x80`\xC0\x01`@R\x80`\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01`\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01``\x81R` \x01``\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[`\0\x805``\x90\x81\x1C\x83R\x90`\x14a\x01\x16`\x02\x825`\xF8\x1Ca\x05?V[`\x01`\x01`\xA0\x1B\x03\x16` \x85\x01R`\x01\x01`\0\x815`\xF8\x1C`\x01\x92\x83\x01\x92\x90\x91P`\x0F`\x04\x83\x90\x1C\x16\x01`\xFF\x16\x825`\x08\x82\x02a\x01\0\x03\x1C`\xA0\x87\x01R\x91\x82\x01\x91`\x0F\x82\x16`\x01\x01`\xFF\x16\x835`\x08\x82\x02a\x01\0\x03\x1C`\x80\x88\x01R\x92\x83\x01\x92a\x01\x88\x845`\xF8\x1C`\x05\x90`\xFF\x16a\x05?V[\x94P\x83`\x01\x01\x93PPPP`\0a\x01\xA0\x825`\xF8\x1C\x90V[`\x01\x92\x83\x01\x92`\xFF\x82\x16\x92P`\x0F`\x04\x83\x90\x1C\x81\x16\x82\x01\x92\x16\x01`\0\x82g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x01\xD5Wa\x01\xD5a\n:V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x02\x1AW\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x01\xF3W\x90P[P\x90P`\0\x82g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x028Wa\x028a\n:V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x02}W\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x02VW\x90P[P\x90P`\0\x80[\x85\x81\x10\x15a\x03 Wa\x02\xA1\x885`\xF0\x1C[`\x08\x90a\xFF\xFF\x16a\x05?V[\x84\x82\x81Q\x81\x10a\x02\xB3Wa\x02\xB3a\nPV[` \x90\x81\x02\x91\x90\x91\x01\x01Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90R`\x02\x97\x90\x97\x01\x96a\x02\xDD\x885`\xF8\x1C\x90V[`\x01\x98\x89\x01\x98\x01`\xFF\x16\x91P\x875`\x08\x83\x02a\x01\0\x03\x1C\x84\x82\x81Q\x81\x10a\x03\x06Wa\x03\x06a\nPV[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q\x01R\x96\x81\x01\x96`\x01\x01a\x02\x84V[P`@\x8A\x01\x83\x90R`\0[\x84\x81\x10\x15a\x03\xBFWa\x03@\x885`\xF0\x1Ca\x02\x95V[\x83\x82\x81Q\x81\x10a\x03RWa\x03Ra\nPV[` \x90\x81\x02\x91\x90\x91\x01\x01Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90R`\x02\x97\x90\x97\x01\x96a\x03|\x885`\xF8\x1C\x90V[`\x01\x98\x89\x01\x98\x01`\xFF\x16\x91P\x875`\x08\x83\x02a\x01\0\x03\x1C\x83\x82\x81Q\x81\x10a\x03\xA5Wa\x03\xA5a\nPV[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q\x01R\x96\x81\x01\x96`\x01\x01a\x03+V[PP``\x89\x81\x01\x91\x90\x91R`@\x80Q`\x01\x88\x015` \x82\x01R`!\x88\x015\x81\x83\x01R\x875`\x01`\x01`\xF8\x1B\x03\x19\x16\x92\x81\x01\x92\x90\x92R\x80Q`A\x81\x84\x03\x81\x01\x82R`a\x90\x93\x01\x90\x91R\x97P\x90\x94\x01\x93Pc\x06M[\xC3`\xE0\x1B\x92P\x86\x91P\x85\x90P\x84a\x04+\x8A\x86\x81\x8Ea\nfV[`@Q`$\x01a\x04?\x95\x94\x93\x92\x91\x90a\x0BNV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16`\x01`\x01`\xE0\x1B\x03\x19\x93\x90\x93\x16\x92\x90\x92\x17\x82RQ\x97P\x95PPPPPP\xF3[a\x04\x85a\x05TV[`@Qa\x04\x92\x91\x90a\x0C\x02V[`@Q\x80\x91\x03\x90\xF3[a\x04\xAEa\x04\xA96`\x04a\x0CvV[a\x05eV[\0[a\x04\x85a\x05\x81V[a\x04\xAEa\x05\x8DV[a\x04\xAEa\x05\xA1V[a\x04\xAEa\x04\xD66`\x04a\x0C\xB1V[a\x06 V[`\0T`\x01`\x01`\xA0\x1B\x03\x16[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x04\x92V[a\x04\x85a\x068V[a\x04\xAEa\x05\x166`\x04a\x0CvV[a\x06DV[`\x01T`\x01`\x01`\xA0\x1B\x03\x16a\x04\xE8V[a\x04\xAEa\x05:6`\x04a\x0C\xD6V[a\x06[V[`\0a\x05K\x83\x83a\x07\x1CV[\x90P[\x92\x91PPV[``a\x05``\x02a\x07\x8CV[\x90P\x90V[a\x05ma\x08pV[a\x05|`\x02`\xFF\x84\x16\x83a\x08\xCAV[PPPV[``a\x05``\x05a\x07\x8CV[a\x05\x95a\x08pV[a\x05\x9F`\0a\x08\xE8V[V[`\x01T3\x90`\x01`\x01`\xA0\x1B\x03\x16\x81\x14a\x06\x14W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`)`$\x82\x01R\x7FOwnable2Step: caller is not the `D\x82\x01Rh72\xBB\x907\xBB\xB72\xB9`\xB9\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[a\x06\x1D\x81a\x08\xE8V[PV[a\x06(a\x08pV[a\x05|`\x08a\xFF\xFF\x84\x16\x83a\x08\xCAV[``a\x05``\x08a\x07\x8CV[a\x06La\x08pV[a\x05|`\x05`\xFF\x84\x16\x83a\x08\xCAV[a\x06ca\x08pV[`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x83\x16`\x01`\x01`\xA0\x1B\x03\x19\x90\x91\x16\x81\x17\x90\x91Ua\x06\x94`\0T`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x7F8\xD1k\x8C\xAC\"\xD9\x9F\xC7\xC1$\xB9\xCD\r\xE2\xD3\xFA\x1F\xAE\xF4 \xBF\xE7\x91\xD8\xC3b\xD7e\xE2'\0`@Q`@Q\x80\x91\x03\x90\xA3PV[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x84U`@Q\x91\x90\x92\x16\x92\x83\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PPV[`\0\x81\x81R`\x02\x83\x01` R`@\x81 T\x80\x15\x15\x80a\x07@WPa\x07@\x84\x84a\t\x01V[a\x05KW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1E`$\x82\x01R\x7FEnumerableMap: nonexistent key\0\0`D\x82\x01R`d\x01a\x06\x0BV[```\0a\x07\x99\x83a\t\rV[\x90P`\0\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x07\xB6Wa\x07\xB6a\n:V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x07\xFBW\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x07\xD4W\x90P[P\x90P`\0[\x82\x81\x10\x15a\x08hWa\x08\x13\x85\x82a\t\x18V[\x83\x83\x81Q\x81\x10a\x08%Wa\x08%a\nPV[` \x02` \x01\x01Q`\0\x01\x84\x84\x81Q\x81\x10a\x08BWa\x08Ba\nPV[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x92\x01\x91\x90\x91RR`\x01\x01a\x08\x01V[P\x93\x92PPPV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x05\x9FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x06\x0BV[`\0a\x08\xE0\x84\x84`\x01`\x01`\xA0\x1B\x03\x85\x16a\t4V[\x94\x93PPPPV[`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x90Ua\x06\x1D\x81a\x06\xCCV[`\0a\x05K\x83\x83a\tQV[`\0a\x05N\x82a\tiV[`\0\x80\x80\x80a\t'\x86\x86a\ttV[\x90\x97\x90\x96P\x94PPPPPV[`\0\x82\x81R`\x02\x84\x01` R`@\x81 \x82\x90Ua\x08\xE0\x84\x84a\t\x9FV[`\0\x81\x81R`\x01\x83\x01` R`@\x81 T\x15\x15a\x05KV[`\0a\x05N\x82a\t\xABV[`\0\x80\x80a\t\x82\x85\x85a\t\xB5V[`\0\x81\x81R`\x02\x96\x90\x96\x01` R`@\x90\x95 T\x94\x95\x93PPPPV[`\0a\x05K\x83\x83a\t\xC1V[`\0a\x05N\x82T\x90V[`\0a\x05K\x83\x83a\n\x10V[`\0\x81\x81R`\x01\x83\x01` R`@\x81 Ta\n\x08WP\x81T`\x01\x81\x81\x01\x84U`\0\x84\x81R` \x80\x82 \x90\x93\x01\x84\x90U\x84T\x84\x82R\x82\x86\x01\x90\x93R`@\x90 \x91\x90\x91Ua\x05NV[P`\0a\x05NV[`\0\x82`\0\x01\x82\x81T\x81\x10a\n'Wa\n'a\nPV[\x90`\0R` `\0 \x01T\x90P\x92\x91PPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0\x80\x85\x85\x11\x15a\nvW`\0\x80\xFD[\x83\x86\x11\x15a\n\x83W`\0\x80\xFD[PP\x82\x01\x93\x91\x90\x92\x03\x91PV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a\n\xD4W\x81Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x88R\x83\x01Q\x83\x88\x01R`@\x90\x96\x01\x95\x90\x82\x01\x90`\x01\x01a\n\xA4V[P\x94\x95\x94PPPPPV[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15a\x0B\x05W` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a\n\xE9V[P`\0` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[\x81\x83R\x81\x81` \x85\x017P`\0\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[`\x80\x81R`\0`\x01\x80`\xA0\x1B\x03\x80\x88Q\x16`\x80\x84\x01R\x80` \x89\x01Q\x16`\xA0\x84\x01RP`@\x87\x01Q`\xC0\x80\x84\x01Ra\x0B\x8Aa\x01@\x84\x01\x82a\n\x90V[\x90P``\x88\x01Q`\x7F\x19\x84\x83\x03\x01`\xE0\x85\x01Ra\x0B\xA7\x82\x82a\n\x90V[\x91PP`\x80\x88\x01Qa\x01\0\x84\x01R`\xA0\x88\x01Qa\x01 \x84\x01R\x82\x81\x03` \x84\x01Ra\x0B\xD2\x81\x88a\n\xDFV[`\x01`\x01`\xA0\x1B\x03\x87\x16`@\x85\x01R\x90P\x82\x81\x03``\x84\x01Ra\x0B\xF6\x81\x85\x87a\x0B%V[\x98\x97PPPPPPPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90`@\x90\x81\x85\x01\x90\x86\x84\x01\x85[\x82\x81\x10\x15a\x0CMW\x81Q\x80Q\x85R\x86\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x86\x85\x01R\x92\x84\x01\x92\x90\x85\x01\x90`\x01\x01a\x0C\x1FV[P\x91\x97\x96PPPPPPPV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x0CqW`\0\x80\xFD[\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x0C\x89W`\0\x80\xFD[\x825`\xFF\x81\x16\x81\x14a\x0C\x9AW`\0\x80\xFD[\x91Pa\x0C\xA8` \x84\x01a\x0CZV[\x90P\x92P\x92\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x0C\xC4W`\0\x80\xFD[\x825a\xFF\xFF\x81\x16\x81\x14a\x0C\x9AW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x0C\xE8W`\0\x80\xFD[a\x05K\x82a\x0CZV\xFE\xA2dipfsX\"\x12 \xE9O<o\xAAm\xCD\xB3\xAA\xB9\x02T\x88\x1C\x9BZ\x18z\x17\x97\xF9p\xBAW}\xB2\xD7T\x18\x15\x84\x10dsolcC\0\x08\x11\x003";
    /// The deployed bytecode of the contract.
    pub static DECODERWITHREGISTRY_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct DecoderWithRegistry<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for DecoderWithRegistry<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for DecoderWithRegistry<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for DecoderWithRegistry<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for DecoderWithRegistry<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(DecoderWithRegistry))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> DecoderWithRegistry<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                DECODERWITHREGISTRY_ABI.clone(),
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
                DECODERWITHREGISTRY_ABI.clone(),
                DECODERWITHREGISTRY_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `acceptOwnership` (0x79ba5097) function
        pub fn accept_ownership(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([121, 186, 80, 151], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `fulfillers` (0x2f6d2003) function
        pub fn fulfillers(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<IdToAddress>> {
            self.0
                .method_hash([47, 109, 32, 3], ())
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
        ///Calls the contract's `pendingOwner` (0xe30c3978) function
        pub fn pending_owner(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([227, 12, 57, 120], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `renounceOwnership` (0x715018a6) function
        pub fn renounce_ownership(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([113, 80, 24, 166], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setFulfiller` (0xd7b41a47) function
        pub fn set_fulfiller(
            &self,
            fulfiller_id: u8,
            fulfiller: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([215, 180, 26, 71], (fulfiller_id, fulfiller))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setToken` (0x8767171c) function
        pub fn set_token(
            &self,
            token_id: u16,
            token: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([135, 103, 23, 28], (token_id, token))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setZone` (0x1b4a718a) function
        pub fn set_zone(
            &self,
            zone_id: u8,
            zone: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([27, 74, 113, 138], (zone_id, zone))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `tokens` (0x9d63848a) function
        pub fn tokens(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<IdToAddress>> {
            self.0
                .method_hash([157, 99, 132, 138], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transferOwnership` (0xf2fde38b) function
        pub fn transfer_ownership(
            &self,
            new_owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 253, 227, 139], new_owner)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `zones` (0x03d25049) function
        pub fn zones(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<IdToAddress>> {
            self.0
                .method_hash([3, 210, 80, 73], ())
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `OwnershipTransferStarted` event
        pub fn ownership_transfer_started_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OwnershipTransferStartedFilter,
        > {
            let mut event = self.0.event();
            event.filter = event.filter.address(self.address());

            event
        }
        ///Gets the contract's `OwnershipTransferred` event
        pub fn ownership_transferred_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, OwnershipTransferredFilter>
        {
            let mut event = self.0.event();
            event.filter = event.filter.address(self.address());

            event
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, DecoderWithRegistryEvents>
        {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
        for DecoderWithRegistry<M>
    {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
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
        name = "OwnershipTransferStarted",
        abi = "OwnershipTransferStarted(address,address)"
    )]
    pub struct OwnershipTransferStartedFilter {
        #[ethevent(indexed)]
        pub previous_owner: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_owner: ::ethers::core::types::Address,
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
        name = "OwnershipTransferred",
        abi = "OwnershipTransferred(address,address)"
    )]
    pub struct OwnershipTransferredFilter {
        #[ethevent(indexed)]
        pub previous_owner: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_owner: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum DecoderWithRegistryEvents {
        OwnershipTransferStartedFilter(OwnershipTransferStartedFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
    }
    impl ::ethers::contract::EthLogDecode for DecoderWithRegistryEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = OwnershipTransferStartedFilter::decode_log(log) {
                return Ok(DecoderWithRegistryEvents::OwnershipTransferStartedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(DecoderWithRegistryEvents::OwnershipTransferredFilter(
                    decoded,
                ));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for DecoderWithRegistryEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::OwnershipTransferStartedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OwnershipTransferredFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<OwnershipTransferStartedFilter> for DecoderWithRegistryEvents {
        fn from(value: OwnershipTransferStartedFilter) -> Self {
            Self::OwnershipTransferStartedFilter(value)
        }
    }
    impl ::core::convert::From<OwnershipTransferredFilter> for DecoderWithRegistryEvents {
        fn from(value: OwnershipTransferredFilter) -> Self {
            Self::OwnershipTransferredFilter(value)
        }
    }
    ///Container type for all input parameters for the `acceptOwnership` function with signature `acceptOwnership()` and selector `0x79ba5097`
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
    #[ethcall(name = "acceptOwnership", abi = "acceptOwnership()")]
    pub struct AcceptOwnershipCall;
    ///Container type for all input parameters for the `fulfillers` function with signature `fulfillers()` and selector `0x2f6d2003`
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
    #[ethcall(name = "fulfillers", abi = "fulfillers()")]
    pub struct FulfillersCall;
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
    ///Container type for all input parameters for the `pendingOwner` function with signature `pendingOwner()` and selector `0xe30c3978`
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
    #[ethcall(name = "pendingOwner", abi = "pendingOwner()")]
    pub struct PendingOwnerCall;
    ///Container type for all input parameters for the `renounceOwnership` function with signature `renounceOwnership()` and selector `0x715018a6`
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
    #[ethcall(name = "renounceOwnership", abi = "renounceOwnership()")]
    pub struct RenounceOwnershipCall;
    ///Container type for all input parameters for the `setFulfiller` function with signature `setFulfiller(uint8,address)` and selector `0xd7b41a47`
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
    #[ethcall(name = "setFulfiller", abi = "setFulfiller(uint8,address)")]
    pub struct SetFulfillerCall {
        pub fulfiller_id: u8,
        pub fulfiller: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setToken` function with signature `setToken(uint16,address)` and selector `0x8767171c`
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
    #[ethcall(name = "setToken", abi = "setToken(uint16,address)")]
    pub struct SetTokenCall {
        pub token_id: u16,
        pub token: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setZone` function with signature `setZone(uint8,address)` and selector `0x1b4a718a`
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
    #[ethcall(name = "setZone", abi = "setZone(uint8,address)")]
    pub struct SetZoneCall {
        pub zone_id: u8,
        pub zone: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `tokens` function with signature `tokens()` and selector `0x9d63848a`
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
    #[ethcall(name = "tokens", abi = "tokens()")]
    pub struct TokensCall;
    ///Container type for all input parameters for the `transferOwnership` function with signature `transferOwnership(address)` and selector `0xf2fde38b`
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
    #[ethcall(name = "transferOwnership", abi = "transferOwnership(address)")]
    pub struct TransferOwnershipCall {
        pub new_owner: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `zones` function with signature `zones()` and selector `0x03d25049`
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
    #[ethcall(name = "zones", abi = "zones()")]
    pub struct ZonesCall;
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum DecoderWithRegistryCalls {
        AcceptOwnership(AcceptOwnershipCall),
        Fulfillers(FulfillersCall),
        Owner(OwnerCall),
        PendingOwner(PendingOwnerCall),
        RenounceOwnership(RenounceOwnershipCall),
        SetFulfiller(SetFulfillerCall),
        SetToken(SetTokenCall),
        SetZone(SetZoneCall),
        Tokens(TokensCall),
        TransferOwnership(TransferOwnershipCall),
        Zones(ZonesCall),
    }
    impl ::ethers::core::abi::AbiDecode for DecoderWithRegistryCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) =
                <AcceptOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::AcceptOwnership(decoded));
            }
            if let Ok(decoded) = <FulfillersCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Fulfillers(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded) = <PendingOwnerCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::PendingOwner(decoded));
            }
            if let Ok(decoded) =
                <RenounceOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RenounceOwnership(decoded));
            }
            if let Ok(decoded) = <SetFulfillerCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetFulfiller(decoded));
            }
            if let Ok(decoded) = <SetTokenCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetToken(decoded));
            }
            if let Ok(decoded) = <SetZoneCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetZone(decoded));
            }
            if let Ok(decoded) = <TokensCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Tokens(decoded));
            }
            if let Ok(decoded) =
                <TransferOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TransferOwnership(decoded));
            }
            if let Ok(decoded) = <ZonesCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Zones(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for DecoderWithRegistryCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AcceptOwnership(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Fulfillers(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PendingOwner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RenounceOwnership(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetFulfiller(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetToken(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetZone(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Tokens(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TransferOwnership(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Zones(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for DecoderWithRegistryCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AcceptOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::Fulfillers(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::PendingOwner(element) => ::core::fmt::Display::fmt(element, f),
                Self::RenounceOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetFulfiller(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetToken(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetZone(element) => ::core::fmt::Display::fmt(element, f),
                Self::Tokens(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::Zones(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AcceptOwnershipCall> for DecoderWithRegistryCalls {
        fn from(value: AcceptOwnershipCall) -> Self {
            Self::AcceptOwnership(value)
        }
    }
    impl ::core::convert::From<FulfillersCall> for DecoderWithRegistryCalls {
        fn from(value: FulfillersCall) -> Self {
            Self::Fulfillers(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for DecoderWithRegistryCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<PendingOwnerCall> for DecoderWithRegistryCalls {
        fn from(value: PendingOwnerCall) -> Self {
            Self::PendingOwner(value)
        }
    }
    impl ::core::convert::From<RenounceOwnershipCall> for DecoderWithRegistryCalls {
        fn from(value: RenounceOwnershipCall) -> Self {
            Self::RenounceOwnership(value)
        }
    }
    impl ::core::convert::From<SetFulfillerCall> for DecoderWithRegistryCalls {
        fn from(value: SetFulfillerCall) -> Self {
            Self::SetFulfiller(value)
        }
    }
    impl ::core::convert::From<SetTokenCall> for DecoderWithRegistryCalls {
        fn from(value: SetTokenCall) -> Self {
            Self::SetToken(value)
        }
    }
    impl ::core::convert::From<SetZoneCall> for DecoderWithRegistryCalls {
        fn from(value: SetZoneCall) -> Self {
            Self::SetZone(value)
        }
    }
    impl ::core::convert::From<TokensCall> for DecoderWithRegistryCalls {
        fn from(value: TokensCall) -> Self {
            Self::Tokens(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipCall> for DecoderWithRegistryCalls {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
        }
    }
    impl ::core::convert::From<ZonesCall> for DecoderWithRegistryCalls {
        fn from(value: ZonesCall) -> Self {
            Self::Zones(value)
        }
    }
    ///Container type for all return fields from the `fulfillers` function with signature `fulfillers()` and selector `0x2f6d2003`
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
    pub struct FulfillersReturn(pub ::std::vec::Vec<IdToAddress>);
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
    ///Container type for all return fields from the `pendingOwner` function with signature `pendingOwner()` and selector `0xe30c3978`
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
    pub struct PendingOwnerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `tokens` function with signature `tokens()` and selector `0x9d63848a`
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
    pub struct TokensReturn(pub ::std::vec::Vec<IdToAddress>);
    ///Container type for all return fields from the `zones` function with signature `zones()` and selector `0x03d25049`
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
    pub struct ZonesReturn(pub ::std::vec::Vec<IdToAddress>);
    ///`IdToAddress(uint256,address)`
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
    pub struct IdToAddress {
        pub id: ::ethers::core::types::U256,
        pub addr: ::ethers::core::types::Address,
    }
}
