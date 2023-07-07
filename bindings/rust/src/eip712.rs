pub use eip712::*;
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
pub mod eip712 {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("DOMAIN_SEPARATOR"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("DOMAIN_SEPARATOR"),
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
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static EIP712_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xC0`@R4\x80\x15a\0\x10W`\0\x80\xFD[PF`\xA0\x81\x81R`@\x80Q\x7F\x8C\xAD\x95h{\xA8,,\xE5\x0Et\xF7\xB7Td^Q\x17\xC3\xA5\xBE\xC8\x15\x1C\x07&\xD5\x85y\x80\xA8f` \x80\x83\x01\x91\x90\x91R\x7F\x9A\xC9\x97An\x8F\xF9\xD2\xFFk\xEB\xEBqI\xF6\\\xDA\xE5\xE3.+\x90D\x0BVk\xB3\x04@A\xD3j\x82\x84\x01R``\x82\x01\x94\x90\x94R0`\x80\x80\x83\x01\x91\x90\x91R\x82Q\x80\x83\x03\x90\x91\x01\x81R\x92\x01\x90R\x80Q\x91\x01 `\x80R`\x80Q`\xA0Qa\x01La\0\xB5`\09`\0`N\x01R`\0`\xF4\x01Ra\x01L`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0+W`\x005`\xE0\x1C\x80c6D\xE5\x15\x14a\x000W[`\0\x80\xFD[a\08a\0JV[`@Q\x90\x81R` \x01`@Q\x80\x91\x03\x90\xF3[`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0F\x14a\0\xF1WP`@\x80Q\x7F\x8C\xAD\x95h{\xA8,,\xE5\x0Et\xF7\xB7Td^Q\x17\xC3\xA5\xBE\xC8\x15\x1C\x07&\xD5\x85y\x80\xA8f` \x80\x83\x01\x91\x90\x91R\x7F\x9A\xC9\x97An\x8F\xF9\xD2\xFFk\xEB\xEBqI\xF6\\\xDA\xE5\xE3.+\x90D\x0BVk\xB3\x04@A\xD3j\x82\x84\x01RF``\x83\x01R0`\x80\x80\x84\x01\x91\x90\x91R\x83Q\x80\x84\x03\x90\x91\x01\x81R`\xA0\x90\x92\x01\x90\x92R\x80Q\x91\x01 \x90V[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90V\xFE\xA2dipfsX\"\x12 \xD2\xB5\"W\xC4\xEEn\xC3\xE7\x03\xFD\xD0\xE3h\xE7\xBD\x85\xE6\x19\xD7\xC9v]\x02\xEB\x80L\xF0}\xB7\xC7\xAEdsolcC\0\x08\x11\x003";
    /// The bytecode of the contract.
    pub static EIP712_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0+W`\x005`\xE0\x1C\x80c6D\xE5\x15\x14a\x000W[`\0\x80\xFD[a\08a\0JV[`@Q\x90\x81R` \x01`@Q\x80\x91\x03\x90\xF3[`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0F\x14a\0\xF1WP`@\x80Q\x7F\x8C\xAD\x95h{\xA8,,\xE5\x0Et\xF7\xB7Td^Q\x17\xC3\xA5\xBE\xC8\x15\x1C\x07&\xD5\x85y\x80\xA8f` \x80\x83\x01\x91\x90\x91R\x7F\x9A\xC9\x97An\x8F\xF9\xD2\xFFk\xEB\xEBqI\xF6\\\xDA\xE5\xE3.+\x90D\x0BVk\xB3\x04@A\xD3j\x82\x84\x01RF``\x83\x01R0`\x80\x80\x84\x01\x91\x90\x91R\x83Q\x80\x84\x03\x90\x91\x01\x81R`\xA0\x90\x92\x01\x90\x92R\x80Q\x91\x01 \x90V[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90V\xFE\xA2dipfsX\"\x12 \xD2\xB5\"W\xC4\xEEn\xC3\xE7\x03\xFD\xD0\xE3h\xE7\xBD\x85\xE6\x19\xD7\xC9v]\x02\xEB\x80L\xF0}\xB7\xC7\xAEdsolcC\0\x08\x11\x003";
    /// The deployed bytecode of the contract.
    pub static EIP712_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct EIP712<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for EIP712<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for EIP712<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for EIP712<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for EIP712<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(EIP712)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> EIP712<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    EIP712_ABI.clone(),
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
                EIP712_ABI.clone(),
                EIP712_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `DOMAIN_SEPARATOR` (0x3644e515) function
        pub fn domain_separator(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([54, 68, 229, 21], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for EIP712<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `DOMAIN_SEPARATOR` function with signature `DOMAIN_SEPARATOR()` and selector `0x3644e515`
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
    #[ethcall(name = "DOMAIN_SEPARATOR", abi = "DOMAIN_SEPARATOR()")]
    pub struct DomainSeparatorCall;
    ///Container type for all return fields from the `DOMAIN_SEPARATOR` function with signature `DOMAIN_SEPARATOR()` and selector `0x3644e515`
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
    pub struct DomainSeparatorReturn(pub [u8; 32]);
}
