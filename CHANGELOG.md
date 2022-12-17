# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).


## [0.3.1] - 2022-12-16

### Changed
- Update `@typechain/ethers-v5` to the latest version, so we don't have to do manual patching anymore


## [0.3.0] - 2022-12-12

### Added
- Added a `FloodRegistry` contract which is now responsible for holding all protocol related information like tokens whitelisted and the latest oracle in use.


### Changed
- The oracle now pulls both bonds from the book. This means disputers only need to approve the book and not the oracle anymore.
- The book and oracle don't maintain an internal whitelist of allowed tokens anymore, but rely on the registry instead. 
- Deploy Scripts have been broken up and refactored


### Fixed
- Addressed some minor reentrancy bugs


## [0.2.2] - 2022-11-18

### Fixed

- Further fixes for JS bindings typechain patch


## [0.2.1] - 2022-11-17

### Fixed

- Fixed JS bindings @typechain/ethers-v5 patch to only apply when installing dev dependencies


## [0.2.0] - 2022-10-06

### Changed

- The user requesting the trade, which we call trader in the code, is now part of the tradeId.
The tradeId is now calculated from "tokenIn, tokenOut, amountIn, minAmountOut, recipient, tradeIndex, trader" 
- Add trader as an indexed field in the TradeRequested event, make tokenOut non indexed
- Add trader as an indexed field in the TradeFilled event
- Add trader as an indexed field in the TradeSettled event
- Add trader as an indexed field in the TradeDisputed event, make relayer non indexed 
- Add trader as an indexed field in the TradeCancelled event
- Add trader as an indexed field in the TradeDisputeSettled event, make relayer non indexed
