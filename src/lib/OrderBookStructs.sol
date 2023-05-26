// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

struct OrderParameters {
    address offerer;
    address fulfiller;
    address zone;
    Item[] offer;
    Item[] consideration;
    uint256 deadline;
    uint256 salt;
    bytes extraData;
}

struct Item {
    bool isNative;
    address token;
    uint256 amount;
}

struct Order {
    OrderParameters parameters;
    bytes signature;
}

struct OrderComponents {
    OrderParameters parameters;
    uint256 counter;
}

struct OrderStatus {
    bool isValidated;
    bool isCancelled;
}
