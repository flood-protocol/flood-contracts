const ethers = require('ethers');

const STRUCTS = {
  Order: [{
    name: 'offerer',
    type: 'address'
    },
    {
    name: 'zone',
    type: 'address'
    },
    {
    name: 'recipient',
    type: 'address'
    },
    {
    name: 'offer',
    type: 'Item[]'
    },
    {
    name: 'consideration',
    type: 'Item'
    },
    {
    name: 'deadline',
    type: 'uint256'
    },
    {
    name: 'nonce',
    type: 'uint256'
    },
    {
    name: 'preHooks',
    type: 'Hook[]'
    },
    {
    name: 'postHooks',
    type: 'Hook[]'
    }],
  Item: [{
    name: 'token',
    type: 'address'
    },
    {
    name: 'amount',
    type: 'uint256'
    }],
  Hook: [{
    name: 'target',
    type: 'address'
    },
    {
    name: 'data',
    type: 'bytes'
    }],
  PermitBatchWitnessTransferFrom: [{
    name: 'permitted',
    type: 'TokenPermissions[]'
    },
    {
    name: 'spender',
    type: 'address'
    },
    {
    name: 'nonce',
    type: 'uint256'
    },
    {
    name: 'deadline',
    type: 'uint256'
    },
    {
    name: 'witness',
    type: 'Order'
  }],
  TokenPermissions: [{
    name: 'token',
    type: 'address'
    },
    {
    name: 'amount',
    type: 'uint256'
  }]
}

const item = {
  token: ethers.ZeroAddress,
  amount: 0
}

const hook = {
  target: ethers.ZeroAddress,
  data: "0x"
}

const order = {
  offerer: ethers.ZeroAddress,
  zone: ethers.ZeroAddress,
  recipient: ethers.ZeroAddress,
  offer: [item, item, item],
  consideration: item,
  deadline: 0,
  nonce: 0,
  preHooks: [hook],
  postHooks: [hook,hook]
}

const permit = {
  permitted: [item, item, item],
  spender: ethers.getAddress('0x0000000000000000000000000000000000000420'),
  nonce: 0,
  deadline: 0,
  witness: order
}

const itemHash = ethers.TypedDataEncoder.hashStruct("Item", STRUCTS, item)
console.log("itemHash: " + itemHash);

const hookHash = ethers.TypedDataEncoder.hashStruct("Hook", STRUCTS, hook)
console.log("hookHash: " + hookHash);

const orderHash = ethers.TypedDataEncoder.hashStruct("Order", STRUCTS, order)
console.log("orderHash: " + orderHash);

const permitHash = ethers.TypedDataEncoder.hashStruct("PermitBatchWitnessTransferFrom", STRUCTS, permit)
console.log("permitHash: " + permitHash);
